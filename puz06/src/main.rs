mod functions;
mod utils;


use crate::functions::{part1, part2};
use crate::utils::read_data_from_url;
use simple_logger::SimpleLogger;

#[cfg(test)] mod tests;

fn main() {
    SimpleLogger::new().init().unwrap();

    let data = &read_data_from_url("https://adventofcode.com/2021/day/6/input");
    let ts = part1("3,4,3,1,2", 18);
    let resultado_part1 = part1(data, 80);
    println!("Resultado parte 1: {}", resultado_part1);
    let resultado_part2 = part2(data, 256);
    println!("Resultado parte 2: {}", resultado_part2);
}
