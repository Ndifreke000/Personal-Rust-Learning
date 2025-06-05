use crate::player::{Player, Position};

#[derive(Debug)]
pub struct Team {
    pub name: String,
    pub city: String,
    pub players: Vec<Player>,    // Vector storing list of players
}

impl Team {
    pub fn new(name: String, city: String) -> Self {
        Team {
            name,
            city,
            players: Vec::new(),     // Initialize empty vector
        }
    }
    
    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }
    
    pub fn remove_player(&mut self, jersey_number: u8) -> Option<Player> {
        if let Some(pos) = self.players.iter().position(|p| p.jersey_number == jersey_number) {
            Some(self.players.remove(pos))
        } else {
            None
        }
    }
    
    pub fn get_player_by_number(&self, jersey_number: u8) -> Option<&Player> {
        self.players.iter().find(|p| p.jersey_number == jersey_number)
    }
    
    pub fn get_players_by_position(&self, position: Position) -> Vec<&Player> {
        self.players.iter().filter(|p| p.position == position).collect()
    }
    
    pub fn squad_size(&self) -> usize {
        self.players.len()
    }
    
    pub fn average_rating(&self) -> f64 {
        if self.players.is_empty() {
            0.0
        } else {
            let total: u32 = self.players.iter().map(|p| p.rating as u32).sum();
            total as f64 / self.players.len() as f64
        }
    }
    
    pub fn get_formation_strength(&self) -> (usize, usize, usize, usize) {
        let gk = self.players.iter().filter(|p| p.position == Position::Goalkeeper).count();
        let def = self.players.iter().filter(|p| p.position == Position::Defender).count();
        let mid = self.players.iter().filter(|p| p.position == Position::Midfielder).count();
        let fwd = self.players.iter().filter(|p| p.position == Position::Forward).count();
        (gk, def, mid, fwd)
    }
    
    pub fn list_squad(&self) -> Vec<String> {
        self.players.iter().map(|p| p.display_info()).collect()
    }
}