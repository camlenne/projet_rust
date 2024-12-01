pub struct Player {
    pub x: usize,         // Position du joueur sur l'axe X (horizontal)
    pub y: usize,         // Position du joueur sur l'axe Y (vertical)
    pub name: String,     // Nom du joueur
    pub health: i32,      // Points de vie du joueur
    pub emoji: char,      // Symbole représentant le joueur (par exemple, un emoji)
}

impl Player {
    // Méthode pour créer un nouveau joueur avec les paramètres spécifiés
    pub fn new(name: &str, x: usize, y: usize, health: i32, emoji: char) -> Self {
        Player { 
            x, 
            y, 
            name: name.to_string(),  
            health,
            emoji,
        }
    }
    
    // Déplace le joueur vers le haut (réduit la coordonnée Y si possible)
    pub fn move_up(&mut self) {
        if self.y > 0 { 
            self.y -= 1;  
        }
    }

    // Déplace le joueur vers le bas (augmente la coordonnée Y si possible)
    pub fn move_down(&mut self, map_height: usize) {
        if self.y < map_height - 1 {  
            self.y += 1;  
        }
    }

    // Déplace le joueur vers la gauche (réduit la coordonnée X si possible)
    pub fn move_left(&mut self) {
        if self.x > 0 {  
            self.x -= 1;  
        }
    }

    // Déplace le joueur vers la droite (augmente la coordonnée X si possible)
    pub fn move_right(&mut self, map_width: usize) {
        if self.x < map_width - 1 {  
            self.x += 1;  
        }
    }

    // Réduit les points de vie du joueur
    pub fn remove_life(&mut self, points_removed: i32) {
        if self.health > points_removed {  
            self.health -= points_removed; 
        } else {
            self.health = 0;  
        }
    }

    // Ajoute des points de vie au joueur
    pub fn gain_life(&mut self, points_gained: i32) {
        if self.health >= 100 {  
            self.health = 100;
        } else {
            self.health += points_gained;  
        }
    }
}
