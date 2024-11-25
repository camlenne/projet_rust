use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Créer une variable partagée avec Mutex pour une gestion sûre de la concurrence.
    let counter = Arc::new(Mutex::new(0));

    // Cloner l'Arc pour le passer dans le thread secondaire.
    let counter_clone = Arc::clone(&counter);

    // Créer un thread secondaire.
    let handle = thread::spawn(move || {
        for _ in 0..5 {
            // Verrouiller l'accès à la variable partagée.
            let mut num = counter_clone.lock().unwrap();
            *num += 1; // Modifier la variable partagée.
            println!("Thread secondaire: {}", num);
            thread::sleep(Duration::from_millis(100)); // Attendre un peu avant la prochaine itération.
        }
    });

    // Le thread principal fait également des itérations.
    for _ in 0..5 {
        // Verrouiller l'accès à la variable partagée.
        let mut num = counter.lock().unwrap();
        *num += 1; // Modifier la variable partagée.
        println!("Thread principal: {}", num);
        thread::sleep(Duration::from_millis(150)); // Attendre avant la prochaine itération.
    }

    // Attendre que le thread secondaire termine.
    handle.join().unwrap();

    // Afficher la valeur finale de la variable partagée.
    let final_value = *counter.lock().unwrap();
    println!("Valeur finale du compteur: {}", final_value);
}
