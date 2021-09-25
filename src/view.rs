use crate::board::{Board, CellState, GameState};
use std::time::Instant;
use macroquad::prelude::*;

pub fn render(board: &Board, atlas: &Texture2D) {
    let (board_width, board_height) = board.grid.size();

    let square = if screen_width() / board_width as f32 > screen_height() / board_height as f32 {
        screen_height() / board_height as f32
    } else {
        screen_width() / board_width as f32
    };

    clear_background(color_u8!(20, 20, 20, 255));

    for x in 0..board_width {
        for y in 0..board_height {
            let cell = board.grid[y][x];

            if cell.revealed {
                if (x + y % 2) % 2 != 0 {
                    draw_rectangle(
                        square * x as f32,
                        square * y as f32,
                        square,
                        square,
                        color_u8!(30, 30, 30, 255)
                    );
                }
            } else {
                draw_texture_ex(
                    *atlas,
                    square * x as f32,
                    square * y as f32,
                    board.color,
                    DrawTextureParams {
                        dest_size: Some(vec2(square, square)),
                        source: Some(Rect::new(8. * 10., 0., 8., 8.)),
                        ..Default::default()
                    },
                );
            }

            if cell.flagged {
                draw_texture_ex(
                    *atlas,
                    square * x as f32,
                    square * y as f32,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(square, square)),
                        source: Some(Rect::new(8. * 9., 0., 8., 8.)),
                        ..Default::default()
                    },
                );
            }

            draw_texture_ex(
                *atlas,
                square * x as f32,
                square * y as f32,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(square, square)),
                    source: Some(Rect::new(
                        match cell.state {
                            CellState::Mine => {
                                if board.state == GameState::Fail {
                                    8. * 8.
                                } else {
                                    continue;
                                }
                            }
                            CellState::Number(number) => match number {
                                0 => continue,
                                _ => {
                                    if cell.revealed {
                                        (number - 1) as f32 * 8.
                                    } else {
                                        continue;
                                    }
                                }
                            },
                        },
                        0.,
                        8.,
                        8.,
                    )),
                    ..Default::default()
                },
            );
        }
    }

    if board.grid[board.hovering].revealed && is_mouse_button_down(MouseButton::Right) {
        match board.grid[board.hovering].state {
            CellState::Number(number) => {
                if number != 0 {
                    let (x, y) = board.hovering;

                    draw_rectangle(
                        square * (x as i32 - 1) as f32,
                        square * (y as i32 - 1) as f32,
                        square * 3.,
                        square * 3.,
                        color_u8!(255, 0, 0, 30),
                    );
                };
            },
            _ => (),
        };
    };

    if board.state == GameState::Fail {
        draw_rectangle(
            0.,
            0.,
            screen_width(),
            screen_height(),
            color_u8!(255, 128, 128, 64),
        );
    }

    if board.state == GameState::Win {
        draw_rectangle(
            0.,
            0.,
            screen_width(),
            screen_height(),
            color_u8!(128, 255, 128, 64),
        );
    }


    let text = &format!(
        "{}",
        (Instant::now() - board.start_time).as_secs(),
    );
    draw_text_ex(
        text,
        square,
        square * 2.,
        TextParams {
            color: color_u8!(0, 255, 0, 64),
            font_size: (square * 3.) as u16,
            ..Default::default()
        }
    );

    let text = &format!(
        "{}",
        board.mine_count as isize - board.flag_count as isize
    );
    draw_text_ex(
        text,
        (square * board_width as f32) - square * 2. - square * text.len() as f32,
        square * 2.,
        TextParams {
            color: color_u8!(255, 0, 0, 64),
            font_size: (square * 3.) as u16,
            ..Default::default()
        }
    );
}
