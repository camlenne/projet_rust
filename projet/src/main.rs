use std::io::{self, Write};  // Utilisé pour la gestion de l'entrée/sortie
use std::sync::{Arc, Mutex, Condvar};  // Utilisé pour gérer la synchronisation entre threads
use std::thread;  
use std::time::Duration;  
use rand::Rng;  
use crossterm::event::{self, Event, KeyCode};  // Pour capturer les événements clavier
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};  // Pour activer/désactiver le mode brut du terminal

mod player;
mod map;
mod score;
use score::{save_score, display_last_scores};  
use player::Player;  
use map::{check_obj, Map, find_a_place};

fn main() {
    let size_map_x: i32 = 30;  // Largeur de la carte
    let size_map_y: i32 = 15;  // Hauteur de la carte
    let mut map = Map::new(30, 15);  // Création d'une nouvelle carte

    // Synchronisation entre threads avec Mutex et Condvar
    let turn = Arc::new((Mutex::new(0), Condvar::new()));  // 0 pour le thread principal, 1 pour le thread secondaire
    let turn_clone = Arc::clone(&turn);
    let score = Arc::new(Mutex::new(0));  // Initialisation du score à 0
    let score_clone = Arc::clone(&score);

    // Trouver un endroit sur la carte pour placer un joueur
    let place = find_a_place(&mut map, size_map_x, size_map_y);
    let first_x_dino = place.0;
    let first_y_dino = place.1;

    // Création de la liste des joueurs
    let players = Arc::new(Mutex::new(vec![
        Player::new("Humain", 0, 0, 100, '🦖'),  // Le joueur humain
        Player::new("Dino", first_x_dino, first_y_dino, 100, '🦕'),  // Le joueur dino
    ]));

    // Cloner la référence partagée pour le thread secondaire
    let players_thread = Arc::clone(&players);

    // Thread secondaire qui gère le mouvement du "dino" (joueur CPU)
    thread::spawn(move || {
        loop {
            let (lock, cvar) = &*turn_clone;
            let mut turn_num = lock.lock().unwrap();

            // Attente que ce soit au tour du thread secondaire (turn_num doit être 1)
            while *turn_num != 1 {
                turn_num = cvar.wait(turn_num).unwrap();
            }

            // Le thread secondaire exécute son code
            let scoring = score_clone.lock().unwrap();
            if *scoring % 10 == 0 {
                let mut players_lock = players_thread.lock().unwrap();
                players_lock.push(Player::new("Nouveau", 2, 2, 100, '🦕'));  // Ajouter un nouveau joueur
            }

            let mut players = players_thread.lock().unwrap();
            // Le dino se déplace de manière aléatoire
            for i in 1..players.len() {
                let dino = &mut players[i];
                let num = rand::thread_rng().gen_range(1..5);  // Nombre aléatoire pour déterminer le mouvement
                match num {
                    1 => dino.move_up(),
                    2 => dino.move_down(map.height),
                    3 => dino.move_left(),
                    4 => dino.move_right(map.width),
                    _ => {
                        println!("Commande invalide. Essayez à nouveau.");
                        continue;
                    }
                }
            }
            *turn_num = 0;  // Passer la main au thread principal
            cvar.notify_one();  // Notifier le thread principal
            thread::sleep(Duration::from_millis(100));  // Attendre un peu avant la prochaine itération
        }
    });

    // Boucle principale du jeu
    loop {
        let mut stdout = io::stdout();
        let (lock, cvar) = &*turn;
        let mut turn_num = lock.lock().unwrap();

        // Attente que ce soit au tour du thread principal (turn_num doit être 0)
        while *turn_num != 0 {
            turn_num = cvar.wait(turn_num).unwrap();
        }

        // Afficher la carte et les joueurs
        let mut scoring = score.lock().unwrap();
        {
            let players_snapshot = players.lock().unwrap();
            map.display(&*players_snapshot);
        }

        let mut players = players.lock().unwrap();
        let humain = &mut players[0];  // Référence au joueur humain

        // Affichage des informations et demande de mouvement
        println!("{} (Déplacez-vous Z: Haut, Q: Gauche, S: Bas, D: Droite, X: Quitter) score ({}), points de vie ({})", humain.name, *scoring, humain.health);
        let input = read_input();  // Lire l'entrée du joueur

        // Gestion du mouvement du joueur humain
        match input {
            'z' => {
                if humain.y > 0 && check_obj(&mut map, humain.x, humain.y - 1, humain) {
                    humain.move_up();
                }
            }
            's' => {
                if check_obj(&mut map, humain.x, humain.y + 1, humain) {
                    humain.move_down(map.height);
                }
            }
            'q' => {
                if humain.x > 0 && check_obj(&mut map, humain.x - 1, humain.y, humain) {
                    humain.move_left();
                }
            }
            'd' => {
                if check_obj(&mut map, humain.x + 1, humain.y, humain) {
                    humain.move_right(map.width);
                }
            }
            'x' => {
                // Sauvegarder le score et afficher les derniers scores si le joueur quitte
                save_score(*scoring);
                display_last_scores();
                break;
            }
            _ => {
                println!("Commande invalide. Essayez à nouveau.");
                continue;
            }
        }

        write!(stdout, "{}", termion::clear::All).unwrap();  // Effacer l'écran
        *scoring += 1;  // Incrémenter le score

        // Vérification des objets sur la case du joueur
        if map.get_tile(humain.x, humain.y).is_some() {
            humain.remove_life(2);  // Perdre de la vie si la case contient un objet
        } else {
            humain.remove_life(20);  // Perdre plus de vie si la case est vide
        }

        // Vérification si le joueur a atteint l'objectif
        if humain.x == map.width - 1 && humain.y == map.height - 1 {
            println!("Félicitations ! Vous avez atteint l'objectif en {} actions.", *scoring);
            save_score(*scoring);
            display_last_scores();
            break;
        }

        // Vérification si le joueur est mort
        if humain.health <= 0 {
            println!("Vous n'avez pas atteint l'objectif, vous êtes décédé");
            save_score(*scoring);
            display_last_scores();
            break;
        }

        *turn_num = 1;  // Passer la main au thread secondaire
        cvar.notify_one();  // Notifier le thread secondaire
        thread::sleep(Duration::from_millis(150));  // Attendre avant la prochaine itération
    }
}

// Fonction pour lire l'entrée utilisateur
fn read_input() -> char {
    enable_raw_mode().unwrap();  // Activer le mode brut pour une saisie plus réactive
    let result = loop {
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char(c) => break c,  // Retourner la touche pressée
                    KeyCode::Esc => break 'x',  // Quitter si Échap est pressé
                    _ => {}
                }
            }
        }
    };
    disable_raw_mode().unwrap();  // Désactiver le mode brut après avoir lu l'entrée
    result
}
