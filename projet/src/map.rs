use std::fmt;
use rand::prelude::SliceRandom;
use std::collections::VecDeque;
use crate::player::Player;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]  // Ajoutez PartialEq ici
pub enum Tile {
    Empty,
    Wall,
    Tree,
    Start,
    End,
    Life,
    Piege,
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
            Tile::Piege => '💣',
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
        let tree_count = (width * height) / 15;
        let life_count = (width * height) / 20;
        let piege_count = (width * height) / 25; 

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

        let mut placed_piege = 0;
        while placed_piege < piege_count {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);

            // Ajouter une vie uniquement sur une case vide non utilisée dans le chemin principal
            if matches!(tiles[y][x], Tile::Empty) {
                tiles[y][x] = Tile::Piege;
                placed_piege += 1;
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



pub fn find_a_place(map: &mut Map, size_map_x: i32, size_map_y: i32) -> (usize, usize) {
    let mut rng = rand::thread_rng();

    loop {
        // Générer de nouveaux x et y aléatoires dans les limites de la carte
        let new_x = rng.gen_range(10..size_map_x);
        let new_y = rng.gen_range(5..size_map_y);

        // Vérifier si la case est vide
        if let Some(tile) = map.get_tile(new_x as usize, new_y as usize) {
            match *tile {
                Tile::Empty => {
                    // Si la case est vide, retourner les coordonnées
                    return (new_x as usize, new_y as usize);
                }
                _ => {
                    // Si la case n'est pas vide, continuer la boucle pour générer de nouvelles coordonnées
                    continue;
                }
            }
        }
    }
}



pub fn check_obj(map: &mut Map, new_x: usize, new_y: usize, player: &mut Player) -> bool {
    if let Some(tile) = map.get_tile(new_x, new_y) {
        match tile {
            Tile::Wall => {
                //println!("Vous êtes face à un mur.");
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
                //println!("Gain de vie !");
                player.gain_life(20); // Ajouter 20 points de vie
                map.set_tile(new_x, new_y, Tile::Empty); // Remplace la case par une tuile vide
                true
            }
            Tile::Piege => {
                //println!("Gain de vie !");
                player.remove_life(20); // enlever 20 points de vie
                map.set_tile(new_x, new_y, Tile::Empty); // Remplace la case par une tuile vide
                true
            }
            _ => {
                true // Case traversable
            }
        }
    } else {
        false // Hors limites, on ne peut pas avancer
    }
}

