use std::io::{self, Write};
use rand::Rng;

mod game;
mod map_generation;
mod score; // Nouveau module

use game::{Map, Tile};
use map_generation::{place_trees, place_walls};
use score::{save_score, display_last_scores}; // Import des fonctions liées aux scores

fn main() {
    let mut map = Map::new(30, 10);
    let mut player_x = 0;
    let mut player_y = 0;

    map.set_tile(0, 0, Tile::Start);
    map.set_tile(9, 9, Tile::End);
    map.set_tile(5, 5, Tile::Monster);

    let mut rng = rand::thread_rng();

    // Génération de la carte
    place_trees(&mut map, 8, &mut rng);
    place_walls(&mut map, 15, &mut rng);

    let mut cutting_tree = None;
    let mut actions = 0; // Compteur d'actions du joueur

    loop {
        map.display(player_x, player_y);

        print!("Déplacez-vous (Z: Haut, Q: Gauche, S: Bas, D: Droite, T: Terminer) : ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erreur de lecture");
        let input = input.trim().to_lowercase();

        if input == "t" {
            println!("Terminaison demandée !");
            save_score(actions);
            display_last_scores();
            break;
        }

        if let Some((tree_x, tree_y)) = cutting_tree {
            if input == "y" {
                println!("Vous avez coupé l'arbre !");
                map.set_tile(tree_x, tree_y, Tile::Empty);
                cutting_tree = None;
            } else {
                println!("L'arbre est toujours là (pour couper l'arbre, tapez 'y').");
            }
            continue;
        }

        let (new_x, new_y) = match input.as_str() {
            "z" => (player_x, if player_y > 0 { player_y - 1 } else { player_y }),
            "s" => (player_x, if player_y < map.height - 1 { player_y + 1 } else { player_y }),
            "q" => (if player_x > 0 { player_x - 1 } else { player_x }, player_y),
            "d" => (if player_x < map.width - 1 { player_x + 1 } else { player_x }, player_y),
            _ => {
                println!("Commande invalide. Essayez à nouveau.");
                continue;
            }
        };

        if let Some(tile) = map.get_tile(new_x, new_y) {
            match tile {
                Tile::Wall => {
                    println!("Vous êtes face à un mur.");
                    continue;
                }
                Tile::Tree => {
                    println!("Voulez-vous couper l'arbre ? (pour valider, tapez 'y').");
                    cutting_tree = Some((new_x, new_y));
                    continue;
                }
                Tile::Monster => {
                    println!("Vous avez rencontré un dinosaure ! Game over !");
                    save_score(actions);
                    display_last_scores();
                    break;
                }
                _ => {}
            }
        }

        player_x = new_x;
        player_y = new_y;
        actions += 1;

        if player_x == 9 && player_y == 9 {
            println!("Félicitations ! Vous avez atteint l'objectif en {actions} actions.");
            save_score(actions);
            display_last_scores();
            break;
        }
    }
}
