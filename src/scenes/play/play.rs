use crate::level_history::{generate_procedural, GameTrack};
use crate::levels::Level;
use crate::logic::constraints::{compute_satisfaction, Constraints, Satisfaction};
use crate::logic::grid::{
    count_neighbours, get, get_cell, get_cell_mut, get_mut, in_expanded_range_inner, in_range,
    is_system_fixed_v, Grid,
};
use crate::render::{render_cells, render_constraints, render_grid};
use crate::scenes::play::panel::Panel;
use crate::theme::{new_text, render_button, render_text, render_tooltip, Theme};
use crate::{
    new_layout, AnyError, NextStage, BACKGROUND, BACKGROUND_2, CACHE_TEXTURE,
    DEFAULT_SHOW_SOLUTION, MAX_CELLS_COEF, SHOW_FPS, SHOW_SLIDER, STEP_GENERATION, STYLE,
    TEXT_STYLE, TOOLTIP_DELAY, VISUALIZE,
};
use juquad::lazy::{set_positions, Interactable, Renderable, WidgetTrait};
use juquad::widgets::anchor::{Anchor, Horizontal};
use macroquad::camera::{set_camera, set_default_camera, Camera2D};
use macroquad::color::{Color, WHITE};
use macroquad::input::{
    is_key_pressed, is_mouse_button_pressed, is_mouse_button_released, mouse_position, KeyCode,
    MouseButton,
};
use macroquad::math::{ivec2, vec2, IVec2, Vec2};
use macroquad::miniquad::date::now;
use macroquad::miniquad::FilterMode;
use macroquad::prelude::{
    clear_background, draw_texture, get_fps, next_frame, screen_height, screen_width, RenderTarget,
};
use macroquad::rand::rand;

pub struct State {
    solution: Grid,
    grid: Grid,
    constraints: Constraints,
    show_solution: bool,
    previous_satisfaction: Option<Satisfaction>,
    success_sound_played: bool,
    ui: UiState,
}
pub type ShowingSinceSeconds = f64;
pub struct UiState {
    tooltip_showing: Option<(Tooltips, ShowingSinceSeconds)>,
}
pub enum Tooltips {
    FixedCell,
    UserFixedCell,
    EditSolution,
}

pub async fn play(theme: &mut Theme, game_track: &mut GameTrack) -> Result<NextStage, AnyError> {
    let (mut sw, mut sh) = (screen_width(), screen_height());
    let current_level = game_track.get_current(&theme.resources.levels);
    let (mut state, mut panel) = reset(current_level, theme, game_track).await;

    let mut render_target_scale = 1.0;
    let mut slider_value = render_target_scale;
    let mut render_target = reset_render_target(sw, sh, render_target_scale);
    let mut refresh_render = true;
    let mut resize = false;

    let mut right_click_pressed = None;
    loop {
        render_target_scale = slider_value;
        let now = now();
        let (new_sw, new_sh) = (screen_width(), screen_height());
        if new_sw != sw || new_sh != sh {
            resize = true;
        }
        if resize {
            resize = false;
            refresh_render = true;
            sw = new_sw;
            sh = new_sh;
            theme.layout = new_layout(sw, sh).resize_grid(state.grid.rows(), state.grid.columns());
            panel = Panel::new(theme.button_panel_rect(&state.grid), theme, game_track);
            render_target = reset_render_target(sw, sh, render_target_scale);
        }
        if is_key_pressed(KeyCode::P) {
            let level = Level {
                initial_grid: state.grid.clone(),
                constraints: state.constraints.clone(),
                solution: state.solution.clone(),
            };
            println!("{}", level);
        }

        let pos = Vec2::from(mouse_position());
        let hovered_cell = pixel_to_coord(pos, &state.grid, &theme);

        // draw_text(&format!("pos clicked: {:?}", grid_indexes), 0.0, 16.0, 16.0, BLACK);
        if is_mouse_button_pressed(MouseButton::Right) {
            state.ui.tooltip_showing = None;
            if !state.show_solution {
                right_click_pressed = hovered_cell.clone();
            }
        }

        if is_mouse_button_released(MouseButton::Right) {
            if let (
                Some((released_row, released_column)),
                Some((right_clicked_row, right_clicked_column)),
            ) = (hovered_cell.clone(), right_click_pressed)
            {
                if state.show_solution {
                    state.ui.tooltip_showing = Some((Tooltips::EditSolution, now));
                } else {
                    let released = ivec2(released_column, released_row);
                    let pressed = ivec2(right_clicked_column, right_clicked_row);
                    if is_system_fixed_v(released, &state.grid) && pressed == released {
                        state.ui.tooltip_showing = Some((Tooltips::FixedCell, now));
                    } else {
                        add_user_constraint(pressed, released, &mut state, &mut refresh_render);
                    }
                }
            }
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            state.ui.tooltip_showing = None;
            if let Some((i_row, i_column)) = hovered_cell.clone() {
                if state.show_solution {
                    state.ui.tooltip_showing = Some((Tooltips::EditSolution, now));
                } else {
                    let clicked = ivec2(i_column, i_row);
                    if is_system_fixed_v(clicked, &state.grid) {
                        state.ui.tooltip_showing = Some((Tooltips::FixedCell, now));
                    } else {
                        let fixed = get(&state.grid.fixed_cells, i_row, i_column);
                        if *fixed {
                            state.ui.tooltip_showing = Some((Tooltips::UserFixedCell, now));
                        } else {
                            let cell = get_cell_mut(&mut state.grid, i_row, i_column);
                            *cell = !*cell;
                            state.grid.recalculate_rails();
                            refresh_render = true;
                        }
                    }
                }
            }
        }

        clear_background(BACKGROUND);
        if state.show_solution {
            render_cells(&state.solution, &hovered_cell, theme);
        } else {
            render_cells(&state.grid, &hovered_cell, theme);
        }
        if refresh_render || !CACHE_TEXTURE {
            if let Some(render_target) = render_target {
                refresh_render = false;
                set_camera(&Camera2D {
                    //     target: vec2(sw * 0.5, sh * 0.5),
                    target: vec2(sw, sh) * Vec2::splat(0.5 * render_target_scale),
                    zoom: vec2(
                        1.0 / (sw) / render_target_scale as f32 * 2.0,
                        1.0 / (sh) / render_target_scale as f32 * 2.0,
                    ),
                    render_target: Some(render_target),
                    ..Default::default()
                });
                clear_background(Color::new(0.0, 0.0, 0.0, 0.0));
            }
            let satisfaction = compute_satisfaction(&state.grid, &state.constraints);
            if satisfaction.success() {
                game_track.solved()
            }
            if satisfaction.success() {
                if !state.success_sound_played {
                    state.success_sound_played = true;
                    theme.resources.sounds.play_correct();
                }
            }
            panel.add_satisfaction(&satisfaction, &theme, &mut state.show_solution);
            panel.render_static(theme);
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
        }

        if let Some(render_target) = render_target {
            set_default_camera();
            draw_texture(render_target.texture, 0., 0., WHITE);
        }

        if panel.main_menu.interaction().is_clicked() || is_key_pressed(KeyCode::Escape) {
            return Ok(NextStage::MainMenu);
        }
        panel.interact(theme);
        if is_key_pressed(KeyCode::N) || panel.next_game.interaction().is_clicked() {
            let level = game_track.next(theme).await.get_current(&theme.resources.levels);
            (state, panel) = reset(level, theme, game_track).await;
            refresh_render = true;
        }
        if let Some(show) = panel.show_solution.as_mut() {
            if show.interaction().is_clicked() {
                state.show_solution = !state.show_solution;
                refresh_render = true;
            }
        }
        if panel.restart_game.interaction().is_clicked() {
            refresh_render = true;
            (state, panel) = reset(game_track.get_current(&theme.resources.levels), theme, game_track).await;
        }
        if SHOW_FPS {
            let text = format!("FPS: {}", get_fps());
            let text = new_text(&text, Anchor::top_left(0.0, 0.0), 1.0, &theme);
            render_text(&text, &TEXT_STYLE);
        }
        panel.render_interactive();

        if SHOW_SLIDER {
            use_debug_slider(
                theme,
                &panel,
                &mut slider_value,
                &mut refresh_render,
                &mut resize,
            );
        }
        if let Some((tooltip, since)) = &state.ui.tooltip_showing {
            if since + TOOLTIP_DELAY < now {
                state.ui.tooltip_showing = None;
            } else {
                tooltip.render(pos, &theme);
            }
        }
        next_frame().await
    }
}

fn use_debug_slider(
    theme: &mut Theme,
    panel: &Panel,
    mut slider_value: &mut f32,
    refresh_render: &mut bool,
    resize: &mut bool,
) {
    let mut slider = juquad::lazy::slider::Slider::new(
        juquad::lazy::Style {
            coloring: STYLE.clone(),
            ..Default::default()
        },
        0.1,
        5.0,
        *slider_value,
    );
    set_positions(
        &mut slider,
        Anchor::above(panel.main_menu.rect(), Horizontal::Center, theme.cell_pad()),
    );
    slider.interact();
    if *slider_value != slider.custom.current {
        *refresh_render = true;
        *resize = true;
    }
    *slider_value = slider.custom.current;
    slider.render();
    render_text(
        &new_text(
            &format!("slider value: {}", slider_value),
            Anchor::above(slider.rect(), Horizontal::Center, theme.cell_pad()),
            1.0,
            theme,
        ),
        &TEXT_STYLE,
    );
}

fn add_user_constraint(
    pressed: IVec2,
    released: IVec2,
    state: &mut State,
    refresh_render: &mut bool,
) {
    let diff_vec = (pressed - released).abs();
    let diff = diff_vec.x + diff_vec.y;
    if diff == 1 {
        let rail = pressed.max(released);
        let fixed = if diff_vec.y == 1 {
            state.grid.fixed_rails.get_horiz_mut(rail.y, rail.x)
        } else {
            state.grid.fixed_rails.get_vert_mut(rail.y, rail.x)
        };
        *fixed = !*fixed;
        *refresh_render = true;
    } else if diff == 0 {
        let cell = get_mut(&mut state.grid.fixed_cells, released.y, released.x);
        *cell = !*cell;
        *refresh_render = true;
    }
    // TODO: diff = 2, diagonal constraints
}

impl Tooltips {
    pub fn render(&self, pos: Vec2, theme: &Theme) {
        let anchor = Anchor::bottom_left_v(Vec2::from(pos));
        let text = match self {
            Tooltips::FixedCell => "Can't change locked cells",
            Tooltips::UserFixedCell => "Can't change locked cells, use right click to unlock",
            Tooltips::EditSolution => "Can't change cells from solution, click 'HIDE SOLUTION'",
        };
        let text_rect = new_text(text, anchor, 1.0, &theme);
        render_tooltip(&text_rect, &TEXT_STYLE);
    }
}

async fn reset(level: Level, theme: &mut Theme, game_track: &GameTrack) -> (State, Panel) {
    let Level {
        initial_grid: grid,
        constraints,
        solution,
    } = level;
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
        ui: UiState {
            tooltip_showing: None,
        },
    };
    let panel = Panel::new(theme.button_panel_rect(&state.grid), theme, game_track);
    (state, panel)
}
fn reset_render_target(sw: f32, sh: f32, render_target_scale: f32) -> Option<RenderTarget> {
    if CACHE_TEXTURE {
        let render_target = macroquad::prelude::render_target(
            (sw * render_target_scale) as u32,
            (sh * render_target_scale) as u32,
        );
        render_target.texture.set_filter(FilterMode::Nearest);
        Some(render_target)
    } else {
        None
    }
}

pub async fn generate_grid(visualize: bool, theme: &Theme) -> Grid {
    let rows = theme.preferred_rows();
    let columns = theme.preferred_columns();
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
            render_grid(&solution, theme);
            next_frame().await;
        }
    }
    solution
}

fn pixel_to_coord(pixel_pos: Vec2, grid: &Grid, theme: &Theme) -> Option<(i32, i32)> {
    pixel_to_coord_inner(
        pixel_pos,
        grid.rows(),
        grid.columns(),
        theme.grid_pad(),
        theme.cell_pad(),
        theme.cell_width(),
        theme.cell_height(),
    )
}
fn pixel_to_coord_inner(
    pixel_pos: Vec2,
    rows: i32,
    columns: i32,
    grid_pad: f32,
    cell_pad: f32,
    cell_width: f32,
    cell_height: f32,
) -> Option<(i32, i32)> {
    let grid_indexes =
        (pixel_pos - grid_pad + cell_pad * 0.5) / (vec2(cell_width, cell_height) + cell_pad);
    let i_row = grid_indexes.y.floor() as i32;
    let i_column = grid_indexes.x.floor() as i32;
    if in_expanded_range_inner(i_row, i_column, rows, columns) {
        Some((i_row, i_column))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_pixel_to_row_column_out_top_left() {
        let pixel = vec2(1.0, 5.0);
        let cell_coords = pixel_to_coord_inner(pixel, 5, 5, 10.0, 2.0, 8.0, 8.0);
        assert_eq!(cell_coords, None);
    }
    #[test]
    fn test_pixel_to_row_column() {
        let pixel = vec2(25.0, 12.0);
        let cell_coords = pixel_to_coord_inner(pixel, 5, 5, 10.0, 2.0, 8.0, 8.0);
        assert_eq!(cell_coords, Some((0, 1)));
    }
    #[test]
    fn test_pixel_to_row_column_in_cell_border() {
        let pixel = vec2(28.0, 9.0);
        let cell_coords = pixel_to_coord_inner(pixel, 5, 5, 10.0, 2.0, 8.0, 8.0);
        assert_eq!(cell_coords, Some((0, 1)));
    }
}
