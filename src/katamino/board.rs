use std::collections::HashMap;

use super::tile::{Tile, TileDefinition, TileOrientation};

pub struct Board<'a> {
    pub cells: HashMap<[i16; 2], u16>,
    pub tiles: Vec<Tile<'a>>,
    pub x_min: i16,
    pub x_max: i16,
    pub y_min: i16,
    pub y_max: i16,
}

impl<'a> Board<'a> {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
            tiles: Vec::new(),
            x_min: 0,
            x_max: 0,
            y_min: 0,
            y_max: 0,
        }
    }
    fn can_place_tile(&self, t: &TileDefinition, px: i16, py: i16, o: TileOrientation) -> bool {
        for coord_index in 0..t.coords.len() {
            let def_coord = t.coords[coord_index];
            let check_coord = [px + def_coord[0], py + def_coord[1]];
            if self.cells.contains_key(&check_coord) {
                return false;
            }
        }
        true
    }
    fn place_tile(&mut self, t: &Tile) {
        for coord_index in 0..t.definition.coords.len() {
            let def_coord = t.definition.coords[coord_index];
            let check_coord = [t.pos_x + def_coord[0], t.pos_y + def_coord[1]];
            self.cells.insert(check_coord, t.id);
            if check_coord[0] > self.x_max {
                self.x_max = check_coord[0]
            } else if check_coord[0] < self.x_min {
                self.x_min = check_coord[0]
            }
            if check_coord[1] > self.y_max {
                self.y_max = check_coord[1]
            } else if check_coord[1] < self.y_min {
                self.y_min = check_coord[1]
            }
        }
    }
    fn unplace_tile(&mut self, t: &Tile) {
        for coord_index in 0..t.definition.coords.len() {
            let def_coord = t.definition.coords[coord_index];
            let check_coord = [t.pos_x + def_coord[0], t.pos_y + def_coord[1]];
            self.cells.remove(&check_coord);
        }
    }
    pub fn add_tile(
        &mut self,
        t: &'a TileDefinition,
        px: i16,
        py: i16,
        o: TileOrientation,
    ) -> Option<u16> {
        if !self.can_place_tile(t, px, py, o) {
            return None;
        }
        let id = self.tiles.len() as u16;
        let tile: Tile<'a> = Tile {
            id,
            definition: t,
            pos_x: px,
            pos_y: py,
            orientation: o,
        };
        self.place_tile(&tile);
        self.tiles.push(tile);
        Some(id)
    }
    pub fn remove_tile(&mut self, id: u16) {
        let t = self.tiles.remove(id as usize);
        self.unplace_tile(&t);
    }
}
