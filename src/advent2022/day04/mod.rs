use std::str::FromStr;
use colour::{dark_green_ln};
use lab_advent_of_code::utils;
use lab_advent_of_code::utils::{input_path, read_input};

const NAME: &str = "Day 4: Camp Cleanup";
const IN_FILE: &str = "advent2022/day04/in";

/*
 * --- Day 4: Camp Cleanup ---
 * https://adventofcode.com/2022/day/4
 *
 * Full puzzle input is not in repo, please download it
 */
pub fn main() {
    utils::log_setup(NAME, IN_FILE);

    let count = read_input(input_path(IN_FILE))
        .iter()
        .map(|it| it.parse::<Pair>().unwrap().overlap())
        .filter(|it| *it)
        .count();

    dark_green_ln!("Result: {:?}", count);
}

#[derive(Debug, Clone, Copy)]
struct Pair {
    section1: Section,
    section2: Section,
}

impl Pair {
    fn overlap(&self) -> bool {
        self.section1.overlap(&self.section2) || self.section2.overlap(&self.section1)
    }
}

impl FromStr for Pair {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut splits = line.split(",").map(|it| it.trim().parse::<Section>().unwrap());
        Ok(Pair {
            section1: splits.next().unwrap(),
            section2: splits.next().unwrap(),
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Section {
    start: i32,
    stop: i32,
}

impl Section {
    fn overlap(&self, other: &Section) -> bool {
        (self.start <= other.start && self.stop >= other.start)
            || (self.start <= other.stop && self.stop >= other.stop)
    }
}

impl FromStr for Section {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split("-").map(|it| it.trim().parse::<i32>().unwrap());
        Ok(Section {
            start: splits.next().unwrap(),
            stop: splits.next().unwrap(),
        })
    }
}
