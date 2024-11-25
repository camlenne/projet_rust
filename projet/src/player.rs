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
    pub fn remove_life(&mut self, points_removed: i32) {
        if self.health > points_removed{
            self.health = self.health - points_removed;
        }
        else{
            self.health = 0;
        }
    }
    pub fn gain_life(&mut self, points_gained: i32) {
        if self.health >= 100{
            self.health = 100;
        }else{
            self.health = self.health + points_gained;
        }
    }
}
