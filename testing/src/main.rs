use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};


fn main() {
    loop{
        let mut input = read_input();
        if input == 'x' {
            println!("Au revoir !");
            break;
        }
        println!("{}",input);
    }
    
}
// Lit l'entrée utilisateur
fn read_input() -> char {
    enable_raw_mode().unwrap();
    let result = loop {
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char(c) => break c,
                    KeyCode::Esc => break 'x',
                    _ => {}
                }
            }
        }
    };
    disable_raw_mode().unwrap();
    result
}

