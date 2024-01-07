use super::tile::TileDefinitionsStore;

pub fn add_prefab_tiles(store: &mut TileDefinitionsStore) {
    // 0: Bar
    store.add_tile_definition(vec![[0, 0], [0, -1], [0, -2], [0, -3], [0, -4]]);

    // 1: Cross
    store.add_tile_definition(vec![[0, -1], [1, 0], [1, -1], [1, -2], [2, -1]]);

    // 2: L
    store.add_tile_definition(vec![[0, 0], [0, -1], [0, -2], [0, -3], [1, -3]]);
}
