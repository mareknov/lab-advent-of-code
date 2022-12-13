use colour::{dark_green_ln};
use grid::{Grid};
use itertools::{Either, FoldWhile, Itertools};
use lab_advent_of_code::utils;
use lab_advent_of_code::utils::{input_path, read_input};

const NAME: &str = "Day 8: Treetop Tree House";
const IN_FILE: &str = "advent2022/day08/in";

/*
 * --- Day 8: Treetop Tree House ---
 * https://adventofcode.com/2022/day/8
 *
 * Full puzzle input is not in repo, please download it
 */
pub fn main() {
    utils::log_setup(NAME, IN_FILE);

    let input: Vec<Vec<i32>> = parse_input();
    let size = input.len();

    let grid = Grid::from_vec(input.iter().flatten().map(|it| it.clone()).collect(), size);

    let mut max_scenic_score = 0;

    for col in 1..size - 1 {
        for row in 1..size - 1 {
            let score = scenic_score(col, row, &grid);
            if score > max_scenic_score {
                max_scenic_score = score;
            }
        }
    }

    dark_green_ln!("Result: {:?}", max_scenic_score);
}

fn scenic_score(col: usize, row: usize, grid: &Grid<i32>) -> usize {
    let tree = &grid[row][col];
    let size = grid[col].len();

    let count_see = count_see(tree, row, 0, grid.iter_col(col))
        * count_see(tree, row + 1, size, grid.iter_col(col))
        * count_see(tree, col, 0, grid.iter_row(row))
        * count_see(tree, col + 1, size, grid.iter_row(row));

    count_see.clone()
}

fn count_see<'a, I>(tree: &i32, start: usize, stop: usize, grid_iter: I) -> usize
    where I: Iterator<Item=&'a i32> {
    let is_revert = stop < start;
    let range = if is_revert { stop..start } else { start..stop };

    let a = &grid_iter.collect::<Vec<&i32>>()[range];

    let mut ray = if is_revert { Either::Left(a.iter().rev()) } else { Either::Right(a.iter()) };

    let count = ray
        .fold_while(0, |acc, it| {
            if it < &&&tree { FoldWhile::Continue(acc + 1) } else { FoldWhile::Done(acc + 1) }
        }).into_inner();

    count
}

fn parse_input() -> Vec<Vec<i32>> {
    read_input(input_path(IN_FILE))
        .iter()
        .map(|line| line.chars().map(|it| it.to_digit(10).unwrap() as i32).collect())
        .collect()
}
