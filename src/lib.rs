mod katamino;

use katamino::board::Board;
use katamino::display::display_board;
use katamino::prefab_tile_definitions::add_prefab_tiles;
use katamino::tile::{TileDefinitionsStore, TileOrientation};

pub fn lol() {
    let mut t = TileDefinitionsStore::new(); // Instantiate the TileDefinitionsStore struct
    add_prefab_tiles(&mut t);
    let _d1 = t.get_tile_definition(0u16);
    let _d2 = t.get_tile_definition(1u16);
    let _d3 = t.get_tile_definition(2u16);
    let mut b = Board::new();
    match b.add_tile(_d1, 0, 0, TileOrientation::UP) {
        Ok(_) => {
            // Print everything from board.cells
            for kv in b.cells.iter() {
                println!("({:?}, {:?}): {:?}", kv.0[0], kv.0[1], kv.1);
            }
        }
        Err(msg) => {
            println!("error placing! {:?}", msg)
        }
    }
    match b.add_tile(_d2, 1, 0, TileOrientation::UP) {
        Ok(_) => {
            // Print everything from board.cells
            for kv in b.cells.iter() {
                println!("({:?}, {:?}): {:?}", kv.0[0], kv.0[1], kv.1);
            }
        }
        Err(msg) => {
            println!("error placing! {:?}", msg)
        }
    }
    match b.add_tile(_d2, 1, 0, TileOrientation::UP) {
        Ok(_) => {
            // Print everything from board.cells
            for kv in b.cells.iter() {
                println!("({:?}, {:?}): {:?}", kv.0[0], kv.0[1], kv.1);
            }
        }
        Err(msg) => {
            println!("error placing! {:?}", msg)
        }
    }
    match b.add_tile(_d3, 4, 1, TileOrientation::UP) {
        Ok(_) => {
            // Print everything from board.cells
            for kv in b.cells.iter() {
                println!("({:?}, {:?}): {:?}", kv.0[0], kv.0[1], kv.1);
            }
        }
        Err(msg) => {
            println!("error placing! {:?}", msg)
        }
    }
    display_board(&b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        lol();
    }
}
