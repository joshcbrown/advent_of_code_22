use crate::Solution;
use RPS::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn new(symbol: char) -> Self {
        match symbol {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => panic!("attempt to create RPS out of invalid symbol: {symbol}"),
        }
    }
    fn value(&self) -> i32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn wins_against(&self) -> Self {
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn loses_against(&self) -> Self {
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }
}

struct Game {
    them: RPS,
    me: RPS,
}

impl Game {
    fn outcome(&self) -> i32 {
        if self.me == self.them {
            return 3;
        } else if self.me.wins_against() == self.them {
            return 6;
        }
        0
    }

    fn new(line: &str) -> Self {
        let chars: String = line.split(' ').collect();
        if let [a, b] = chars.as_bytes() {
            Self {
                them: RPS::new(*a as char),
                me: RPS::new(*b as char),
            }
        } else {
            panic!("line too long: {line}")
        }
    }
}

pub(crate) struct Day2 {
    games: Vec<Game>,
}

impl Solution for Day2 {
    fn new(content: String) -> Self {
        Self {
            games: content.lines().map(|line| Game::new(line)).collect(),
        }
    }

    fn solve1(&self) -> String {
        self.games
            .iter()
            .map(|game| game.outcome() + game.me.value())
            .sum::<i32>()
            .to_string()
    }

    fn solve2(&self) -> String {
        // HACK: twist for this was that X, Y, and Z don't actually mean
        // what we assumed they meant. going to use the prior (incorrect)
        // assumption in the following map:
        let new_games = self
            .games
            .iter()
            .map(|Game { them, me }| match me {
                Rock => Game {
                    them: *them,
                    me: them.wins_against(),
                },
                Paper => Game {
                    them: *them,
                    me: *them,
                },
                Scissors => Game {
                    them: *them,
                    me: them.loses_against(),
                },
            })
            .collect();
        Self { games: new_games }.solve1()
    }
}
