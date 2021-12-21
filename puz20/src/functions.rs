use std::{convert::TryInto};

pub fn part1(iea: &str, image: &str) -> usize{
    let original = read_data(image);
    let width:i32 = original[0].len().try_into().unwrap();
    let height:i32 = original.len().try_into().unwrap();
    println!("{}", width);
    let incx = 100;
    let incy = 0;
    let nw:usize = (width + 2 * incx).try_into().unwrap();
    let nh:usize = (height + 2 * incy).try_into().unwrap();
    let mut processed = vec![vec![0usize; nw]; nh];
    let mut contador = 0;
    for x in (0 - incx)..(width + incx){
        for y in (0 - incy)..(height + incy){
            let binary = get_data(&original, x, y);
            let decimal = binary_to_decimal(&binary);
            let value = get_value(iea, decimal);
            let light = if value == "#" {1} else {0};
            if value == "#" {
                contador += 1;
            }
            let xp: usize = (x + incx).try_into().unwrap();
            let yp: usize = (y + incy).try_into().unwrap();
            println!("{}, {}", nw, nh);
            println!("{}, {} -> {},{}", x, y, xp, yp);
            processed[xp][yp] = light;
        }
    }
    contador = 0;
    for x in 0..(width + 2 * incx - 1){
        for y in 0..(height + 2 * incy - 1){
            let binary = get_data(&processed, x, y);
            let decimal = binary_to_decimal(&binary);
            let value = get_value(IEA, decimal);
            if value == "#"{
                contador += 1;
            }
        }
    }
    contador
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
    for x in 0..width{
        for y in 0..height{
            let binary = get_data(&original, x.try_into().unwrap(),
                y.try_into().unwrap());
            let decimal = binary_to_decimal(&binary);
            let value = get_value(IEA, decimal);
            let light = if value == "#" {1} else {0};
            if value == "#" {
                contador += 1;
            }
            processed[x][y] = light;
        }
    }
    contador = 0;
    for x in 1..(width - 1){
        for y in 1..(height - 1){
            let binary = get_data(&processed, x.try_into().unwrap(),
                y.try_into().unwrap());
            let decimal = binary_to_decimal(&binary);
            let value = get_value(IEA, decimal);
            if value == "#"{
                contador += 1;
            }
        }
    }
    assert_eq!(35, contador);
}

#[test]
fn test_get_data(){
    let image = read_data(&IMAGE);
    let result = get_data(&image, 7, 7);
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
    let binary = get_data(&image, 7, 7);
    let decimal = binary_to_decimal(&binary);
    assert_eq!(34, decimal);
}

fn binary_to_decimal(binary: &str) -> usize{
    let intval = match isize::from_str_radix(binary, 2){
        Ok(value) => value,
        _ => {println!("=== {} ===", binary);0}
    };
    intval.try_into().unwrap()
}

fn get_value(iea: &str, position: usize) -> &str{
    &iea[position..(position+1)]
}

fn get_data(image: &Vec<Vec<usize>>, xc: i32, yc: i32) -> String{
    // Reconstruir la imagen teniendo presente el efecto borde
    let mut number = "".to_string();
    let width:i32 = image[0].len().try_into().unwrap();
    let height:i32 = image.len().try_into().unwrap();
    for x in (xc - 1)..(xc + 2){
        for y in (yc - 1)..(yc + 2){
            if x > -1 && y > -1 && x < width && y < height{
                let i:usize = x.try_into().unwrap();
                let j:usize = y.try_into().unwrap();
                let digit = char::from_digit(image[i][j] as u32, 10).unwrap();
                number.push(digit);
            }else{
                number.push('0');
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
