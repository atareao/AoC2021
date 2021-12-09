use std::{convert::TryInto, usize};
use log::debug;

pub fn part1(data: &str) -> u32{
    let field = get_field(data);
    let low_points = get_low_points(&field);
    let mut risk_level = 0;
    for low_point in low_points{
        risk_level += field[low_point.0][low_point.1] + 1;
    }
    risk_level
}

pub fn part2(data: &str) -> u32 {
    let mut size_of_basins: Vec<u32> = Vec::new();
    let field = get_field(data);
    let low_points = get_low_points(&field);
    for low_point in low_points{
        let mut basin: Vec<(usize, usize)> = Vec::new();
        add_to_basin(&field, &mut basin, low_point.0, low_point.1);
        debug!("Size of basin: {}", &basin.len());
        size_of_basins.push(basin.len().try_into().unwrap());
    }
    size_of_basins.sort();
    size_of_basins.reverse();
    size_of_basins[0] * size_of_basins[1] * size_of_basins[2]
}

fn add_to_basin(field: &Vec<Vec<u32>>, mut basin: &mut Vec<(usize, usize)>, x: usize, y: usize){
    let width = field.len();
    let height = field[0].len();
    if field[x][y] < 9  && !basin.contains(&(x, y)){
        basin.push((x, y));
        if x > 0 {
            add_to_basin(field, &mut basin, x - 1, y);
        }
        if x < width - 1 {
            add_to_basin(field, &mut basin, x + 1, y);
        }
        if y > 0 {
            add_to_basin(field, &mut basin, x, y - 1);
        }
        if y < height - 1 {
            add_to_basin(field, &mut basin, x, y + 1);
        }
    }
}

fn get_field(data: &str) -> Vec<Vec<u32>> {
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
    dots
}

fn get_low_points(field: &Vec<Vec<u32>>) -> Vec<(usize, usize)>{
    let width = field.len();
    let height = field[0].len();
    let mut low_points: Vec<(usize, usize)> = Vec::new();
    for x in 0..width{
        for y in 0..height{
            if (x == 0 || field[x][y] < field[x-1][y]) &&
                (x == width - 1 || field[x][y] < field[x+1][y]) &&
                (y == 0 || field[x][y] < field[x][y-1]) &&
                (y == height - 1 || field[x][y] < field[x][y+1]) {
                    debug!("({},{}) => {}", x, y, field[x][y]);
                    low_points.push((x, y));
            }
        }
    }
    low_points
}
