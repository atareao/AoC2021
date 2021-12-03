use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Read};
use std::path::Path;
use curl::easy::{Easy2, Handler, WriteError};
use std::convert::{TryFrom, TryInto};

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

fn main() {
    let testdata = &"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
".to_string();
    println!("========== PART 1 ==========");
    //part1(testdata);
    let data = &read_data_from_url("https://adventofcode.com/2021/day/3/input");
    //part1(data);
    println!("========== PART 2 ==========");
    part2(testdata);
    part2(data);
}

fn part2(data: &str) {
    let mut oxigen: Vec<String> = Vec::new();
    let mut co2: Vec<String> = Vec::new();
    for line in data.lines(){
        oxigen.push(line.to_string());
        co2.push(line.to_string());
    }
    let positions = i32::try_from(oxigen[0].len()).unwrap();
    for position in 0..positions{
        println!("Position: {}", position);
        if oxigen.len() > 1{
            oxigen = more_repeated(oxigen, position);
            println!("Oxigen: {}", oxigen.len());
        }
        if co2.len() > 1 {
            co2 = less_repeated(co2, position);
            println!("C02: {}", co2.len());
        }
    }
    println!("==== Part 2 ====");
    let oxigenval = isize::from_str_radix(&oxigen[0], 2).unwrap();
    let co2val = isize::from_str_radix(&co2[0], 2).unwrap();
    println!("Oxigen: {}, {}", oxigen[0], oxigenval);
    println!("C02: {}, {}", co2[0], co2val);
    println!("Life support {}", oxigenval*co2val);
}

fn more_repeated(vector: Vec<String>, position: i32) -> Vec<String>{
    let mut result: Vec<String> = Vec::new();
    let mut ceros = 0;
    let mut unos = 0;
    for item in vector.iter(){
        let ch = item.chars().nth(position.try_into().unwrap()).unwrap();
        if ch == '0'{
            ceros += 1;
        }else{
            unos += 1;
        }
    }
    for item in vector.iter(){
        let ch = item.chars().nth(position.try_into().unwrap()).unwrap();
        if (ch == '0' && ceros > unos) || (ch == '1' && unos >= ceros){
            result.push(item.to_string());
        }
    }
    result
}

fn less_repeated(vector: Vec<String>, position: i32) -> Vec<String>{
    let mut result: Vec<String> = Vec::new();
    let mut ceros = 0;
    let mut unos = 0;
    for item in vector.iter(){
        let ch = item.chars().nth(position.try_into().unwrap()).unwrap();
        if ch == '0'{
            ceros += 1;
        }else{
            unos += 1;
        }
    }
    for item in vector.iter(){
        let ch = item.chars().nth(position.try_into().unwrap()).unwrap();
        if (ch == '0' && unos >= ceros) || (ch == '1' && ceros > unos){
            result.push(item.to_string());
        }
    }
    result
}
fn part1(data: &str) {
    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();
    let mut ceros: Vec<i32> = Vec::new();
    let mut unos: Vec<i32> = Vec::new();
    for (i, line) in data.lines().enumerate(){
        if i == 0{
            for(_, _) in line.chars().enumerate(){
                ceros.push(0);
                unos.push(0);
            }
        }
        for(index, char) in line.chars().enumerate(){
            if char == '0'{
                ceros[index] += 1;
            }else{
                unos[index] += 1;
            }
        }
    }
    for (index, cero) in ceros.iter().enumerate() {
        if cero > &unos[index]{
            gamma.push('0');
            epsilon.push('1');
        }else{
            gamma.push('1');
            epsilon.push('0');
        }
    }
    let gammaval = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilonval = isize::from_str_radix(&epsilon, 2).unwrap();
    println!("==== Part 1 ====");
    println!("Gamma: {}, {}", gamma, gammaval);
    println!("Epsilon: {}, {}", epsilon, epsilonval);
    println!("Power consumption: {}", gammaval * epsilonval);
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



