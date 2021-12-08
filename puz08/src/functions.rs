use std::{io::BufRead};

use itertools::Itertools;
use log::debug;


pub fn part1(data: &str) -> i32{
    let lines = data.lines();
    let mut no_1 = 0;
    let mut no_4 = 0;
    let mut no_7 = 0;
    let mut no_8 = 0;
    for line in lines{
        if line.contains('|'){
            let izde: Vec<&str> = line.split('|').collect();
            let derecha = izde[1].trim();
            for item in derecha.split(' '){
                let chars = item.trim().chars().count();
                match chars{
                    2 => no_1 +=1,
                    4 => no_4 += 1,
                    3 => no_7 += 1,
                    7 => no_8 += 1,
                    _ => ()
                }
            }
        }
    }
    no_1 + no_4 + no_7 + no_8
}

pub fn part2(data: &str) -> i32 {
    let lines = data.lines();
    let mut total = 0;
    for line in lines{
        debug!("==========================================");
        debug!("=== {} ===", &line);
        if line.contains('|'){
            let izde: Vec<&str> = line.split('|').collect();
            let izquierda = izde[0].trim();
            let derecha = izde[1].trim();
            let mut number: [String; 10] = ["".to_string(), "".to_string(),
                                            "".to_string(), "".to_string(),
                                            "".to_string(), "".to_string(),
                                            "".to_string(), "".to_string(),
                                            "".to_string(), "".to_string()];
            let izquierda: Vec<&str> = izquierda.split(' ').collect();
            let mut repeticiones = (0, 0, 0, 0, 0, 0, 0);
            for item in &izquierda{
                for achar in item.trim().chars(){
                    match achar{
                        'a' => repeticiones.0 += 1,
                        'b' => repeticiones.1 += 1,
                        'c' => repeticiones.2 += 1,
                        'd' => repeticiones.3 += 1,
                        'e' => repeticiones.4 += 1,
                        'f' => repeticiones.5 += 1,
                        'g' => repeticiones.6 += 1,
                        _ => ()
                    }
                }
            }
            let thechar = if repeticiones.0 == 9 {'a'}
                else if repeticiones.1 == 9 {'b'}
                else if repeticiones.2 == 9 {'c'}
                else if repeticiones.3 == 9 {'d'}
                else if repeticiones.4 == 9 {'e'}
                else if repeticiones.5 == 9 {'f'}
                else {'g'};
            for item in &izquierda{
                let sorted_word = &item.chars().sorted().collect::<String>();
                match sorted_word.len(){
                    2 => number[1] = sorted_word.to_string(),
                    3 => number[7] = sorted_word.to_string(),
                    4 => number[4] = sorted_word.to_string(),
                    7 => number[8] = sorted_word.to_string(),
                    _ => ()
                }
                if !sorted_word.contains(thechar){
                    number[2] = sorted_word.to_string();
                }
            }
            debug!("=== {} => {} ===", thechar, number[2]);
            for item in &izquierda{
                let sorted_word = &item.chars().sorted().collect::<String>();
                match sorted_word.len(){
                    6 => {
                        if cadena_contains(sorted_word, &number[4]){
                            number[9] = sorted_word.to_string();
                        }else if cadena_contains(sorted_word, &number[1]){
                            number[0] = sorted_word.to_string();
                        }else{
                            number[6] = sorted_word.to_string();
                        }
                    },
                    5 => {
                        if &number[2] != sorted_word{
                            if cadena_contains(sorted_word, &number[1]){
                                debug!("{} ------- {}", sorted_word, &number[1]);
                                number[3] = sorted_word.to_string();
                            }else{
                                number[5] = sorted_word.to_string();
                            }
                        }
                    },
                    _ => ()
                }

            }
            for (index, item) in number.iter().enumerate(){
                debug!("{} => {}", index, item);
            }
            let mut resultado: String = "".to_owned();
            for original_item in derecha.split(' '){
                let sorted_word = &original_item.chars().sorted().collect::<String>();
                for (index, item) in number.iter().enumerate(){
                    if sorted_word == item{
                        debug!("{} => {} ({})", original_item, item, index);
                        resultado.push_str(&index.to_string());
                    }
                }
            }
            debug!("=== {} ===", resultado);
            total += resultado.parse::<i32>().unwrap();
        }
    }
    total
}

fn cadena_contains(cadena: &str, characters: &str) -> bool{
    for character in characters.chars(){
        if !cadena.contains(character){
            return false;
        }
    }
    true
}
