use std::io::{self, Write};

mod game;
use game::{Map, Tile}; // Importation des éléments de game.rs


fn main() {
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