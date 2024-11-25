use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};

// Fonction pour sauvegarder un score
pub fn save_score(score: usize) {
    let file_path = "scores.txt";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .expect("Impossible d'ouvrir le fichier des scores.");
    writeln!(file, "{}", score).expect("Impossible d'écrire dans le fichier des scores.");
}

// Fonction pour afficher les 5 derniers scores
pub fn display_last_scores() {
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
