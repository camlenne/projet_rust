use std::fmt;
use crate::player::Player; 
use rand::Rng;


#[derive(Debug, Clone)]  // Ajoute `Clone` ici
pub enum Tile {
    Empty,
    Wall,
    Tree,
    Start,
    End,
    Life,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match *self {
            Tile::Empty => '⬜',
            Tile::Wall => '🧱',
            Tile::Start => '🔛',
            Tile::Tree => '🌳',
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
        let tiles = vec![vec![Tile::Empty; width]; height]; // Initialise la carte avec des tuiles vides
        let mut rng = rand::thread_rng();

        // Placer des murs et des arbres aléatoirement
        let wall_count = (width * height) / 5;  // Exemple : 20% de la carte en murs
        let tree_count = (width * height) / 15; // Exemple : 10% de la carte en arbres
        let life_count = (width * height) / 20; // Exemple : 10% de la carte en arbres

        let mut map = Map {
            width,
            height,
            tiles,
        };

        place_walls(&mut map, wall_count, &mut rng);
        place_trees(&mut map, tree_count, &mut rng);
        place_life(&mut map, life_count, &mut rng);

        // Mettre les tuiles `Start` et `End`
        map.set_tile(0, 0, Tile::Start);
        map.set_tile(width - 1, height - 1, Tile::End);

        map.tiles
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
    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        if x < self.width && y < self.height {
            Some(&self.tiles[y][x]) // Retourne une référence à la tuile si elle existe
        } else {
            None // Retourne `None` si les coordonnées sont hors limites
        }
    }
}

//TODO rajouter les pansements

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
pub fn place_life(map: &mut Map, tree_count: usize, rng: &mut rand::rngs::ThreadRng) {
    for _ in 0..tree_count {
        loop {
            let x = rng.gen_range(0..map.width);
            let y = rng.gen_range(0..map.height);

            if let Some(tile) = map.get_tile(x, y) {
                if matches!(tile, Tile::Empty) {
                    map.set_tile(x, y, Tile::Life);
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
pub fn check_obj(map: &mut Map,new_x : usize, new_y : usize) -> bool{

    if let Some(tile) = map.get_tile(new_x, new_y) {
        match tile {
            Tile::Wall => {
                println!("Vous êtes face à un mur.");
                false
            }
            Tile::Tree => {
                println!("Voulez-vous couper l'arbre ? (pour valider, tapez 'y').");
                // Exemple d'interaction (si vous voulez confirmer une action) :
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                if input.trim() == "y" {
                    println!("Vous avez coupé l'arbre !");
                    map.set_tile(new_x, new_y, Tile::Empty); // Remplace l'arbre par une tuile vide
                    true
                }
                else{
                    println!("l'arbre est toujours présent ! tapez y pour le couper");
                    false
                }
            }
            Tile::Life => {
                println!("gain de vie");
                map.set_tile(new_x, new_y, Tile::Empty);
                true
            }
            _ => {
                println!("Vous avancez sur une case vide.");
                true // Case traversable
            }
        }
    }
    else {
        println!("Cette case est hors limites !");
        false // Hors limites, on ne peut pas avancer
    } 
}
