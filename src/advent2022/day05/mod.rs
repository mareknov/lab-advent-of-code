use std::str::FromStr;
use colour::{dark_green_ln};
use regex::{Captures, Regex};
use lab_advent_of_code::utils;
use lab_advent_of_code::utils::{input_path, read_as_str};

const NAME: &str = "Day 5: Supply Stacks";
const IN_FILE: &str = "advent2022/day05/in-test";

/*
 * --- Day 5: Supply Stacks ---
 * https://adventofcode.com/2022/day/5
 *
 * Full puzzle input is not in repo, please download it
 */
pub fn main() {
    utils::log_setup(NAME, IN_FILE);

    let input = read_as_str(input_path(IN_FILE));
    let mut splits = input.split("\n\n");
    let lines_cranes = splits.next().unwrap();
    let lines_moves = splits.next().unwrap();

    let moves: Vec<Move> = lines_moves.lines()
        .map(|it| it.parse::<Move>().unwrap())
        .collect();

    let top = lines_cranes
        .parse::<Crates>()
        .unwrap()
        .process_moves(moves)
        .top_crates();

    dark_green_ln!("Result: {:?}", top);
}

#[derive(Debug)]
struct Crates {
    crates: Vec<Vec<char>>,
}

impl Crates {
    fn process_moves(self, moves: Vec<Move>) -> Crates {
        moves.iter().fold(self, |acc, it| acc.process_move(it))
    }

    fn process_move(self, mov: &Move) -> Crates {
        let mut v = self.crates.clone();

        let index_from = (mov.from - 1) as usize;
        let index_to = (mov.to - 1) as usize;

        let stack_from = &v[index_from];
        let stack_to = &v[index_to];

        let creates_to_move = &stack_from[stack_from.len() - mov.count as usize..];
        let new_stack_from = stack_from[..stack_from.len() - mov.count as usize].to_vec();
        let mut new_stack_to = stack_to.clone();

        new_stack_to.extend_from_slice(creates_to_move);

        v[index_from] = new_stack_from;
        v[index_to] = new_stack_to;

        Crates { crates: v }
    }

    fn top_crates(&self) -> String {
        self.crates.iter()
            .map(|it| it.last().unwrap_or(&' '))
            .collect()
    }
}

impl FromStr for Crates {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const CRANE_LEN: usize = 4;

        let mut lines = s.lines().rev();
        let count = lines.next().unwrap().trim().chars().last().unwrap().to_digit(10).unwrap();
        let mut crates: Vec<Vec<char>> = vec![vec![]; count as usize];

        lines.for_each(|line| {
            crates.iter_mut()
                .enumerate()
                .for_each(|(i, stack)| {
                    let c = line.as_bytes()[1 + i * CRANE_LEN] as char;
                    if !c.is_whitespace() {
                        stack.push(c as char);
                    }
                })
        });

        Ok(Crates { crates })
    }
}


#[derive(Debug, Clone, Copy)]
struct Move {
    count: i32,
    from: i32,
    to: i32,
}

impl Move {}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re: Regex = Regex::new(r"^move (?P<move>\d*) from (?P<from>\d*) to (?P<to>\d*)$").unwrap();

        fn to_num(caps: &Captures, name: &str) -> i32 {
            caps.name(name).map_or(0, |it| it.as_str().parse::<i32>().unwrap())
        }

        let caps = re.captures(s).unwrap();

        Ok(Move {
            count: to_num(&caps, "move"),
            from: to_num(&caps, "from"),
            to: to_num(&caps, "to"),
        })
    }
}
