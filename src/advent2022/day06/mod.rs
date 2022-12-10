use std::collections::HashSet;
use colour::{dark_green_ln};
use lab_advent_of_code::utils;
use lab_advent_of_code::utils::{input_path, read_as_str};

const NAME: &str = "Day 6: Tuning Trouble";
const IN_FILE: &str = "advent2022/day06/in";

/*
 * --- Day 6: Tuning Trouble ---
 * https://adventofcode.com/2022/day/6
 *
 * Full puzzle input is not in repo, please download it
 */
pub fn main() {
    utils::log_setup(NAME, IN_FILE);

    const MARKER_SIZE: usize = 14;

    let input = read_as_str(input_path(IN_FILE))
        .chars()
        .collect::<Vec<char>>();

    let start_of_packet = input
        .windows(MARKER_SIZE)
        .enumerate()
        .find(|(_, signal)| {
            let unique = signal.iter().map(|it| it.clone()).collect::<HashSet<char>>();
            unique.len() == signal.len()
        })
        .unwrap().0 + MARKER_SIZE;

    dark_green_ln!("Result: {:?}", start_of_packet);
}
