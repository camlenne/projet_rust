use std::fmt;
use crate::player::Player; 


#[derive(Debug, Clone)]  // Ajoute `Clone` ici
pub enum Tile {
    Empty,
    Wall,
    Monster,
    Start,
    End,
    Life,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match *self {
            Tile::Empty => '⬜',
            Tile::Wall => '🧱',
            Tile::Monster => '👾',
            Tile::Start => '🔛',
            Tile::End => '🔚',
            Tile::Life => '🩹',
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
        let tiles = Map::generate_map(width, height); // Cela fonctionnera maintenant
        Map { width, height, tiles }
    }

    //TODO ajouter la partie aléatoire de la carte ici
    fn generate_map(width: usize, height: usize) -> Vec<Vec<Tile>> {
        let mut tiles = vec![vec![Tile::Empty; width]; height];  // Initialise toute la carte avec des tuiles vides

        // Exemple de génération : Remplir avec des murs de manière aléatoire
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        for y in 0..height {
            for x in 0..width {
                // Logique de génération aléatoire : 10% de chance d'une tuile mur
                // TODO: vérifier que il n'y a pas de chevauchements de mur/monstrer etc...
                if rng.gen_range(0..100) < 15 {
                    tiles[y][x] = Tile::Wall;
                }
                if rng.gen_range(0..100) < 5 {
                    tiles[y][x] = Tile::Monster;
                }
                if rng.gen_range(0..100) < 5 {
                    tiles[y][x] = Tile::Life;
                }
            }
        }
        tiles[0][0] = Tile::Start;// TODO le début ne doit pas etre superposé par un dino
        tiles[width-1][height-1] = Tile::End;// TODO la fin doit etre accessible 
        tiles
    }
    pub fn display(&self, players: &Vec<Player>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let mut printed = false;
                for player in players {
                    if player.x == x && player.y == y {
                        print!("{}", player.emoji);  // Afficher un joueur
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
    //to generate to a specific place 
    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        if x < self.width && y < self.height {
            self.tiles[y][x] = tile;
        }
    }
}