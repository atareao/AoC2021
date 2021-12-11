use std::convert::TryInto;

use log::debug;

pub fn part1(data: &str) -> u32{
    let mut flashes = 0;
    let mut field = get_field(data);
    let width = field.len();
    let height = field[0].len();
    for step in 0..100{
        for x in 0..width{
            for y in 0..height{
                increase_octopus(&mut field, x.try_into().unwrap(), y.try_into().unwrap(), width.try_into().unwrap(), height.try_into().unwrap());
            }
        }
        for x in 0..width{
            for y in 0..height{
                if field[x][y] > 9 {
                    field[x][y] = 0;
                    flashes += 1;
                }
            }
        }
        for y in 0..height{
            print!("{} => ", y);
            for x in 0..width{
                print!("{}", field[x][y]);
            }
            println!();
        }
        println!();
        println!("Vuelta: {}, Flashes: {}", step, flashes);
        println!();
    }
    flashes
}
pub fn part2(data: &str) -> i64 {
    let mut vueltas = 0;
    let mut field = get_field(data);
    let width = field.len();
    let height = field[0].len();
    'principal:for step in 0..1000{
        for x in 0..width{
            for y in 0..height{
                increase_octopus(&mut field, x.try_into().unwrap(), y.try_into().unwrap(), width.try_into().unwrap(), height.try_into().unwrap());
            }
        }
        for x in 0..width{
            for y in 0..height{
                if field[x][y] > 9 {
                    field[x][y] = 0;
                }
            }
        }
        println!("Vuelta: {}", step);
        for y in 0..height{
            for x in 0..width{
                if field[x][y] != 0 {
                    continue 'principal;
                }
            }
        }

        vueltas = step;
        break;
    }
    vueltas + 1
}

fn increase_octopus(field: &mut Vec<Vec<u32>>, x: i32, y: i32, width: i32, height: i32){
    if x >= 0 && y >= 0 && x < width && y < height {
        let xa: usize = x.try_into().unwrap();
        let ya: usize = y.try_into().unwrap();
        field[xa][ya] += 1;
        if field[xa][ya] == 10 {
            for incx in -1..2{
                for incy in -1..2{
                    if incx == 0 && incy == 0{
                        continue;
                    }else{
                        increase_octopus(field, x + incx, y + incy, width, height);
                    }
                }
            }
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
        println!();
    }
    dots
}

fn calculate_score(completion: &Vec<char>) -> i64{
    let mut score = 0;
    for item in completion{
        let value = match item{
            ')' => 1,
            ']' => 2,
            '}' => 3,
            _ => 4
        };
        score = score * 5 + value;
    }
    score
}
fn get_position_of_last_one(ceros: &Vec<i32>) -> usize{
    let mut last_position = 0;
    for (index, cero) in ceros.iter().enumerate(){
        if cero == &1{
            last_position = index;
        }
    }
    last_position
}


fn is_close_char(achar: char) -> bool {
    achar == '}' || achar == ']' || achar == ')' || achar == '>'
}

fn get_close_char(achar: char) -> char {
    match achar {
        '}' => '{',
        ']' => '[',
        ')' => '(',
        _ => '<',
    }
}
fn is_corrupted(line: &str) -> bool {
    let chars: Vec<char> = line.chars().collect();
    let mut ceros = Vec::new();
    for achar in line.chars(){
        if !is_close_char(achar){
            ceros.push(1);
        }else{
            let close_char = get_close_char(achar);
            let last_position = get_position_of_last_one(&ceros);
            if chars[last_position] == close_char{
                ceros[last_position] = 0;
                ceros.push(0);
            }else{
                return true;
            }
        }
    }
    false
}
