
#[derive(Debug, Clone)]
pub struct MatchData {
    pub home_team: String,
    pub away_team: String,
    pub home_score: u8,
    pub away_score: u8,
    pub home_scorers: Vec<String>,    // Vector of scorer names
    pub away_scorers: Vec<String>,    // Vector of scorer names
}

impl MatchData {
    pub fn new(
        home_team: String,
        away_team: String,
        home_score: u8,
        away_score: u8,
        home_scorers: Vec<String>,
        away_scorers: Vec<String>,
    ) -> Self {
        MatchData {
            home_team,
            away_team,
            home_score,
            away_score,
            home_scorers,
            away_scorers,
        }
    }
    
    pub fn get_winner(&self) -> Option<String> {
        if self.home_score > self.away_score {
            Some(self.home_team.clone())
        } else if self.away_score > self.home_score {
            Some(self.away_team.clone())
        } else {
            None // Draw
        }
    }
    
    pub fn is_draw(&self) -> bool {
        self.home_score == self.away_score
    }
    
    pub fn total_goals(&self) -> u8 {
        self.home_score + self.away_score
    }
    
    pub fn get_all_scorers(&self) -> Vec<String> {
        let mut all_scorers = Vec::new();
        all_scorers.extend(self.home_scorers.clone());
        all_scorers.extend(self.away_scorers.clone());
        all_scorers
    }
    
    pub fn match_summary(&self) -> String {
        format!("{} {} - {} {}", 
                self.home_team, self.home_score, 
                self.away_score, self.away_team)
    }
}