use rand::Rng;
use crate::game::{Map, Tile};

// Fonction pour placer des arbres aléatoirement
pub fn place_trees(map: &mut Map, tree_count: usize, rng: &mut rand::rngs::ThreadRng) {
    for _ in 0..tree_count {
        loop {
            let x = rng.gen_range(0..map.width);
            let y = rng.gen_range(0..map.height);

            if let Some(tile) = map.get_tile(x, y) {
                if matches!(tile, Tile::Empty) {
                    map.set_tile(x, y, Tile::Tree);
                    break;
                }
            }
        }
    }
}

// Fonction pour placer des murs aléatoirement
pub fn place_walls(map: &mut Map, wall_count: usize, rng: &mut rand::rngs::ThreadRng) {
    let mut placed_walls = 0;

    while placed_walls < wall_count {
        let x = rng.gen_range(0..map.width);
        let y = rng.gen_range(0..map.height);

        if let Some(tile) = map.get_tile(x, y) {
            if matches!(tile, Tile::Empty) {
                map.set_tile(x, y, Tile::Wall);
                placed_walls += 1;

                // Essayer de placer des murs adjacents
                if placed_walls < wall_count && rng.gen_bool(0.7) {
                    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                    let mut placed_adjacent = 0;

                    for (dx, dy) in directions {
                        let nx = (x as isize + dx) as usize;
                        let ny = (y as isize + dy) as usize;

                        if nx < map.width && ny < map.height {
                            if let Some(adj_tile) = map.get_tile(nx, ny) {
                                if matches!(adj_tile, Tile::Empty) && placed_walls < wall_count {
                                    map.set_tile(nx, ny, Tile::Wall);
                                    placed_walls += 1;
                                    placed_adjacent += 1;

                                    if placed_adjacent == 2 && placed_walls >= 3 {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
