use std::fmt;
use std::io::{self, Write};


#[derive(Debug, Clone)]  // Ajoute `Clone` ici
enum Tile {
    Empty,
    Wall,
    Monster,
    Start,
    End,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match *self {
            Tile::Empty => '.',
            Tile::Wall => '🧱',
            Tile::Monster => '👾',
            Tile::Start => '🔛',
            Tile::End => '🔚',
        };
        write!(f, "{}", symbol)
    }
}

struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![Tile::Empty; width]; height]; // Cela fonctionnera maintenant
        Map { width, height, tiles }
    }

    fn display(&self, player_x: usize, player_y: usize) {
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

    fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        if x < self.width && y < self.height {
            self.tiles[y][x] = tile;
        }
    }
}

fn main() {
    // Créer une carte de 10x10
    let mut map = Map::new(30, 10);

    let mut player_x = 0;
    let mut player_y = 0;

    // Définir quelques cases
    map.set_tile(0, 0, Tile::Start);
    map.set_tile(9, 9, Tile::End);
    map.set_tile(3, 4, Tile::Wall);
    map.set_tile(3, 6, Tile::Wall);
    map.set_tile(3, 7, Tile::Wall);
    map.set_tile(3, 8, Tile::Wall);
    map.set_tile(3, 9, Tile::Wall);
    map.set_tile(5, 5, Tile::Monster);

    loop {
        // Afficher la carte avec la position actuelle du joueur
        map.display(player_x, player_y);
    
        // Demander au joueur de se déplacer
        print!("Déplacez-vous (Z: Haut, Q: Gauche, S: Bas, D: Droite) : ");
        io::stdout().flush().unwrap(); // Pour s'assurer que l'affichage soit immédiat
    
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erreur de lecture");
    
        match input.trim().to_lowercase().as_str() {
            "z" => {
                if player_y > 0 { player_y -= 1; } // Déplacer vers le haut
            }
            "s" => {
                if player_y < map.height - 1 { player_y += 1; } // Déplacer vers le bas
            }
            "q" => {
                if player_x > 0 { player_x -= 1; } // Déplacer vers la gauche
            }
            "d" => {
                if player_x < map.width - 1 { player_x += 1; } // Déplacer vers la droite
            }
            _ => {
                println!("Commande invalide. Essayez à nouveau.");
                continue;
            }
        }
    
        // Si le joueur atteint l'objectif, terminer le jeu
        if player_x == 9 && player_y == 9 {
            println!("Félicitations ! Vous avez atteint l'objectif.");
            break;
        }
    }
}