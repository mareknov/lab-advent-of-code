use colour::{dark_green_ln};
use lab_advent_of_code::utils;
use lab_advent_of_code::utils::read_input;

const NAME: &str = "Day 3: Rucksack Reorganization";
const IN_FILE: &str = "advent2022/day03/in";

/*
 * --- Day 3: Rucksack Reorganization ---
 * https://adventofcode.com/2022/day/3
 *
 * Full puzzle input is not in repo, please download it
 */
pub fn main() {
    utils::log_setup(NAME, IN_FILE);

    const ELVES_IN_GROUP: usize = 3;

    let score: i32 = read_input(utils::input_path(IN_FILE))
        .windows(ELVES_IN_GROUP)
        .step_by(ELVES_IN_GROUP)
        .map(|it| shared_items_sum(it))
        .sum();

    dark_green_ln!("Result: {:?}", score);
}

fn shared_items_sum(lines: &[String]) -> i32 {
    let (elf1, elf2, elf3) = (&lines[0], &lines[1], &lines[2]);

    elf1
        .chars()
        .fold(None, |mut acc, c| {
            if acc == None && elf2.contains(c) && elf3.contains(c) {
                acc = Some(c);
            }
            acc
        })
        .iter()
        .map(|c| to_priority(c))
        .sum()
}

fn to_priority(c: &char) -> i32 {
    let res = if c.is_uppercase() {
        *c as i32 - 'A' as i32 + 27
    } else {
        *c as i32 - 'a' as i32 + 1
    };
    println!("priority of {} is {}", c, res);

    res
}

