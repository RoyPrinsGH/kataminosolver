use super::board::Board;

extern crate crossterm;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{
    cursor::{DisableBlinking, MoveTo},
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdout, Write};

fn get_color_for_tile_id(id: u16) -> Color {
    match id {
        0 => Color::Red,
        1 => Color::Green,
        2 => Color::Blue,
        3 => Color::Yellow,
        4 => Color::Magenta,
        5 => Color::Cyan,
        6 => Color::DarkRed,
        7 => Color::DarkGreen,
        8 => Color::DarkBlue,
        9 => Color::DarkYellow,
        10 => Color::DarkMagenta,
        11 => Color::DarkCyan,
        12 => Color::Grey,
        13 => Color::DarkGrey,
        14 => Color::Black,
        15 => Color::White,
        _ => Color::White,
    }
}

pub fn display_board(b: &Board) {
    let top_corner_x = b.x_min;
    let top_corner_y = b.y_max;

    let mut stdout = stdout();
    stdout.execute(DisableBlinking).unwrap();
    stdout.execute(Clear(ClearType::All)).unwrap(); // Clear the screen

    for cell_key in b.cells.keys() {
        let cell_id = b.cells[cell_key];
        let x = cell_key[0] - top_corner_x;
        let y = top_corner_y - cell_key[1];

        // Print 'X' at cell position
        stdout
            .execute(MoveTo(x as u16, y as u16))
            .unwrap()
            .execute(SetForegroundColor(get_color_for_tile_id(cell_id)))
            .unwrap()
            .execute(Print('█'))
            .unwrap();
    }

    stdout.flush().unwrap();
    stdout.execute(MoveTo(0 as u16, 6 as u16)).unwrap();
}
