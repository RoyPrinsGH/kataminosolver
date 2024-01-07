pub struct TileDefinition {
    pub id: u16,
    pub coords: Vec<[i16; 2]>,
}

#[derive(Clone, Copy)]
pub enum TileOrientation {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

pub struct Tile<'a> {
    pub id: u16,
    pub definition: &'a TileDefinition,
    pub pos_x: i16,
    pub pos_y: i16,
    pub orientation: TileOrientation,
}

pub struct TileDefinitionsStore {
    tiles: Vec<TileDefinition>,
}

impl TileDefinitionsStore {
    pub fn new() -> Self {
        Self { tiles: Vec::new() }
    }
    pub fn add_tile_definition(&mut self, coords: Vec<[i16; 2]>) -> u16 {
        let id = self.tiles.len() as u16;
        self.tiles.push(TileDefinition { id, coords });
        id
    }
    pub fn get_tile_definition<'a>(&'a self, id: u16) -> &'a TileDefinition {
        &self.tiles[id as usize]
    }
}
