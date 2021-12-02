use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Read};
use std::path::Path;
use curl::easy::{Easy2, Handler, WriteError};

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

fn main() {
    let testdata = &"forward 5
down 5
forward 8
up 3
down 8
forward 2".to_string();
    println!("========== TEST ==========");
    part1(testdata);
    part2(testdata);
    println!("========== DEFI ==========");
    let data = &read_data_from_url("https://adventofcode.com/2021/day/2/input");
    part1(data);
    part2(data);
}

fn part1(data: &str) {
    let mut position: i32 = 0;
    let mut depth: i32 = 0;
    for line in data.lines(){
        let instruction: Vec<&str> = line.split(' ').collect();
        let value = instruction[1].parse::<i32>().unwrap();
        if instruction[0] == "forward"{
            position += value;
        }else if instruction[0] == "down" {
            depth += value;
        }else if instruction[0] == "up" {
            depth -= value;
        }
    }
    println!("==== Part 1 ====");
    println!("Position: {}", position);
    println!("Depth: {}", depth);
    println!("Solution: {}", depth * position);
}

fn part2(data: &str) {
    let mut position: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;
    for line in data.lines(){
        let instruction: Vec<&str> = line.split(' ').collect();
        let value = instruction[1].parse::<i32>().unwrap();
        if instruction[0] == "forward"{
            position += value;
            depth += aim * value;
        }else if instruction[0] == "down" {
            aim += value;
        }else if instruction[0] == "up" {
            aim -= value;
        }
    }
    println!("==== Part 2 ====");
    println!("Position: {}", position);
    println!("Depth: {}", depth);
    println!("Solution: {}", depth * position);
}
fn read_data_from_url(url: &str) -> String {
    let cookie_session = read_from_toml(".env", "cookie");
    let cookie = format!("session={}", cookie_session.unwrap());
    let mut easy = Easy2::new(Collector(Vec::new()));
    easy.cookie(&cookie);
    easy.get(true).unwrap();
    easy.url(&url).unwrap();
    easy.perform().unwrap();

    let contenido = easy.get_ref();
    String::from_utf8_lossy(&contenido.0).to_string()
}

fn read_data_from_file<P>(filename: P) -> String where P: AsRef<Path>{
    let file = File::open(filename).unwrap();
    let mut buf = String::new();
    BufReader::new(file).read_to_string(&mut buf);
    buf
}

fn read_from_toml<P>(filename: P, searched_key: &str)->Result<String, String> where P: AsRef<Path>{
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();
    for line in lines {
        let keyvalue = line.unwrap();
        let v: Vec<&str> = keyvalue.split('=').collect();
        let key = v[0].trim();
        let value = v[1].trim().to_string();
        if key == searched_key{
            return Ok(value);
        }
    }
    Err("Not found".to_string())
}



