use std::borrow::Borrow;
use std::str::FromStr;
use colour::{blue_ln, dark_green_ln};
use lab_advent_of_code::utils;

const NAME: &str = "Day 3: Binary Diagnostic!";
const IN_FILE: &str = "advent2021/day03/in-test";

/*
 * --- Day 3: Binary Diagnostic ---
 * https://adventofcode.com/2021
 *
 * Full puzzle input is not in repo, please download it
 */
pub fn main() {
    log_setup();

    let report = read_input();
    let gamma_rate = rate(report.borrow(), true).to_int();
    let epsilon_rate = rate(report.borrow(), false).to_int();

    println!("gamma rate: {:?}", gamma_rate);
    println!("epsilon rate: {:?}", epsilon_rate);
    println!("power consumption: {:?}", gamma_rate * epsilon_rate);

    let oxyxgen_rating = filter_report(report.borrow(), true).to_int();
    let scrubber_rating = filter_report(report.borrow(), false).to_int();
    let life_support = oxyxgen_rating * scrubber_rating;

    println!("oxygen rating: {:?}", oxyxgen_rating);
    println!("scrubber rating: {:?}", scrubber_rating);

    dark_green_ln!("Result: {:?}", life_support);
}

fn rate(report: &Vec<BinaryNum>, one_priority: bool) -> BinaryNum {
    let num_len = report[0].num.len();
    let mut gamma_rate: Vec<u8> = Vec::with_capacity(num_len);

    for i in 0..num_len {
        let mut sum: u32 = 0;
        for j in 0..report.len() {
            sum += report[j].num[i] as u32
        }
        gamma_rate.push(
            if one_priority {
                match sum * 2 {
                    it if it >= report.len() as u32 => 1,
                    _ => 0,
                }
            } else {
                match sum * 2 {
                    it if it >= report.len() as u32 => 0,
                    _ => 1,
                }
            }
        );
    }

    BinaryNum { num: gamma_rate }
}

fn filter_report(report: &Vec<BinaryNum>, one_priority: bool) -> BinaryNum {
    let num_len = report[0].num.len();
    let mut ratings: Vec<BinaryNum> = report.clone();
    for i in 0..num_len {
        let mut ratings_temp: Vec<BinaryNum> = Vec::with_capacity(ratings.len());
        let rate = rate(ratings.borrow(), one_priority);
        for j in 0..ratings.len() {
            if ratings[j].num[i] == rate.num[i] {
                ratings_temp.push(ratings[j].clone())
            }
        }
        ratings = ratings_temp;
        if ratings.len() == 1 {
            break;
        }
    }
    ratings
        .first()
        .expect("Must not be empty")
        .clone()
}

fn read_input() -> Vec<BinaryNum> {
    utils::read_input(utils::input_path(IN_FILE))
        .iter()
        .map(|line| BinaryNum::from_str(line).unwrap())
        .collect()
}

#[derive(Debug, Clone)]
struct BinaryNum {
    num: Vec<u8>,
}

impl FromStr for BinaryNum {
    type Err = ();

    fn from_str(input: &str) -> Result<BinaryNum, Self::Err> {
        let num = input
            .chars()
            .map(|it| if it == '1' { 1 } else { 0 })
            .collect();
        Ok(BinaryNum { num })
    }
}

impl BinaryNum {
    fn inverse(&self) -> BinaryNum {
        BinaryNum {
            num: self.num.iter().map(|it| if *it == 1 { 0 } else { 1 }).collect()
        }
    }
}

impl BinaryNum {
    fn to_int(&self) -> u32 {
        self.num
            .iter()
            .fold(0, |acc, &it| acc * 2 + it as u32)
    }
}

fn log_setup() {
    blue_ln!("=======================");
    blue_ln!("{}", NAME);
    blue_ln!("=======================");
    println!("  input file:\t{}", IN_FILE);
}
