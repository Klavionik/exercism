use std::collections::HashMap;
use std::fmt::Display;

const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";

#[derive(Debug)]
enum Result {
    Win,
    Loss,
    Draw,
}

impl From<&str> for Result {
    fn from(value: &str) -> Self {
        match value {
            "win" => Result::Win,
            "loss" => Result::Loss,
            "draw" => Result::Draw,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct MatchResult {
    first_team: String,
    second_team: String,
    result: Result,
}

impl MatchResult {
    fn new(first_team: &str, second_team: &str, result: Result) -> Self {
        Self {
            first_team: first_team.to_string(),
            second_team: second_team.to_string(),
            result,
        }
    }
}

impl From<&str> for MatchResult {
    fn from(value: &str) -> Self {
        let parsed = value.split(";").collect::<Vec<_>>();
        Self::new(parsed[0], parsed[1], Result::from(parsed[2]))
    }
}

#[derive(Default)]
struct TeamStats {
    total_matches: i32,
    wins: i32,
    ties: i32,
    losses: i32,
    points: i32,
}

#[derive(Default)]
struct Stats {
    data: HashMap<String, TeamStats>,
}

impl Stats {
    fn add_match(&mut self, match_result: MatchResult) {
        let mut first_team = self
            .data
            .remove(&match_result.first_team)
            .unwrap_or_default();

        let mut second_team = self
            .data
            .remove(&match_result.second_team)
            .unwrap_or_default();

        first_team.total_matches += 1;
        second_team.total_matches += 1;

        match match_result.result {
            Result::Win => {
                first_team.wins += 1;
                first_team.points += 3;

                second_team.losses += 1;
            }
            Result::Loss => {
                first_team.losses += 1;

                second_team.wins += 1;
                second_team.points += 3;
            }
            Result::Draw => {
                first_team.ties += 1;
                first_team.points += 1;

                second_team.ties += 1;
                second_team.points += 1;
            }
        }

        self.data.insert(match_result.first_team, first_team);
        self.data.insert(match_result.second_team, second_team);
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut data = self.data.iter().collect::<Vec<_>>();
        data.sort_by(|(a_team, a_stats), (b_team, b_stats)| {
            b_stats
                .points
                .cmp(&a_stats.points)
                .then_with(|| a_team.cmp(b_team))
        });
        let mut rows = vec![String::from(HEADER)];

        for (team, stats) in data {
            let row = format!(
                "{:30} |  {} |  {} |  {} |  {} |{:3}",
                team, stats.total_matches, stats.wins, stats.ties, stats.losses, stats.points
            );
            rows.push(row)
        }

        write!(f, "{}", rows.join("\n"))
    }
}

pub fn tally(match_results: &str) -> String {
    let mut stats = Stats::default();

    for match_result in match_results.lines() {
        let match_result = MatchResult::from(match_result);
        stats.add_match(match_result)
    }

    stats.to_string()
}