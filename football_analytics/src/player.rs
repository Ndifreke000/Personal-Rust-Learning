
#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,           // UTF-8 encoded text
    pub jersey_number: u8,
    pub position: Position,
    pub rating: u8,             // Overall rating (0-100)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Position {
    Goalkeeper,
    Defender,
    Midfielder,
    Forward,
}

impl Player {
    pub fn new(name: String, jersey_number: u8, position: Position, rating: u8) -> Self {
        Player {
            name,
            jersey_number,
            position,
            rating,
        }
    }
    
    pub fn display_info(&self) -> String {
        format!("{} (#{}) - {:?} - Rating: {}", 
                self.name, self.jersey_number, self.position, self.rating)
    }
    
    // Check if player is in attacking position
    pub fn is_attacker(&self) -> bool {
        matches!(self.position, Position::Forward | Position::Midfielder)
    }
    
    // Check if player is defensive
    pub fn is_defender(&self) -> bool {
        matches!(self.position, Position::Defender | Position::Goalkeeper)
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Position::Goalkeeper => write!(f, "GK"),
            Position::Defender => write!(f, "DEF"),
            Position::Midfielder => write!(f, "MID"),
            Position::Forward => write!(f, "FWD"),
        }
    }
}