use std::io::{self, Write};

use rand::Rng;

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

    // Placer 5 arbres aléatoirement
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        loop {
            let x = rng.gen_range(0..map.width);
            let y = rng.gen_range(0..map.height);

            // Assurez-vous que l'arbre ne se trouve pas sur un mur, le joueur, ou un monstre
            if let Some(tile) = map.get_tile(x, y) {
                if matches!(tile, Tile::Empty) {
                    map.set_tile(x, y, Tile::Tree);
                    break;
                }
            }
        }
    }

    let mut is_cutting_tree = false;
    let mut tree_x = 0;
    let mut tree_y = 0;

    loop {
        // Afficher la carte avec la position actuelle du joueur
        map.display(player_x, player_y);
    
        // Demander au joueur de se déplacer
        print!("Déplacez-vous (Z: Haut, Q: Gauche, S: Bas, D: Droite) : ");
        io::stdout().flush().unwrap(); // Pour s'assurer que l'affichage soit immédiat
    
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erreur de lecture");
    
        // Calcul des nouvelles coordonnées après le déplacement
        let (new_x, new_y) = match input.trim().to_lowercase().as_str() {
            "z" => (player_x, if player_y > 0 { player_y - 1 } else { player_y }),
            "s" => (player_x, if player_y < map.height - 1 { player_y + 1 } else { player_y }),
            "q" => (if player_x > 0 { player_x - 1 } else { player_x }, player_y),
            "d" => (if player_x < map.width - 1 { player_x + 1 } else { player_x }, player_y),
            _ => {
                println!("Commande invalide. Essayez à nouveau.");
                continue;
            }
        };

        // Vérifier si la nouvelle position est valide
        if let Some(tile) = map.get_tile(new_x, new_y) {
            match tile {
                Tile::Wall => {
                    println!("Vous ne pouvez pas avancer, vous êtes face à un mur.");
                    continue;
                }
                Tile::Tree => {
                    if is_cutting_tree {
                        // Si le joueur a confirmé qu'il coupe l'arbre
                        println!("Vous avez coupé l'arbre !");
                        map.set_tile(new_x, new_y, Tile::Empty);
                        is_cutting_tree = false; // Réinitialiser
                    } else {
                        // Si le joueur se dirige vers l'arbre sans avoir confirmé
                        println!("Voulez-vous couper l'arbre ? (pour valider, continuez en direction de l'arbre)");
                        is_cutting_tree = true;
                        tree_x = new_x;
                        tree_y = new_y;
                    }
                    continue; // Le dinosaure ne se déplace pas pour ce tour
                }
                Tile::Monster => {
                    println!("Coup dur, vous avez été tué par le dinosaure !");
                    break; // Terminer la boucle de jeu
                }
                _ => {} // Permet le déplacement
            }
        }

        // Mettre à jour les coordonnées du joueur
        player_x = new_x;
        player_y = new_y;

        // Si le joueur atteint l'objectif, terminer le jeu
        if player_x == 9 && player_y == 9 {
            println!("Félicitations ! Vous avez atteint l'objectif.");
            break;
        }
    }


}
