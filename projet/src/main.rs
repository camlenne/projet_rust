use std::io::{self, Write};

mod game;
use game::{Map, Tile,Player}; // Importation des éléments de game.rs


fn main() {
    let mut map = Map::new(10, 10);

    let mut players = vec![
        Player::new("Joueur1", 0, 0, 100),
        Player::new("Joueur2", 1, 0, 100),
    ];

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
        // Afficher la carte avec les positions de tous les joueurs
        map.display(&players);

        // Demander aux joueurs de se déplacer
        for player in &mut players {
            print!("{} (Déplacez-vous Z: Haut, Q: Gauche, S: Bas, D: Droite) : ", player.name);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Erreur de lecture");

            match input.trim().to_lowercase().as_str() {
                "z" => player.move_up(),
                "s" => player.move_down(map.height),
                "q" => player.move_left(),
                "d" => player.move_right(map.width),
                _ => {
                    println!("Commande invalide. Essayez à nouveau.");
                    continue;
                }
            }
        }

        // Vérifier si un joueur atteint l'objectif
        for player in &players {
            if player.x == 2 && player.y == 2 {
                println!("{} a atteint l'objectif !", player.name);
                //casser la loop
            }
            else{
                println!("{} est a la position {}:{}", player.name,player.x,player.y);
            }
        }
    }
}