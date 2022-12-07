use std::borrow::Borrow;
use std::str::FromStr;
use colour::{dark_green_ln};
use lab_advent_of_code::utils;
use lab_advent_of_code::utils::read_input;

const NAME: &str = "Day 2: Dive!";
const IN_FILE: &str = "advent2022/day02/in-test";

/*
 * --- Day 2: Rock Paper Scissors ---
 * https://adventofcode.com/2022/day/2
 *
 * Full puzzle input is not in repo, please download it
 */
pub fn main() {
    utils::log_setup(NAME, IN_FILE);

    let score: u32 = read_input(utils::input_path(IN_FILE))
        .iter()
        .map(|it| it.parse::<Strategy>().unwrap().calc_score())
        .sum();

    dark_green_ln!("Result: {:?}", score);
}

#[derive(Debug, Clone)]
enum Game {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum GameResult {
    Loose,
    Draw,
    Win,
}

impl Game {
    fn to_score(self) -> u32 {
        match self {
            Game::Rock => 1,
            Game::Paper => 2,
            Game::Scissors => 3,
        }
    }
}

impl GameResult {
    fn to_score(self) -> u32 {
        match self {
            GameResult::Loose => 0,
            GameResult::Draw => 3,
            GameResult::Win => 6,
        }
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(input: &str) -> Result<Game, Self::Err> {
        match input.to_uppercase().borrow() {
            "A" => Ok(Game::Rock),
            "B" => Ok(Game::Paper),
            "C" => Ok(Game::Scissors),
            _ => Err(()),
        }
    }
}

impl FromStr for GameResult {
    type Err = ();

    fn from_str(input: &str) -> Result<GameResult, Self::Err> {
        match input.to_uppercase().borrow() {
            "X" => Ok(GameResult::Loose),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Strategy {
    opponent: Game,
    me: GameResult,
}

impl Strategy {
    fn calc_score(&self) -> u32 {
        let my_game = match self.opponent {
            Game::Rock => {
                match self.me {
                    GameResult::Loose => Game::Scissors,
                    GameResult::Draw => Game::Rock,
                    GameResult::Win => Game::Paper
                }
            }
            Game::Paper => {
                match self.me {
                    GameResult::Loose => Game::Rock,
                    GameResult::Draw => Game::Paper,
                    GameResult::Win => Game::Scissors
                }
            }
            Game::Scissors => {
                match self.me {
                    GameResult::Loose => Game::Paper,
                    GameResult::Draw => Game::Scissors,
                    GameResult::Win => Game::Rock
                }
            }
        };
        my_game.to_score() + self.me.to_score()
    }
}

impl FromStr for Strategy {
    type Err = ();

    fn from_str(input: &str) -> Result<Strategy, Self::Err> {
        let (opponent, me) = input.split_at(1);
        Ok(Strategy {
            opponent: opponent.trim().parse().unwrap(),
            me: me.trim().parse().unwrap(),
        })
    }
}
