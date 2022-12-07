use std::borrow::Borrow;
use std::str::FromStr;
use colour::{blue_ln, dark_green_ln};
use lab_advent_of_code::utils;

const NAME: &str = "Day 2: Dive!";
const IN_FILE: &str = "advent2021/day02/in-test";

/*
 * --- Day 2: Dive! ---
 * https://adventofcode.com/2021/day/2
 *
 * Full puzzle input is not in repo, please download it
 */
pub fn main() {
    log_setup();

    let start = Position::default();
    let end: Position = read_input()
        .iter()
        .fold(start, |pos, course| {
            match course.direction {
                Direction::Forward => {
                    Position {
                        horizontal: pos.horizontal + course.steps,
                        depth: pos.depth + pos.aim * course.steps,
                        aim: pos.aim,
                    }
                }
                Direction::Down => {
                    Position {
                        horizontal: pos.horizontal,
                        depth: pos.depth,
                        aim: pos.aim + course.steps,
                    }
                }
                Direction::Up => {
                    Position {
                        horizontal: pos.horizontal,
                        depth: pos.depth,
                        aim: pos.aim - course.steps,
                    }
                }
            }
        });

    dark_green_ln!("Result: {:?}", end.horizontal * end.depth);
}

struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Default for Position {
    fn default() -> Position {
        Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }
}

enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input.to_lowercase().borrow() {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

struct Course {
    direction: Direction,
    steps: i32,
}

impl FromStr for Course {
    type Err = ();

    fn from_str(input: &str) -> Result<Course, Self::Err> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        Ok(
            Course {
                direction: Direction::from_str(parts[0]).expect("Direction invalid"),
                steps: utils::to_int(&parts[1].to_string()),
            }
        )
    }
}

fn read_input() -> Vec<Course> {
    utils::read_input(utils::input_path(IN_FILE))
        .iter()
        .map(|line| Course::from_str(line).expect("Cannot parse Course"))
        .collect()
}

fn log_setup() {
    blue_ln!("=======================");
    blue_ln!("{}", NAME);
    blue_ln!("=======================");
    println!("  input file:\t{}", IN_FILE);
}
