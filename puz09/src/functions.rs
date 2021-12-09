use std::convert::TryInto;

use log::debug;


pub fn part1(data: &str) -> u32{
    let width = data.lines().next().unwrap().chars().count();
    let height = data.lines().count();
    debug!("widht:{}, height: {}", width, height);
    let mut dots = vec![vec![0; height]; width];
    for (y, line) in data.lines().enumerate(){
        for (x, achar) in line.chars().enumerate(){
            if x < width && y < height{
                dots[x][y] = achar.to_digit(10).unwrap();
            }
        }
    }
    let mut risk_level = 0;
    for x in 0..width{
        for y in 0..height{
            if (x == 0 || dots[x][y] < dots[x-1][y]) &&
                (x == width - 1 || dots[x][y] < dots[x+1][y]) &&
                (y == 0 || dots[x][y] < dots[x][y-1]) &&
                (y == height - 1 || dots[x][y] < dots[x][y+1]) {
                    debug!("({},{}) => {}", x, y, dots[x][y]);
                    risk_level += dots[x][y] + 1;
            }
        }
    }
    risk_level
}

pub fn part2(data: &str) -> i32 {
    1
}
