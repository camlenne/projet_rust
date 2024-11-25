use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, Write};
use rand::Rng;

mod game;
use game::{Map, Tile};

fn main() {
    let mut map = Map::new(30, 10);
    let mut player_x = 0;
    let mut player_y = 0;

    map.set_tile(0, 0, Tile::Start);
    map.set_tile(9, 9, Tile::End);
    map.set_tile(5, 5, Tile::Monster);

    // Placer 5 arbres aléatoires
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
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

    // Placer les murs
    place_walls(&mut map, &mut rng);

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



fn place_walls(map: &mut Map, rng: &mut rand::rngs::ThreadRng) {
    let mut wall_count = 0;

    while wall_count < 7 {
        let x = rng.gen_range(0..map.width);
        let y = rng.gen_range(0..map.height);

        // Vérifier si la case est vide
        if let Some(tile) = map.get_tile(x, y) {
            if matches!(tile, Tile::Empty) {
                // Placer un mur
                map.set_tile(x, y, Tile::Wall);
                wall_count += 1;

                // Après avoir placé un mur, vérifier si on peut en placer d'autres adjacents
                if wall_count < 7 && rng.gen_bool(0.7) { // 70% de chance de créer des murs adjacents
                    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                    let mut placed_adjacent = 0;

                    for (dx, dy) in directions {
                        let nx = (x as isize + dx) as usize;
                        let ny = (y as isize + dy) as usize;

                        if nx < map.width && ny < map.height {
                            if let Some(adj_tile) = map.get_tile(nx, ny) {
                                if matches!(adj_tile, Tile::Empty) && wall_count < 7 {
                                    map.set_tile(nx, ny, Tile::Wall);
                                    wall_count += 1;
                                    placed_adjacent += 1;

                                    // Assurez-vous d'avoir un groupe de 3 murs connectés
                                    if placed_adjacent == 2 && wall_count >= 3 {
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


fn save_score(score: usize) {
    let file_path = "scores.txt";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .expect("Impossible d'ouvrir le fichier des scores.");
    writeln!(file, "{}", score).expect("Impossible d'écrire dans le fichier des scores.");
}

fn display_last_scores() {
    let file_path = "scores.txt";
    let file = OpenOptions::new()
        .read(true)
        .open(file_path)
        .expect("Impossible de lire le fichier des scores.");

    let scores: Vec<usize> = io::BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok()?.parse().ok())
        .collect();

    println!("Les 5 derniers scores :");
    for score in scores.iter().rev().take(5) {
        println!("{}", score);
    }
}