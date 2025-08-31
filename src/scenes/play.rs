use crate::logic::constraints::{
    choose_constraints, compute_satisfaction, count_unreachable_rails, Constraints, Satisfaction,
};
use crate::logic::grid::{count_neighbours, get, get_cell, get_cell_mut, get_mut, in_range, Grid};
use crate::render::{render_cells, render_constraints, render_grid, render_satisfaction};
use crate::theme::{new_button, new_text, render_button, render_text, Theme};
use crate::{
    grid_height, grid_width, AnyError, BACKGROUND, BACKGROUND_2, BUTTON_PANEL_WIDTH, CELL_HEIGHT,
    CELL_PAD, CELL_WIDTH, DEFAULT_SHOW_SOLUTION, FONT_SIZE_CHANGING, GRID_PAD, MAX_CELLS,
    NUM_COLUMNS, NUM_ROWS, PANEL_BACKGROUND, SHOW_FPS, STEP_GENERATION, STYLE, VISUALIZE,
};
use juquad::draw::draw_rect;
use juquad::widgets::anchor::{Anchor, Vertical};
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
    clear_background, draw_texture_ex, get_fps, next_frame, screen_height, screen_width,
    DrawTextureParams,
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

pub async fn play(theme: &mut Theme) -> Result<(), AnyError> {
    let mut state = reset(VISUALIZE).await;
    let (sw, sh) = (screen_width(), screen_height());
    let texture_params = DrawTextureParams {
        flip_y: true,
        ..Default::default()
    };
    let render_target = macroquad::prelude::render_target(sw as u32, sh as u32);
    render_target.texture.set_filter(FilterMode::Nearest);

    let mut refresh_render = true;
    let mut show_solution_button = None;

    let button_panel = Rect::new(
        grid_width() + GRID_PAD * 2.0,
        GRID_PAD,
        BUTTON_PANEL_WIDTH,
        grid_height(),
    );
    let mut right_clicked = None;
    let mut _should_play_sound = false;
    let mut should_play_intro = true;
    let mut should_play_background = true;
    // play_sound_once(music_background_intro);
    // play_sound_once(music_background);
    // play_sound(music_background, PlaySoundParams { looped: true, volume: 0.5 });
    let mut start_ts = None;
    loop {
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
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        let mut reset_button = new_button(
            "New Game",
            Anchor::top_center(
                button_panel.x + button_panel.w * 0.5,
                button_panel.y + GRID_PAD,
            ),
            &theme,
        );
        if is_key_pressed(KeyCode::R) || reset_button.interact().is_clicked() {
            state = reset(VISUALIZE).await;
            refresh_render = true;
        }
        let pos = Vec2::from(mouse_position());
        let grid_indexes =
            (pos - GRID_PAD + CELL_PAD * 0.5) / (vec2(CELL_WIDTH, CELL_HEIGHT) + CELL_PAD);
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
                zoom: vec2(1.0 / sw * 2.0, -1.0 / sh * 2.0),
                render_target: Some(render_target.clone()),
                ..Default::default()
            });

            clear_background(Color::new(0.0, 0.0, 0.0, 0.0));

            draw_rect(button_panel, PANEL_BACKGROUND);
            let satisfaction = compute_satisfaction(&state.grid, &state.constraints);
            show_solution_button = render_satisfaction(
                &satisfaction,
                reset_button.rect(),
                button_panel,
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
                render_grid(&state.solution);
                render_constraints(&state.constraints, &state.solution);
            } else {
                render_grid(&state.grid);
                render_constraints(&state.constraints, &state.grid);
            }
            set_default_camera();
        }
        clear_background(BACKGROUND);
        if state.show_solution {
            render_cells(&state.solution, &hovered_cell);
        } else {
            render_cells(&state.grid, &hovered_cell);
        }
        draw_texture_ex(render_target.texture, 0., 0., WHITE, texture_params.clone());
        render_button(&reset_button);

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
        if FONT_SIZE_CHANGING {
            let mut decrease = new_button(
                "decrease",
                Anchor::bottom_right(button_panel.right(), button_panel.bottom()),
                &theme,
            );
            let mut increase = new_button(
                "increase",
                Anchor::leftwards(decrease.rect(), Vertical::Center, CELL_PAD),
                &theme,
            );
            let font_size_text = new_text(
                &format!("font size: {}", theme.layout.font_size()),
                Anchor::leftwards(increase.rect(), Vertical::Center, CELL_PAD),
                1.0,
                &theme,
            );
            if increase.interact().is_clicked() {
                *theme.layout.font_size_mut() += 1.0;
                refresh_render = true;
            }
            if decrease.interact().is_clicked() {
                *theme.layout.font_size_mut() -= 1.0;
                refresh_render = true;
            }
            render_text(&font_size_text, &STYLE.at_rest);
            render_button(&increase);
            render_button(&decrease);
        }
        next_frame().await
    }
    Ok(())
}

async fn reset(visualize: bool) -> State {
    let mut solution = generate_grid(visualize).await;
    solution.recalculate_rails();
    while count_unreachable_rails(&solution) > 0 {
        solution = generate_grid(visualize).await;
        solution.recalculate_rails();
    }
    // println!("tried {} iterations", i);
    let mut grid = Grid::new(NUM_ROWS, NUM_COLUMNS, solution.root);
    grid.recalculate_rails();
    let constraints = choose_constraints(&solution);
    let show_solution = DEFAULT_SHOW_SOLUTION;
    let previous_satisfaction = None;
    let success_sound_played = false;
    State {
        solution,
        grid,
        constraints,
        show_solution,
        previous_satisfaction,
        success_sound_played,
    }
}

async fn generate_grid(visualize: bool) -> Grid {
    let mut solution = Grid::new(NUM_ROWS, NUM_COLUMNS, ivec2(NUM_COLUMNS / 2, NUM_ROWS / 2));
    let mut enabled = Vec::new();

    enabled.push((solution.root.y, solution.root.x));
    let mut i = 0;
    while enabled.len() < MAX_CELLS {
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
            render_grid(&mut solution);
            next_frame().await;
        }
    }
    solution
}
