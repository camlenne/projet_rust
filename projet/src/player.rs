pub struct Player{
    pub x: usize,
    pub y: usize,
    pub name: String,
    pub health: i32,
    pub emoji: char,
}

impl Player {
    pub fn new(name: &str,x: usize,y: usize,health: i32,emoji : char) -> Self {
        Player { 
            x, 
            y, 
            name: name.to_string(), 
            health,
            emoji,
        }
    }
    pub fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    pub fn move_down(&mut self, map_height: usize) {
        if self.y < map_height - 1 {
            self.y += 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self, map_width: usize) {
        if self.x < map_width - 1 {
            self.x += 1;
        }
    }
}
