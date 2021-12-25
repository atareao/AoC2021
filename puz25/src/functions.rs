use std::{convert::TryInto};

use regex::Split;


pub fn part2(data: &str) -> usize{
    0
}

#[test]
fn sample(){
    let data = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
    let value = part1(&data);
    assert_eq!(58, value);
}

pub fn part1(data: &str) -> usize{
    let lines:Vec<&str> = data.lines().collect();
    let width = lines[0].chars().count();
    let height = lines.len();
    println!("Width: {}, Height: {}", width, height);
    let mut map = vec![vec!['.'; height]; width];
    for (j, line) in lines.iter().enumerate(){
        for (i, achar) in line.chars().into_iter().enumerate(){
            map[i][j] = achar;
        }
    }
    let mut contador = 0;
    let mut movements = 1;
    while movements > 0{
        movements = 0;
        for j in 0..height{
            movements += move_row(j, width, &mut map);
        }
        for i in 0..width{
            movements += move_col(i, height, &mut map);
        }
        contador += 1;
        println!("=== Movement number: {}. Movements: {} ===", contador, movements);
    }
    contador
}

fn move_row(row: usize, width: usize, map: &mut Vec<Vec<char>>) -> usize{
    let mut movements = 0;
    let mut pos = 0;
    let first = map[0][row];
    let previous = map[width - 1][row];
    while pos < width - 1 {
        if map[pos][row] == '>' && map[pos + 1][row] == '.'{
            map[pos][row] = '.';
            map[pos + 1][row] = '>';
            pos += 1;
            movements += 1;
        }
        pos += 1;
    }
    if previous == '>' && first == '.'{
        map[width - 1][row] = '.';
        map[0][row] = '>';
        movements += 1;
    }
    movements
}

fn move_col(col: usize, height: usize, map: &mut Vec<Vec<char>>) -> usize{
    let mut movements = 0;
    let mut pos = 0;
    let first = map[col][0];
    let previous = map[col][height - 1];
    while pos < height - 1 {
        if map[col][pos] == 'v' && map[col][pos + 1] == '.'{
            map[col][pos] = '.';
            map[col][pos + 1] = 'v';
            pos += 1;
            movements += 1;
        }
        pos += 1;
    }
    if previous == 'v' && first == '.'{
        map[col][height - 1] = '.';
        map[col][0] = 'v';
        movements += 1;
    }
    movements
}
