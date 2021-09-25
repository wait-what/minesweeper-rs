use crate::{ board::Board, ui::Ui };
use macroquad::prelude::*;

pub fn get_input(board: &mut Board, ui: &mut Ui) {
    if is_key_pressed(KeyCode::Space) {
        ui.showing = !ui.showing;
    };

    if ui.showing {
        return;
    }

    if is_key_pressed(KeyCode::R) {
        *board = Board::new((ui.width, ui.height), ui.mine_count(), ui.color());
    }

    let (board_width, board_height) = board.grid.size();
    let square = if screen_width() / board_width as f32 > screen_height() / board_height as f32 {
        screen_height() / board_height as f32
    } else {
        screen_width() / board_width as f32
    };

    let coordinates = (
        (mouse_position().0 / square) as usize,
        (mouse_position().1 / square) as usize,
    );

    if coordinates.0 >= board_width || coordinates.1 >= board_height {
        return;
    }

    board.hovering = coordinates;

    if is_mouse_button_pressed(MouseButton::Left) {
        board.reveal_cell(coordinates);
    };

    if is_mouse_button_pressed(MouseButton::Right) {
        board.toggle_flag(coordinates);
    };
}
