mod functions;
mod utils;

use crate::functions::{part1, part2};
use crate::utils::read_data_from_url;

#[cfg(test)] mod tests;

fn main() {
    let data = &read_data_from_url("https://adventofcode.com/2021/day/3/input");
    part1(data);
    part2(data);
}


