mod functions;
mod utils;


use crate::functions::{part1, part2};
use crate::utils::read_data_from_url;

#[cfg(test)] mod tests;

fn main() {
    let data = &read_data_from_url("https://adventofcode.com/2021/day/4/input");
    let resultado_part1 = part1(data);
    println!("Resultado parte 1: {}", resultado_part1);
    let resultado_part2 = part2(data);
    println!("Resultado parte 2: {}", resultado_part2);
}
