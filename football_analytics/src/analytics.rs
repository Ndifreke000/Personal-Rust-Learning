use std::collections::HashMap;
use crate::match_data::MatchData;

pub struct FootballAnalytics {
    matches: Vec<MatchData>,
}

impl FootballAnalytics {
    pub fn new() -> Self {
        FootballAnalytics {
            matches: Vec::new(),
        }
    }

    pub fn add_match(&mut self, match_data: MatchData) {
        self.matches.push(match_data);
    }

    pub fn get_top_scorers(&self, top_n: usize) -> Vec<(String, usize)> {
        let mut scorer_counts: HashMap<String, usize> = HashMap::new();

        for m in &self.matches {
            for scorer in &m.home_scorers {
                *scorer_counts.entry(scorer.clone()).or_insert(0) += 1;
            }
            for scorer in &m.away_scorers {
                *scorer_counts.entry(scorer.clone()).or_insert(0) += 1;
            }
        }

        let mut scorer_vec: Vec<(String, usize)> = scorer_counts.into_iter().collect();
        scorer_vec.sort_by(|a, b| b.1.cmp(&a.1));
        scorer_vec.into_iter().take(top_n).collect()
    }

    pub fn get_team_stats(&self) -> HashMap<String, (usize, usize, usize)> {
        // (games_played, goals_for, goals_against)
        let mut stats: HashMap<String, (usize, usize, usize)> = HashMap::new();

        for m in &self.matches {
            let home = stats.entry(m.home_team.clone()).or_insert((0, 0, 0));
            home.0 += 1;
            home.1 += m.home_score as usize;
            home.2 += m.away_score as usize;

            let away = stats.entry(m.away_team.clone()).or_insert((0, 0, 0));
            away.0 += 1;
            away.1 += m.away_score as usize;
            away.2 += m.home_score as usize;
        }

        stats
    }

    pub fn head_to_head(&self, team1: &str, team2: &str) -> Option<(usize, usize, usize)> {
        let mut team1_wins = 0;
        let mut team2_wins = 0;
        let mut draws = 0;

        for m in &self.matches {
            if (m.home_team == team1 && m.away_team == team2) || (m.home_team == team2 && m.away_team == team1) {
                if m.home_score > m.away_score {
                    if m.home_team == team1 {
                        team1_wins += 1;
                    } else {
                        team2_wins += 1;
                    }
                } else if m.home_score < m.away_score {
                    if m.away_team == team1 {
                        team1_wins += 1;
                    } else {
                        team2_wins += 1;
                    }
                } else {
                    draws += 1;
                }
            }
        }

        Some((team1_wins, team2_wins, draws))
    }
}




⚽ Football Analytics System
============================

📊 Team Analysis:
Barcelona - Average Rating: 88.2
Real Madrid - Average Rating: 88.5

🏆 Top Scorers:
1. Messi - 4 goals
2. Busquets - 3 goals
3. Pique - 2 goals

📈 Team Performance:
Real Madrid: 2 games, 3 goals for, 5 goals against
Atletico Madrid: 1 games, 0 goals for, 4 goals against
FC Barcelona: 3 games, 9 goals for, 3 goals against

🎯 Head-to-Head Analysis:
Barcelona vs Real Madrid: 2-0-0 (W-L-D)