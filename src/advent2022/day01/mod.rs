use itertools::Itertools;
use colour::dark_green_ln;

use lab_advent_of_code::utils;
use lab_advent_of_code::utils::read_as_str;


const NAME: &str = "Day 1: Calorie Counting";
const IN_FILE: &str = "advent2022/day01/in-test";

/*
 * --- Day 1: Calorie Counting ---
 * https://adventofcode.com/2022/day/1
 *
 * Full puzzle input is not in repo, please download it
 */
fn main() {
    utils::log_setup(NAME, IN_FILE);

    let input = read_as_str(utils::input_path(IN_FILE));

    let elves: i32 = input
        .split("\n\n")
        // another solution is to `filter_map( ... it.parse::<i32>().ok() )`
        .map(|elf| { elf.lines().map(|it| it.parse::<i32>().unwrap()).sum::<i32>() })
        .sorted()
        .rev()
        .take(3)
        .sum();

    dark_green_ln!("Result: {:?}", elves);
}
