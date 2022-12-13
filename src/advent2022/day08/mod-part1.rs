use colour::{dark_green_ln};
use grid::Grid;
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

    let grid = Grid::from_vec(input.iter().flatten().collect(), size);

    let mut count = 0;

    for col in 0..size {
        for row in 0..size {
            if is_visible(col, row, &grid) {
                count += 1;
            }
        }
    }

    dark_green_ln!("Result: {:?}", count);
}

fn is_visible(col: usize, row: usize, grid: &Grid<&i32>) -> bool {
    let tree = grid[row][col];
    let size = grid[col].len();

    if col == 0 || row == 0 || col == size - 1 || row == size - 1 {
        return true;
    }

    let is_visible = tree > &max_col_ray(col, 0, row, grid)
        || tree > &max_col_ray(col, row + 1, size, grid)
        || tree > &max_row_ray(row, 0, col, grid)
        || tree > &max_row_ray(row, col + 1, size, grid);

    is_visible
}

fn max_col_ray(col: usize, low: usize, up: usize, grid: &Grid<&i32>) -> i32 {
    ***grid
        .iter_col(col)
        .collect::<Vec<&&i32>>()[low..up]
        .iter()
        .max()
        .unwrap_or(&&&-1)
}

fn max_row_ray(row: usize, low: usize, up: usize, grid: &Grid<&i32>) -> i32 {
    ***grid
        .iter_row(row)
        .collect::<Vec<&&i32>>()[low..up]
        .iter()
        .max()
        .unwrap_or(&&&-1)
}

fn parse_input() -> Vec<Vec<i32>> {
    read_input(input_path(IN_FILE))
        .iter()
        .map(|line| line.chars().map(|it| it.to_digit(10).unwrap() as i32).collect())
        .collect()
}
