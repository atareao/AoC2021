use std::{convert::TryInto};

use regex::Split;

pub fn part1() -> u32{
    let mut player1 = Player::new("1", 3);
    let mut player2 = Player::new("2", 7);
    let mut die_value = 1;
    let mut contador = 0;
    loop{
        die_value = player1.play(die_value);
        contador += 3;
        player1.print();
        if player1.score >= 1000{
            break;
        }
        die_value = player2.play(die_value);
        player2.print();
        if player2.score >= 1000{
            break;
        }
        contador += 3;
    }
    player1.print();
    player2.print();
    let mut value = 0;
    if player1.score > player2.score{
        println!("El ganador fue: {}", player1.name);
        value = player2.score * contador;
    }else{
        println!("El ganador fue: {}", player2.name);
        value = player1.score * contador;
    }
    println!("El dado fue lanzado {} veces", contador);
    println!("Valor final: {}", value);
    value
}

pub fn part2(data: &str) -> usize{
    0
}

struct Player{
    name: String,
    position: u32,
    score: u32
}

impl Player{
    fn new(name: &str, position: u32) -> Player{
        Self{name: name.to_string(), position, score: 0}
    }
    fn play(&mut self, die_value: u32) -> u32{
        self.position = (3 * (die_value + 1) + self.position) % 10;
        if self.position == 0{
            self.position = 10;
        }
        self.score += self.position;
        die_value + 3
    }
    fn print(&self){
        println!("Name: {}. Position: {}, Score: {}", self.name, self.position,
                 self.score);
    }
}

struct Memory{
    number: String,
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl Memory {
    fn new(number: &str) -> Memory{
        Self{
            number: number.to_string(),
            w: 0,
            x: 0,
            y: 0,
            z: 0
        }
    }

    fn pop(&mut self) -> i64{
        let mut chars = self.number.chars();
        let first:i64 = chars.next().unwrap().to_digit(10).unwrap().try_into().unwrap();
        self.number = chars.as_str().to_string();
        first
    }

    fn get(&self, variable: &str) -> i64{
        match variable {
            "w" => self.w,
            "x" => self.x,
            "y" => self.y,
            "z" => self.z,
            _ => variable.parse().unwrap(),
        }
    }

    fn set(&mut self, variable: &str, value: i64){
        match variable {
            "w" => {self.w = value},
            "x" => {self.x = value},
            "y" => {self.y = value},
            "z" => {self.z = value},
            _ => (),
        }
    }
    fn command(&mut self, operation: &str){

        let parts: Vec<&str> = operation.split(' ').collect();
        if parts.len() > 2 {
            self.operation(parts[0], parts[1], parts[2]);
        }else{
            self.operation(parts[0], parts[1], "");
        }
    }

    fn operation(&mut self, instruction: &str, var1: &str, var2: &str){
        let value1 = self.get(var1);
        let value2 = if var2.is_empty() {self.pop()} else {self.get(var2)};
        match instruction {
            "inp" => {self.set(var1, value2)},
            "add" => {self.set(var1, value1 + value2)},
            "mul" => {self.set(var1, value1 * value2)},
            "div" => {self.set(var1, value1 / value2)},
            "mod" => {self.set(var1, value1 % value2)},
            "eql" => {self.set(var1, if value1 == value2 {1} else {0})},
            _ => (),
        };
    }

    fn print(&self){
        println!("w: {}, x:{}, y:{}, z:{}", self.w, self.x, self.y, self.z);
    }
}

//#[test]
fn test_pop(){
    let mut sample = Memory::new("13579246899999");
    assert_eq!(sample.pop(), 1);
}

//#[test]
fn complete_sample(){
    let data = "inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2";
    let mut sample = Memory::new("13579246899999");
    for line in data.lines(){
        sample.command(line);
        sample.print();
    }
    let cosa = 1;
    assert_eq!(2, cosa);
}

#[test]
fn second_test(){
    let data = "inp w
mul x 0
add x z
mod x 26
div z 1
add x 13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 0
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 3
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 8
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -5
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 5
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 13
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 9
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 6
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 1
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -8
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 1
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 2
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x 0
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 7
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -5
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 5
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -9
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 8
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -1
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 15
mul y x
add z y
";

    /*
    let mut sample = Memory::new("13579246899999");
    for line in data.lines(){
        println!("==== {} ====", &line);
        sample.command(line);
        sample.print();
    }
    */
    let mut contador = 0;
    //'main: for i in 11111111111111..99999999999999u64{
    'main: for i in (11111111111111..99999999999999u64).rev(){
        let number = i.to_string();
        println!("Probando con {}", &number);
        let mut sample = Memory::new(&number);
        for line in data.lines(){
            //println!("====== {} ======", line);
            sample.command(line);
            //sample.print();
            if sample.z > 0{
                continue 'main;
            }
        }
        println!("{} -> {}", &number, sample.z);
        if sample.z == 0{
            println!("Number {} has z == 0", &number);
            sample.print();
            break 'main;
        }
        if contador > 1000{
            break 'main;

        }
        contador += 1;
    }
    let cosa = 1;
    assert_eq!(cosa, 2);
}
