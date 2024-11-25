use std::sync::{Arc, Mutex};
use crossterm::{event::{self, Event, KeyCode}, terminal, execute};
use std::io::{stdout, Write};

#[derive(Debug)]
struct Player {
    name: String,
    x: i32,
    y: i32,
    health: i32,
    icon: char,
}

impl Player {
    fn new(name: &str, x: i32, y: i32, health: i32, icon: char) -> Self {
        Self {
            name: name.to_string(),
            x,
            y,
            health,
            icon,
        }
    }
}

fn main() -> crossterm::Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen)?;

    // Liste des joueurs partagée
    let players = Arc::new(Mutex::new(vec![
        Player::new("Humain", 0, 0, 100, '🦖'),
        Player::new("Dino", 1, 0, 100, '🦕'),
    ]));

    println!("Appuyez sur 'N' pour ajouter un nouveau joueur. Échap pour quitter.");

    for c in std::iter::from_fn(|| {
        match event::read() {
            Ok(Event::Key(key_event)) => Some(key_event),
            _ => None,
        }
    }) {
        match c.code {
            KeyCode::Char('n') => {
                let mut players_lock = players.lock().unwrap(); // Obtenir un verrou
                players_lock.push(Player::new("Nouveau", 2, 2, 100, '🙂'));
                println!("\nNouveau joueur ajouté ! Liste actuelle :");
                for player in players_lock.iter() {
                    println!("{:?}", player);
                }
            }
            KeyCode::Esc => {
                println!("\nQuitter !");
                break;
            }
            _ => {
                print!("\rTouche pressée : {:?}\n", c.code);
                stdout.flush()?; // Forcer l'affichage immédiat
            }
        }
    }

    execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
