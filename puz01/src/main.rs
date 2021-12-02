use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;
use curl::easy::{Easy2, Handler, WriteError};

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
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

fn main() {
    let cookie_session = read_from_toml(".env", "cookie");

    let cookie = format!("session={}", cookie_session.unwrap());
    let url = String::from("https://adventofcode.com/2021/day/1/input");
    let mut easy = Easy2::new(Collector(Vec::new()));
    easy.cookie(&cookie);
    easy.get(true).unwrap();
    easy.url(&url).unwrap();
    easy.perform().unwrap();

    let contenido = easy.get_ref();
    let lines = String::from_utf8_lossy(&contenido.0);
    let mut numeros = Vec::new();
    let mut mayores: i32 = 0;
    let mut anterior: Option<i32> = None;
    for line in lines.lines(){
        let numero = line.parse::<i32>().unwrap();
        numeros.push(numero);
        if anterior != None && Some(numero) > anterior {
            mayores += 1;
        }
        anterior = Some(numero);
    }
    println!("Total de mayores: {}", mayores);
    let mut i = 0;
    mayores = 0;
    while i < numeros.len() - 3 {
        let a = numeros[i] + numeros[i + 1] + numeros[i + 2];
        let b = numeros[i + 1] + numeros[i + 2] + numeros[i + 3];
        i += 1;
        if b > a{
            mayores += 1;
        }
    }
    println!("Total de mayores por grupos: {}", mayores);
}

fn first_try() {
    let mut mayores: i32 = 0;
    let mut vec = Vec::new();
    if let Ok(numbers) = read_lines("./src/data.txt"){
        let mut anterior: Option<i32> = None;
        for number in numbers {
            let item = number.unwrap().parse::<i32>().unwrap();
            vec.push(item);
            if anterior != None && Some(item) > anterior{
                //println!("{} > {}", item, anterior.unwrap());
                mayores += 1;
            }
            anterior = Some(item);
        }
        let mut i = 0;
        mayores = 0;
        while i < vec.len() - 3{
            let a = vec[i] + vec[i + 1] + vec[i + 2];
            let b = vec[i + 1] + vec[i + 2] + vec[i + 3];
            //println!("{} > {}", b, a);
            i += 1;
            if b > a{
                //println!("{} > {}", item, anterior.unwrap());
                mayores += 1;
            }
        }
    }
    println!("=== {} ===", mayores);
}

fn read_lines<P>(filename: P) ->Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    Ok(BufReader::new(file).lines())
}

