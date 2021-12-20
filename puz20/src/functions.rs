use std::{convert::TryInto, ptr::read};

pub fn part1(data: &str) -> usize{
    0
}

pub fn part2(data: &str) -> usize{
    0
}

const IEA: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#";
const IMAGE: &str = "...............
...............
...............
...............
...............
.....#..#......
.....#.........
.....##..#.....
.......#.......
.......###.....
...............
...............
...............
...............
...............";

#[test]
fn complete_sample(){
    let original = read_data(&IMAGE);
    let width = original[0].len();
    let height = original.len();
    let mut processed = vec![vec![0usize; width]; height];
    let mut contador = 0;
    for x in 1..(width - 1){
        for y in 1..(height - 1){
            let binary = get_data(&original, (x, y));
            let decimal = binary_to_decimal(&binary);
            let value = get_value(IEA, decimal);
            let light = if value == "#" {1} else {0};
            if value == "#" {
                contador += 1;
            }
            processed[x][y] = light;
        }
    }
    println!("{}", contador);
    contador = 0;
    for x in 1..(width - 1){
        for y in 1..(height - 1){
            let binary = get_data(&processed, (x, y));
            let decimal = binary_to_decimal(&binary);
            let value = get_value(IEA, decimal);
            if value == "#"{
                contador += 1;
            }
        }
    }
    println!("{}", contador);
    println!("{}", contador);
    assert_eq!(35, contador);
}

#[test]
fn test_get_data(){
    let image = read_data(&IMAGE);
    let result = get_data(&image, (7, 7));
    assert_eq!("000100010".to_string(), result);
}

#[test]
fn test_get_value(){
    let value = get_value(IEA, 34);
    assert_eq!("#", value);
}

#[test]
fn test_binary_to_decimal(){
    let image = read_data(&IMAGE);
    let binary = get_data(&image, (7, 7));
    let decimal = binary_to_decimal(&binary);
    assert_eq!(34, decimal);
}

fn binary_to_decimal(binary: &str) -> usize{
    let intval = isize::from_str_radix(binary, 2).unwrap();
    intval.try_into().unwrap()
}

fn get_value(iea: &str, position: usize) -> &str{
    &iea[position..(position+1)]
}

fn get_data(image: &Vec<Vec<usize>>, point: (usize, usize)) -> String{
    let mut number = "".to_string();
    if point.0 > 1 && point.0 < image[0].len() - 2 &&
            point.1 > 1 && point.1 < image.len() - 2{
        for x in (point.0 - 1)..(point.0 + 2){
            for y in (point.1 - 1)..(point.1 + 2){
                let digit = char::from_digit(image[x][y] as u32, 5).unwrap();
                number.push(digit)
            }
        }
    }
    number
}

fn read_data(data: &str) -> Vec<Vec<usize>>{
    let mut result: Vec<Vec<usize>> = Vec::new();
    for line in data.lines(){
        let mut row: Vec<usize> = Vec::new();
        for achar in line.chars(){
            let position:usize = if achar == '#' {1} else {0};
            row.push(position);
        }
        result.push(row);
    }
    result
}
