use std::fmt;
use rand::prelude::SliceRandom;
use std::collections::VecDeque;
use crate::player::Player;
use rand::Rng;

// Définition de l'énumération des types de tuiles possibles
#[derive(Debug, Clone, Copy, PartialEq)]  // Ajoutez PartialEq ici pour permettre les comparaisons
pub enum Tile {
    Empty,  // Case vide
    Wall,   // Mur
    Tree,   // Arbre
    Start,  // Point de départ
    End,    // Point de fin
    Life,   // Case de vie (gain de vie)
    Piege,  // Case piège (perte de vie)
}

// Implémentation du trait fmt::Display pour afficher les tuiles avec des symboles
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Associer chaque type de tuile à un symbole
        let symbol = match *self {
            Tile::Empty => '⬜',
            Tile::Wall => '🧱',
            Tile::Start => '🔛',
            Tile::Tree => '🌳',
            Tile::End => '🔚',
            Tile::Life => '🩹',
            Tile::Piege => '💣',
        };
        write!(f, "{}", symbol)  // Retourne le symbole associé à la tuile
    }
}

// Struct représentant une carte avec des dimensions et des tuiles
pub struct Map {
    pub width: usize,   // Largeur de la carte
    pub height: usize,  // Hauteur de la carte
    tiles: Vec<Vec<Tile>>, // Liste des tuiles de la carte
}

impl Map {
    // Fonction pour créer une nouvelle carte avec des dimensions spécifiques
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = Map::generate_map(width, height); // Appel de la fonction pour générer la carte
        Map { width, height, tiles }
    }
    
    // Fonction pour générer la carte avec des murs et un chemin vide
    fn generate_map(width: usize, height: usize) -> Vec<Vec<Tile>> {
        let mut tiles = vec![vec![Tile::Wall; width]; height]; // Initialisation avec des murs
        let mut rng = rand::thread_rng();  // Générateur de nombres aléatoires

        // Directions possibles pour se déplacer (haut, bas, gauche, droite)
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        // Fonction pour vérifier si une position est dans les limites de la carte et si elle est vide
        let is_within_bounds = |x: isize, y: isize| -> bool {
            x >= 0 && y >= 0 && (x as usize) < width && (y as usize) < height
        };

        // Utilisation d'une file pour effectuer une recherche ou un tracé de chemin
        let mut queue = VecDeque::new();
        queue.push_back((0, 0)); // Point de départ
        tiles[0][0] = Tile::Start;  // Marque la position de départ

        // Tracer un chemin du début à la fin en explorant les tuiles vides
        while let Some((x, y)) = queue.pop_front() {
            let mut shuffled_directions = directions.clone();  // Mélange des directions pour un effet aléatoire
            shuffled_directions.shuffle(&mut rng);

            for (dx, dy) in shuffled_directions {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if is_within_bounds(nx, ny) && matches!(tiles[ny as usize][nx as usize], Tile::Wall) {
                    let next_x = nx as usize;
                    let next_y = ny as usize;

                    // Ouvrir un chemin
                    tiles[next_y][next_x] = Tile::Empty;
                    queue.push_back((next_x, next_y));
                }
            }
        }

        // Marquer la sortie à la dernière cellule visitée
        tiles[height - 1][width - 1] = Tile::End;

        // Ajouter des obstacles aléatoires sans bloquer le chemin principal
        for _ in 0..(width * height / 5) {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);

            if matches!(tiles[y][x], Tile::Empty) {
                tiles[y][x] = Tile::Wall;
            }
        }

        // Ajouter des arbres aléatoires
        let tree_count = (width * height) / 15; // Exemple de ratio d'arbres
        let life_count = (width * height) / 20; // Exemple de ratio de vies
        let piege_count = (width * height) / 25; // Exemple de ratio de pièges

        // Placer des arbres aléatoirement sur la carte
        let mut placed_trees = 0;
        while placed_trees < tree_count {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);

            if matches!(tiles[y][x], Tile::Empty) {
                tiles[y][x] = Tile::Tree;
                placed_trees += 1;
            }
        }

        // Placer des vies aléatoirement sur la carte
        let mut placed_life = 0;
        while placed_life < life_count {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);

            if matches!(tiles[y][x], Tile::Empty) {
                tiles[y][x] = Tile::Life;
                placed_life += 1;
            }
        }

        // Placer des pièges aléatoirement sur la carte
        let mut placed_piege = 0;
        while placed_piege < piege_count {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);

            if matches!(tiles[y][x], Tile::Empty) {
                tiles[y][x] = Tile::Piege;
                placed_piege += 1;
            }
        }

        tiles
    }

    // Afficher la carte avec les joueurs
    pub fn display(&self, players: &Vec<Player>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let mut printed = false;
                for player in players {
                    if player.x == x && player.y == y {
                        print!("{}", player.emoji);  // Affiche l'emoji du joueur sur la case
                        printed = true;
                        break;
                    }
                }
                if !printed {
                    print!("{}", self.tiles[y][x]); // Affiche la tuile (mur, arbre, etc.)
                }
            }
            println!(); // Nouvelle ligne après chaque ligne de la carte
        }
    }

    // Changer la tuile d'une position spécifique
    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        if x < self.width && y < self.height {
            self.tiles[y][x] = tile;
        }
    }

    // Obtenir la tuile d'une position spécifique
    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        if x < self.width && y < self.height {
            Some(&self.tiles[y][x])  // Retourne une référence à la tuile
        } else {
            None  // Retourne `None` si les coordonnées sont hors limites
        }
    }
}

// Trouver une case vide sur la carte pour y placer quelque chose
pub fn find_a_place(map: &mut Map, size_map_x: i32, size_map_y: i32) -> (usize, usize) {
    let mut rng = rand::thread_rng();

    loop {
        // Générer de nouvelles coordonnées aléatoires dans les limites de la carte
        let new_x = rng.gen_range(10..size_map_x);
        let new_y = rng.gen_range(5..size_map_y);

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

// Vérifier la case sur laquelle le joueur se trouve et interagir avec elle
pub fn check_obj(map: &mut Map, new_x: usize, new_y: usize, player: &mut Player) -> bool {
    if let Some(tile) = map.get_tile(new_x, new_y) {
        match tile {
            Tile::Wall => {
                false  // Mur, on ne peut pas passer
            }
            Tile::Tree => {
                // Demander si le joueur veut couper l'arbre
                println!("Voulez-vous couper l'arbre ? (pour valider, tapez 'y').");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                if input.trim() == "y" {
                    println!("Vous avez coupé l'arbre !");
                    map.set_tile(new_x, new_y, Tile::Empty);  // Remplace l'arbre par une case vide
                    true
                } else {
                    println!("L'arbre est toujours présent ! Tapez 'y' pour le couper.");
                    false
                }
            }
            Tile::Life => {
                // Le joueur gagne des points de vie
                player.gain_life(20);
                map.set_tile(new_x, new_y, Tile::Empty);  // Remplace la case de vie par une case vide
                true
            }
            Tile::Piege => {
                // Le joueur perd des points de vie à cause du piège
                player.remove_life(20);
                map.set_tile(new_x, new_y, Tile::Empty);  // Remplace le piège par une case vide
                true
            }
            _ => {
                true  // Case traversable
            }
        }
    } else {
        false  // Hors limites, on ne peut pas avancer
    }
}
