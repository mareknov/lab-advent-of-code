use colour::dark_green_ln;
use lab_advent_of_code::utils;


const NAME: &str = "Day 1: Sonar Sweep";
const IN_FILE: &str = "advent2021/day01/in-test";
const SWEEP_WINDOW: usize = 3;

/*
 * --- Day 1: Sonar Sweep ---
 * https://adventofcode.com/2021/day/1
 *
 * Full puzzle input is not in repo, please download it
 */
fn main() {
    utils::log_setup(NAME, IN_FILE);

    let mut counter = 0;

    read_input()
        .windows(SWEEP_WINDOW)
        .map(|it| it.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .for_each(|it| { if it[0] < it[1] { counter += 1 } });

    dark_green_ln!("Result: {:?}", counter);
}

fn read_input() -> Vec<i32> {
    utils::read_input(utils::input_path(IN_FILE))
        .iter()
        .map(utils::to_int)
        .collect()
}
