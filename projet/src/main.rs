use std::io::{self, Write};
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;
use rand::Rng;

mod game;
mod map;
use game::Player; // Importation des éléments de game.rs
use map::Map;
fn main() {
    let turn = Arc::new((Mutex::new(0), Condvar::new())); // 0 pour le thread principal, 1 pour le thread secondaire
    let turn_clone = Arc::clone(&turn);

    let score = Arc::new(Mutex::new(0));
    let score_clone = Arc::clone(&score);

    let map = Map::new(10, 10);
    

    let players = Arc::new(Mutex::new(vec![
        Player::new("Humain", 0, 0, 100,'🦖'),
        Player::new("Dino", 1, 0, 100,'🦕'),
    ]));

    // Cloner les références partagées pour le thread
    let players_thread = Arc::clone(&players);

    let handle = thread::spawn(move || {
        loop{
            let (lock, cvar) = &*turn_clone;
            let mut turn_num = lock.lock().unwrap();

            // Attendre que ce soit à son tour (turn_num doit être 1 pour le thread secondaire).
            while *turn_num != 1 {
                turn_num = cvar.wait(turn_num).unwrap();
            }

            // Le thread secondaire peut maintenant s'exécuter.
            let mut scoring = score_clone.lock().unwrap();
            if *scoring > 10 {
                let mut players_lock = players_thread.lock().unwrap(); // Obtenir un verrou
                players_lock.push(Player::new("Nouveau", 2, 2, 100, '🦕')); // Ajouter un joueur
                println!("\nNouveau joueur ajouté ! Liste actuelle :");
                *scoring = 0;
            }
            *scoring = *scoring + 1;


            let mut players = players_thread.lock().unwrap();
            
            for i in 1..players.len(){
                //TODO préférez une itération


                let dino = &mut players[i];
                // Generate random number in the range [0, 99]
                let num = rand::thread_rng().gen_range(1..5);
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
            

            *turn_num = 0; // Passer la main au thread principal.
            cvar.notify_one(); // Notifier le thread principal.
            thread::sleep(Duration::from_millis(100)); // Attendre un peu avant la prochaine itération.
        }
    });

    loop {
        let (lock, cvar) = &*turn;
        let mut turn_num = lock.lock().unwrap();

        // Attendre que ce soit à son tour (turn_num doit être 0 pour le thread principal).
        while *turn_num != 0 {
            turn_num = cvar.wait(turn_num).unwrap();
        }
        
        let mut scoring = score.lock().unwrap();
        
        {
            // Emprunt immuable pour l'affichage
            let players_snapshot = players.lock().unwrap();
            map.display(&*players_snapshot);
        }
        let mut players = players.lock().unwrap();
        let humain = &mut players[0];
    
        // Demander aux joueurs de se déplacer
        print!("{} (Déplacez-vous Z: Haut, Q: Gauche, S: Bas, D: Droite) : ", humain.name);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erreur de lecture");

        match input.trim().to_lowercase().as_str() {
            "z" => humain.move_up(),
            "s" => humain.move_down(map.height),
            "q" => humain.move_left(),
            "d" => humain.move_right(map.width),
            _ => {
                println!("Commande invalide. Essayez à nouveau.");
                continue;
            }
        }
        *scoring +=1;
        
        *turn_num = 1; // Passer la main au thread secondaire.
        cvar.notify_one(); // Notifier le thread secondaire.
        thread::sleep(Duration::from_millis(150)); // Attendre un peu avant la prochaine itération.
    }
    
}