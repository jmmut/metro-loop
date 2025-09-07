use crate::level_history::generate_procedural;
use crate::levels::Level;
use crate::logic::constraints::{compute_satisfaction, Constraints, Satisfaction};
use crate::logic::grid::{count_neighbours, get, get_cell, get_cell_mut, get_mut, in_range, Grid};
use crate::render::{render_cells, render_constraints, render_grid, render_satisfaction};
use crate::theme::{new_button, new_text, render_button, render_text, Theme};
use crate::{
    new_layout, AnyError, NextStage, BACKGROUND, BACKGROUND_2, DEFAULT_SHOW_SOLUTION,
    MAX_CELLS_COEF, PANEL_BACKGROUND, SHOW_FPS, STEP_GENERATION, STYLE, VISUALIZE,
};
use juquad::draw::draw_rect;
use juquad::widgets::anchor::Anchor;
use juquad::widgets::button::Button;
use juquad::widgets::text::TextRect;
use macroquad::audio::{play_sound, play_sound_once, PlaySoundParams};
use macroquad::camera::{set_camera, set_default_camera, Camera2D};
use macroquad::color::{Color, WHITE};
use macroquad::input::{
    is_key_pressed, is_mouse_button_pressed, is_mouse_button_released, mouse_position, KeyCode,
    MouseButton,
};
use macroquad::math::{ivec2, vec2, Rect, Vec2};
use macroquad::miniquad::date::now;
use macroquad::miniquad::FilterMode;
use macroquad::prelude::{
    clear_background, draw_texture, get_fps, next_frame, screen_height, screen_width,
};
use macroquad::rand::rand;

pub struct State {
    solution: Grid,
    grid: Grid,
    constraints: Constraints,
    show_solution: bool,
    previous_satisfaction: Option<Satisfaction>,
    success_sound_played: bool,
}

pub struct Panel {
    rect: Rect,
    level_title: TextRect,
    next_game: Button,
    // show_solution: Option<Button>,
}

pub async fn play(theme: &mut Theme) -> Result<NextStage, AnyError> {
    let (mut sw, mut sh) = (screen_width(), screen_height());
    let current_level = theme.resources.level_history.get_current();
    let (mut state, mut panel) = reset(VISUALIZE, current_level, theme).await;
    let mut render_target = macroquad::prelude::render_target(sw as u32, sh as u32);
    render_target.texture.set_filter(FilterMode::Nearest);

    let mut refresh_render = true;
    let mut show_solution_button = None;

    let mut right_clicked = None;
    let mut _should_play_sound = false;
    let mut should_play_intro = true;
    let mut should_play_background = true;
    // play_sound_once(music_background_intro);
    // play_sound_once(music_background);
    // play_sound(music_background, PlaySoundParams { looped: true, volume: 0.5 });
    let mut start_ts = None;
    loop {
        let (new_sw, new_sh) = (screen_width(), screen_height());
        if new_sw != sw || new_sh != sh {
            refresh_render = true;
            sw = new_sw;
            sh = new_sh;
            theme.layout = new_layout(sw, sh).resize_grid(state.grid.rows(), state.grid.columns());
            panel = Panel::new(theme.button_panel_rect(&state.grid), theme);
            render_target = macroquad::prelude::render_target(sw as u32, sh as u32);
        }
        if should_play_intro {
            play_sound_once(theme.resources.sounds.music_background_intro);
            start_ts = Some(now());
            should_play_intro = false;
        }
        if let Some(start_ts) = &start_ts {
            if now() - start_ts > 6.0 {
                if should_play_background {
                    should_play_background = false;
                    play_sound(
                        theme.resources.sounds.music_background,
                        PlaySoundParams {
                            looped: true,
                            volume: 0.75,
                        },
                    );
                }
            }
        }
        if is_key_pressed(KeyCode::P) {
            println!(
                "{}",
                Level {
                    initial_grid: state.grid.clone(),
                    constraints: state.constraints.clone(),
                    solution: state.solution.clone(),
                }
            );
        }

        let pos = Vec2::from(mouse_position());
        let grid_indexes = (pos - theme.grid_pad() + theme.cell_pad() * 0.5)
            / (vec2(theme.cell_width(), theme.cell_height()) + theme.cell_pad());
        let i_row = grid_indexes.y as i32;
        let i_column = grid_indexes.x as i32;
        let hovered_cell = if in_range(&state.grid, i_row, i_column) {
            Some((i_row, i_column))
        } else {
            None
        };
        // draw_text(&format!("pos clicked: {:?}", grid_indexes), 0.0, 16.0, 16.0, BLACK);
        if is_mouse_button_pressed(MouseButton::Right) && !state.show_solution {
            right_clicked = hovered_cell.clone();
        }

        if is_mouse_button_released(MouseButton::Right) && !state.show_solution {
            if let Some((hovered_row, hovered_column)) = hovered_cell.clone() {
                let clicked = ivec2(hovered_column, hovered_row);
                if clicked != state.grid.root && clicked != state.grid.root - ivec2(0, 1) {
                    if let Some((right_clicked_row, right_clicked_column)) = right_clicked {
                        let diff_row = right_clicked_row - clicked.y;
                        let diff_column = right_clicked_column - clicked.x;
                        let diff = diff_row.abs() + diff_column.abs();
                        if diff == 1 {
                            let rail_row = right_clicked_row.max(clicked.y);
                            let rail_column = right_clicked_column.max(clicked.x);
                            let fixed = if diff_row.abs() == 1 {
                                state.grid.fixed_rails.get_horiz_mut(rail_row, rail_column)
                            } else {
                                state.grid.fixed_rails.get_vert_mut(rail_row, rail_column)
                            };
                            *fixed = !*fixed;
                            refresh_render = true;
                        } else if diff == 0 {
                            let cell = get_mut(&mut state.grid.fixed_cells, i_row, i_column);
                            *cell = !*cell;
                            refresh_render = true;
                        } // TODO: diff = 2, diagonal constraints
                    }
                }
            }
        }
        if is_mouse_button_pressed(MouseButton::Left) && !state.show_solution {
            if let Some((i_row, i_column)) = hovered_cell.clone() {
                let clicked = ivec2(i_column, i_row);
                if clicked != state.grid.root && clicked != state.grid.root - ivec2(0, 1) {
                    let fixed = get(&mut state.grid.fixed_cells, i_row, i_column);
                    if !fixed {
                        let cell = get_cell_mut(&mut state.grid, i_row, i_column);
                        *cell = !*cell;
                        state.grid.recalculate_rails();
                        _should_play_sound = true;
                        refresh_render = true;
                    }
                }
            }
        }

        if refresh_render {
            refresh_render = false;
            set_camera(&Camera2D {
                target: vec2(sw * 0.5, sh * 0.5),
                zoom: vec2(1.0 / sw * 2.0, 1.0 / sh * 2.0),
                render_target: Some(render_target.clone()),
                ..Default::default()
            });

            clear_background(Color::new(0.0, 0.0, 0.0, 0.0));
            panel.render_static();
            let satisfaction = compute_satisfaction(&state.grid, &state.constraints);
            if satisfaction.success() {
                theme.resources.level_history.solved()
            }
            show_solution_button = render_satisfaction(
                &satisfaction,
                panel.filled_rect(),
                panel.rect,
                &theme,
                &mut state.show_solution,
            );
            if satisfaction.success() {
                if !state.success_sound_played {
                    state.success_sound_played = true;
                    // if let Some(sound_correct) = sound_correct {
                    //     play_sound(sound_correct, PlaySoundParams::default());
                    // }
                }
            }
            // if let Some(previous) = &previous_satisfaction {
            //     if should_play_sound {
            //         should_play_sound = false;
            //         if satisfaction.failing_rails < previous.failing_rails {
            //             play_sound(sound_correct, PlaySoundParams::default());
            //         } else if satisfaction.failing_rails > previous.failing_rails {
            //             play_sound(sound_incorrect, PlaySoundParams::default());
            //         }
            //     }
            // }
            state.previous_satisfaction = Some(satisfaction);

            if state.show_solution {
                render_grid(&state.solution, theme);
                render_constraints(&state.constraints, &state.solution, theme);
            } else {
                render_grid(&state.grid, theme);
                render_constraints(&state.constraints, &state.grid, theme);
            }
            set_default_camera();
        }
        clear_background(BACKGROUND);
        if state.show_solution {
            render_cells(&state.solution, &hovered_cell, theme);
        } else {
            render_cells(&state.grid, &hovered_cell, theme);
        }
        draw_texture(render_target.texture, 0., 0., WHITE);

        panel.render_interactive();

        if is_key_pressed(KeyCode::N) || panel.next_game.interact().is_clicked() {
            let level = theme.resources.level_history.next().get_current();
            (state, panel) = reset(VISUALIZE, level, theme).await;
            refresh_render = true;
        }
        if let Some(show) = show_solution_button.as_mut() {
            if show.interact().is_clicked() {
                state.show_solution = !state.show_solution;
                refresh_render = true;
            }
            render_button(&show);
        }
        if SHOW_FPS {
            render_text(
                &new_text(
                    &format!("FPS: {}", get_fps()),
                    Anchor::top_left(0.0, 0.0),
                    1.0,
                    &theme,
                ),
                &STYLE.pressed,
            );
        }
        if is_key_pressed(KeyCode::Escape) {
            return Ok(NextStage::MainMenu);
        }
        next_frame().await
    }
}

async fn reset(visualize: bool, level: Option<Level>, theme: &mut Theme) -> (State, Panel) {
    let (grid, constraints, solution) = if let Some(Level {
        initial_grid,
        constraints,
        solution,
    }) = level
    {
        (initial_grid, constraints, solution)
    } else {
        let Level {
            initial_grid,
            constraints,
            solution,
        } = generate_procedural(visualize, theme).await;

        (initial_grid, constraints, solution)
    };
    theme.layout.resize_grid_mut(grid.rows(), grid.columns());
    let show_solution = DEFAULT_SHOW_SOLUTION;
    let previous_satisfaction = None;
    let success_sound_played = false;
    let state = State {
        solution,
        grid,
        constraints,
        show_solution,
        previous_satisfaction,
        success_sound_played,
    };
    let panel = Panel::new(theme.button_panel_rect(&state.grid), theme);
    (state, panel)
}

pub async fn generate_grid(visualize: bool, theme: &Theme) -> Grid {
    let rows = theme.default_rows();
    let columns = theme.default_columns();
    let mut solution = Grid::new(rows, columns, ivec2(columns / 2, rows / 2));
    let mut enabled = Vec::new();

    enabled.push((solution.root.y, solution.root.x));
    let mut i = 0;
    let max_cells = ((rows - 2) as f32 * (columns - 2) as f32 * MAX_CELLS_COEF) as usize;
    while enabled.len() < max_cells {
        if visualize && is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Space) || !STEP_GENERATION {
            i += 1;
            if i > 1000 {
                break;
            }
            // TODO: simplify and allow diagonal cells

            // let index = rand() % enabled.len();
            let mut index = enabled.len() - 1;
            let mut low_neighbours_attempts = 0;
            let (new_row, new_column) = loop {
                let (row, column) = enabled[index];
                let candidate = rand() % 8;
                // println!(
                //     "index: {} ({}, {}), neighbour {}",
                //     index, row, column, neighbour
                // );
                let (new_row, new_column) = match candidate {
                    0 => (row - 1, column),
                    1 => (row, column + 1),
                    2 => (row + 1, column),
                    3 => (row, column - 1),
                    4 => (row - 1, column - 1),
                    5 => (row - 1, column + 1),
                    6 => (row + 1, column + 1),
                    7 => (row + 1, column - 1),
                    _ => panic!(),
                };
                low_neighbours_attempts += 1;
                if in_range(&solution, new_row, new_column) {
                    let neighbours = count_neighbours(&solution, new_row, new_column);
                    let already_enabled = get_cell(&solution, new_row, new_column);
                    if !already_enabled && neighbours <= 2 || low_neighbours_attempts > 50 {
                        break (new_row, new_column);
                    }
                }
                // println!("rejected: ({}, {}), neighbours {}", row, column, neighbours);
                if low_neighbours_attempts % 10 == 0 {
                    index = rand() as usize % enabled.len();
                    // println!("trying another endpoint {}, {}", enabled[index].0, enabled[index].1);
                }
                if low_neighbours_attempts > 100 {
                    panic!();
                }
            };
            let above_root = (solution.root.y - 1, solution.root.x);
            // if the chosen neighbour is already enabled, choose another neighbour
            if in_range(&solution, new_row, new_column) && (new_row, new_column) != above_root {
                *get_cell_mut(&mut solution, new_row, new_column) = true;
                enabled.push((new_row, new_column));
            }
            // for i_row in 0..SIZE {
            //     for i_column in 0..SIZE {
            //         let clicked = *get_mut(&mut grid, i_row, i_column);
            //         let char = if clicked { "0" } else { "-" };
            //         print!("{}", char)
            //     }
            //     println!()
            // }
            // println!()
        }
        if visualize {
            clear_background(BACKGROUND_2);
            render_grid(&mut solution, theme);
            next_frame().await;
        }
    }
    solution
}

impl Panel {
    pub fn new(panel_rect: Rect, theme: &Theme) -> Self {
        let anchor_point = vec2(
            panel_rect.x + panel_rect.w * 0.5,
            panel_rect.y + theme.grid_pad(),
        );
        let half_pad = vec2(theme.cell_pad() * 0.5, 0.0);

        let level_name = theme.resources.level_history.current.to_string();
        let anchor_left = Anchor::top_right_v(anchor_point - half_pad);
        let level_title = new_text(&level_name, anchor_left, 1.0, theme);

        let anchor_right = Anchor::top_left_v(anchor_point + half_pad);
        let next_game = new_button("Next Game", anchor_right, &theme);

        Self {
            rect: panel_rect,
            level_title,
            next_game,
        }
    }
    pub fn filled_rect(&self) -> Rect {
        let mut rect = self.rect;
        rect.h = self.next_game.rect().bottom() - rect.y;
        rect
    }
    pub fn render_static(&self) {
        draw_rect(self.rect, PANEL_BACKGROUND);
        render_text(&self.level_title, &STYLE.at_rest);
    }
    pub fn render_interactive(&self) {
        render_button(&self.next_game);
    }
}
