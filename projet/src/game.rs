use std::fmt;


#[derive(Debug, Clone)]  // Ajoute `Clone` ici
pub enum Tile {
    Empty,
    Wall,
    Monster,
    Start,
    End,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match *self {
            Tile::Empty => '⬜',
            Tile::Wall => '🧱',
            Tile::Monster => '👾',
            Tile::Start => '🔛',
            Tile::End => '🔚',
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

    pub fn display(&self, players: &Vec<Player>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let mut printed = false;
                for player in players {
                    if player.x == x && player.y == y {
                        print!("🦖");  // Afficher un joueur
                        printed = true;
                        break;
                    }
                }
                if !printed {
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
}

pub struct Player{
    pub x: usize,
    pub y: usize,
    pub name: String,
    pub health: i32,
}

impl Player {
    pub fn new(name: &str,x: usize,y: usize,health: i32) -> Self {
        Player { 
            x, 
            y, 
            name: name.to_string(), 
            health,
        }
    }
    pub fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    pub fn move_down(&mut self, map_height: usize) {
        if self.y < map_height - 1 {
            self.y += 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self, map_width: usize) {
        if self.x < map_width - 1 {
            self.x += 1;
        }
    }
}
