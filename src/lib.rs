mod katamino;

use katamino::board::Board;
use katamino::display::{display_board, undisplay_board};
use katamino::prefab_tile_definitions::add_prefab_tiles;
use katamino::tile::{TileDefinitionsStore, TileOrientation};
use std::thread;
use std::time::Duration;

pub fn lol() {
    let mut t = TileDefinitionsStore::new(); // Instantiate the TileDefinitionsStore struct
    add_prefab_tiles(&mut t);
    let _d2 = t.get_tile_definition(1u16);
    let mut b = Board::new();

    let iterations_per_second = 12;
    let iteration_duration = Duration::from_secs_f64(1.0 / iterations_per_second as f64);

    let mut previous_tile_id = 0u16;

    for i in 0..600 {
        match b.add_tile(_d2, i % 10, 0, TileOrientation::UP) {
            Ok(id) => {
                previous_tile_id = id;
            }
            Err(msg) => {
                println!("error placing! {:?}", msg)
            }
        }

        display_board(&b);
        thread::sleep(iteration_duration);
        undisplay_board(&b);
        b.remove_tile(previous_tile_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        lol();
    }
}
