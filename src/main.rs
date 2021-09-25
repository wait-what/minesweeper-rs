#![windows_subsystem = "windows"]

use macroquad::prelude::*;

pub mod board;
pub mod input;
pub mod ui;
mod view;

use board::Board;
use ui::Ui;

fn window_conf() -> Conf {
    Conf {
        window_title: "minesweeper-rs".to_owned(),
        window_width: 20 * 30,
        window_height: 15 * 30,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let atlas = Texture2D::from_file_with_format(include_bytes!("texture.png"), None);
    atlas.set_filter(FilterMode::Nearest);

    let mut ui = Ui::new();
    let mut board = Board::new((ui.width, ui.height), ui.mine_count(), ui.color());

    loop {
        if ui.update() {
            board = Board::new((ui.width, ui.height), ui.mine_count(), ui.color());
        } else {
            input::get_input(&mut board, &mut ui);
        };

        view::render(&board, &atlas);
        next_frame().await;
    }
}
