use std::fmt;


#[derive(Debug, Clone)]  // Ajoute `Clone` ici
pub enum Tile {
    Empty,
    Wall,
    Monster,
    Start,
    End,
    Tree,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match *self {
            Tile::Empty => '⬜',
            Tile::Wall => '🧱',
            Tile::Monster => '👾',
            Tile::Start => '🔛',
            Tile::End => '🔚',
            Tile:: Tree => '🌳',
        };
        write!(f, "{}", symbol)
    }
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![Tile::Empty; width]; height]; // Cela fonctionnera maintenant
        Map { width, height, tiles }
    }

    pub fn display(&self, player_x: usize, player_y: usize) {
        for y in 0..self.height {
            for x in 0..self.width {
                // Si la position du joueur correspond à cette case, on affiche "P" pour le joueur
                if x == player_x && y == player_y {
                    print!("🦖");
                } else {
                    print!("{}", self.tiles[y][x]);
                }
            }
            println!();
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        if x < self.width && y < self.height {
            self.tiles[y][x] = tile;
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        if x < self.width && y < self.height {
            Some(&self.tiles[y][x])
        } else {
            None
        }
    }
}