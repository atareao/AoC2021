use std::{collections::HashMap, convert::TryInto, hash::Hash, io::repeat, slice::SliceIndex};
use itertools::Itertools;
use log::debug;

pub fn part1(data: &str) -> usize{
    let map: Vec<Vec<usize>> = read_map(data);
    let width = map[0].len();
    let height = map.len();
    let mut accumulate = map.clone();
    for i in 0..width{
        for j in 0..width{
            accumulate[i][j] = map[j][i];
            println!("x:{}, y:{} -> {}, {}", i, j, map[i][j], accumulate[i][j]);
        }
    }
    println!("{}, {}", width, height);
    for i in (0..width).rev(){
        for j in (0..height).rev(){
            calcula(i, j, &mut accumulate);
        }
    }
    for i in 0..width{
        for j in 0..width{
            println!("x:{}, y:{} -> {}, {}", i, j, map[i][j], accumulate[i][j]);
        }
    }
    println!("{} ", accumulate[0][0]);
    accumulate[0][0] - map[0][0]
}

pub fn part2(data: &str) -> usize{
    let map: Vec<Vec<usize>> = read_map(data);
    let width = map[0].len();
    let height = map.len();
    let mut accumulate = vec![vec![0usize; width * 5]; height * 5];
    for i in 0..width{
        for j in 0..height{
            accumulate[i][j] = map[j][i];
        }
    }
    for j in 0..height{
        for i in 0..5 * width{
            if i >= width{
                let mut value = accumulate[i - width][j] + 1;
                if value > 9{
                    value = 1;
                }
                accumulate[i][j] = value;
            }
        }
    }
    for j in height..5 * height{
        for i in 0..5 * width{
            let mut value = accumulate[i][j - height] + 1;
            if value > 9{
                value = 1;
            }
            accumulate[i][j] = value;
        }
    }
    for j in 0..(5 * height){
        for i in 0..(5 * width){
            //println!("x:{}, y:{} -> {}, {}", i, j, map[i][j], accumulate[i][j]);
            if i % 10 == 0{
                print!("|");
            }
            print!("{}", accumulate[i][j])
        }
        println!();
        if j % 9 == 0 {
            for i in 0..(5 * width){
                if i % 10 == 0 {
                    print!("|");
                }
                print!("-");
            }
            println!();
        }
    }
    /*
    let cx = 99;
    let mut cy = 0;
    println!("{},{}->{},{},{},{},{}",cx,cy, accumulate[cx][cy], accumulate[cx + 100][cy], accumulate[cx + 200][cy], accumulate[cx + 300][cy], accumulate[cx + 400][cy]);
    cy = 99;
    println!("{},{}->{},{},{},{},{}",cx,cy, accumulate[cx][cy], accumulate[cx + 100][cy], accumulate[cx + 200][cy], accumulate[cx + 300][cy], accumulate[cx + 400][cy]);
    cy = 199;
    println!("{},{}->{},{},{},{},{}",cx,cy, accumulate[cx][cy], accumulate[cx + 100][cy], accumulate[cx + 200][cy], accumulate[cx + 300][cy], accumulate[cx + 400][cy]);
    cy = 299;
    println!("{},{}->{},{},{},{},{}",cx,cy, accumulate[cx][cy], accumulate[cx + 100][cy], accumulate[cx + 200][cy], accumulate[cx + 300][cy], accumulate[cx + 400][cy]);
    cy = 399;
    println!("{},{}->{},{},{},{},{}",cx,cy, accumulate[cx][cy], accumulate[cx + 100][cy], accumulate[cx + 200][cy], accumulate[cx + 300][cy], accumulate[cx + 400][cy]);
    cy = 499;
    println!("{},{}->{},{},{},{},{}",cx,cy, accumulate[cx][cy], accumulate[cx + 100][cy], accumulate[cx + 200][cy], accumulate[cx + 300][cy], accumulate[cx + 400][cy]);
    */
    println!("{}, {}", width, height);
    for i in (0..(5*width)).rev(){
        for j in (0..(5*height)).rev(){
            calcula(i, j, &mut accumulate);
        }
    }
    /*
    for i in 0..5 * width{
        for j in 0..5 * height{
            println!("x:{}, y:{} -> {}", i, j, accumulate[i][j]);
        }
    }
    for i in 480..500{
        for j in 480..500{
            print!("{}.", accumulate[i][j])
        }
        println!();
    }
    */
    println!("Width: {}, height: {}", accumulate[0].len(), accumulate.len());
    println!("{}", map[0][0]);
    println!("{} ", accumulate[0][0]);
    println!("{} ", accumulate[0][0] - map[0][0]);
    accumulate[0][0] - 2 * map[0][0]
}
fn calcula(x:usize, y:usize, accumulate:&mut Vec<Vec<usize>>){
    let width = accumulate[0].len();
    let height = accumulate.len();
    let mut right = 0;
    let mut down = 0;
    // derecha
    let antes = accumulate[x][y];
    if x < width - 1 {
        right = accumulate[x + 1][y];
    }
    if y < height - 1 {
        down = accumulate[x][y + 1];
    }
    if right != 0 && down != 0{
        if right > down{
            accumulate[x][y] += down;
        }else{
            accumulate[x][y] += right;
        }
    }else{
        accumulate[x][y] += down + right;
    }
    /*
    if x > 45 && y > 45 {
        println!("=== x:{},y:{} => {}, {} -> {}, {}", x, y, right, down, antes, accumulate[x][y]);
    }
    */
}

fn read_map(data: &str) -> Vec<Vec<usize>>{
    let mut result: Vec<Vec<usize>> = Vec::new();
    for line in data.lines(){
        println!("{}", line);
        let mut row: Vec<usize> = Vec::new();
        for achar in line.chars(){
            let position:usize = achar.to_digit(10).unwrap().try_into().unwrap();
            row.push(position);
        }
        result.push(row);
    }
    result
}
