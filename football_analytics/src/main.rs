mod player;
mod team;
mod analytics;
mod match_data;

use crate::analytics::FootballAnalytics;
use crate::match_data::MatchData;
use crate::player::{Player, Position};
use crate::team::Team;

fn main() {
    println!("⚽ Football Analytics System");
    println!("============================\n");

    // Create sample teams and players
    let mut analytics = FootballAnalytics::new();
    
    // Create Barcelona squad
    let mut barcelona = Team::new("FC Barcelona".to_string(), "Barcelona".to_string());
    barcelona.add_player(Player::new("Lionel Messi".to_string(), 10, Position::Forward, 91));
    barcelona.add_player(Player::new("Gerard Pique".to_string(), 3, Position::Defender, 85));
    barcelona.add_player(Player::new("Sergio Busquets".to_string(), 5, Position::Midfielder, 88));
    barcelona.add_player(Player::new("Marc-Andre ter Stegen".to_string(), 1, Position::Goalkeeper, 89));
    
    // Create Real Madrid squad
    let mut real_madrid = Team::new("Real Madrid".to_string(), "Madrid".to_string());
    real_madrid.add_player(Player::new("Karim Benzema".to_string(), 9, Position::Forward, 90));
    real_madrid.add_player(Player::new("Sergio Ramos".to_string(), 4, Position::Defender, 89));
    real_madrid.add_player(Player::new("Luka Modric".to_string(), 10, Position::Midfielder, 87));
    real_madrid.add_player(Player::new("Thibaut Courtois".to_string(), 1, Position::Goalkeeper, 88));
    
    // Add sample match data
    analytics.add_match(MatchData::new(
        "FC Barcelona".to_string(),
        "Real Madrid".to_string(),
        3, 2,
        vec!["Messi".to_string(), "Busquets".to_string(), "Pique".to_string()],
        vec!["Benzema".to_string(), "Ramos".to_string()],
    ));
    
    analytics.add_match(MatchData::new(
        "Real Madrid".to_string(),
        "FC Barcelona".to_string(),
        1, 2,
        vec!["Modric".to_string()],
        vec!["Messi".to_string(), "Busquets".to_string()],
    ));
    
    analytics.add_match(MatchData::new(
        "FC Barcelona".to_string(),
        "Atletico Madrid".to_string(),
        4, 0,
        vec!["Messi".to_string(), "Messi".to_string(), "Busquets".to_string(), "Pique".to_string()],
        vec![],
    ));

    // Perform analytics
    println!("📊 Team Analysis:");
    println!("Barcelona - Average Rating: {:.1}", barcelona.average_rating());
    println!("Real Madrid - Average Rating: {:.1}", real_madrid.average_rating());
    
    println!("\n🏆 Top Scorers:");
    let top_scorers = analytics.get_top_scorers(3);
    for (i, (player, goals)) in top_scorers.iter().enumerate() {
        println!("{}. {} - {} goals", i + 1, player, goals);
    }
    
    println!("\n📈 Team Performance:");
    let team_stats = analytics.get_team_stats();
    for (team, stats) in team_stats {
        println!("{}: {} games, {} goals for, {} goals against", 
                 team, stats.0, stats.1, stats.2);
    }
    
    println!("\n🎯 Head-to-Head Analysis:");
    if let Some((team1_wins, team2_wins, draws)) = analytics.head_to_head("FC Barcelona", "Real Madrid") {
        println!("Barcelona vs Real Madrid: {}-{}-{} (W-L-D)", team1_wins, team2_wins, draws);
    }
}