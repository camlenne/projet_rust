use std::fmt;
use rand::prelude::SliceRandom;
use std::collections::VecDeque;
use crate::player::Player;
use rand::Rng;

#[derive(Debug, Clone, Copy)]  // Ajoute `Clone` et `Copy` ici
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
        let mut tiles = vec![vec![Tile::Wall; width]; height]; // Initialiser avec des murs
        let mut rng = rand::thread_rng();

        // Liste des directions pour se déplacer (haut, bas, gauche, droite)
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        // Fonction pour vérifier si une cellule est dans les limites et vide
        let is_within_bounds = |x: isize, y: isize| -> bool {
            x >= 0 && y >= 0 && (x as usize) < width && (y as usize) < height
        };

        // Utiliser une file pour effectuer une recherche ou un tracé
        let mut queue = VecDeque::new();
        queue.push_back((0, 0)); // Commencer au point de départ
        tiles[0][0] = Tile::Start;

        // Tracer un chemin du début à la fin
        while let Some((x, y)) = queue.pop_front() {
            // Mélanger les directions pour un effet aléatoire
            let mut shuffled_directions = directions.clone();
            shuffled_directions.shuffle(&mut rng);

            for (dx, dy) in shuffled_directions {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                // Vérifier les limites et si la cellule est un mur
                if is_within_bounds(nx, ny) && matches!(tiles[ny as usize][nx as usize], Tile::Wall) {
                    let next_x = nx as usize;
                    let next_y = ny as usize;

                    // Ouvrir un chemin
                    tiles[next_y][next_x] = Tile::Empty;
                    queue.push_back((next_x, next_y));
                }
            }
        }

        // Définir la sortie (End) à la dernière cellule visitée
        tiles[height - 1][width - 1] = Tile::End;

        // Ajouter des obstacles aléatoires sans bloquer le chemin
        for _ in 0..(width * height / 5) {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);

            if matches!(tiles[y][x], Tile::Empty) {
                tiles[y][x] = Tile::Wall;
            }
        }

        // Placer des murs et des arbres aléatoirement
        let tree_count = (width * height) / 15; // Exemple : 10% de la carte en arbres
        let life_count = (width * height) / 20; // Exemple : 10% de la carte en arbres

        let mut placed_trees = 0;
        while placed_trees < tree_count {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);

            // Ajouter un arbre uniquement sur une case vide non utilisée dans le chemin principal
            if matches!(tiles[y][x], Tile::Empty) {
                tiles[y][x] = Tile::Tree;
                placed_trees += 1;
            }
        }

        let mut placed_life = 0;
        while placed_life < life_count {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);

            // Ajouter une vie uniquement sur une case vide non utilisée dans le chemin principal
            if matches!(tiles[y][x], Tile::Empty) {
                tiles[y][x] = Tile::Life;
                placed_life += 1;
            }
        }

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
pub fn check_obj(map: &mut Map, new_x: usize, new_y: usize, player: &mut Player) -> bool {
    if let Some(tile) = map.get_tile(new_x, new_y) {
        match tile {
            Tile::Wall => {
                println!("Vous êtes face à un mur.");
                false
            }
            Tile::Tree => {
                println!("Voulez-vous couper l'arbre ? (pour valider, tapez 'y').");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                if input.trim() == "y" {
                    println!("Vous avez coupé l'arbre !");
                    map.set_tile(new_x, new_y, Tile::Empty); // Remplace l'arbre par une tuile vide
                    true
                } else {
                    println!("L'arbre est toujours présent ! Tapez 'y' pour le couper.");
                    false
                }
            }
            Tile::Life => {
                println!("Gain de vie !");
                player.gain_life(20); // Ajouter 20 points de vie
                map.set_tile(new_x, new_y, Tile::Empty); // Remplace la case par une tuile vide
                true
            }
            _ => {
                println!("Vous avancez sur une case vide.");
                true // Case traversable
            }
        }
    } else {
        println!("Cette case est hors limites !");
        false // Hors limites, on ne peut pas avancer
    }
}

