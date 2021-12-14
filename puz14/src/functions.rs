use std::{collections::HashMap, convert::TryInto, hash::Hash, io::repeat, slice::SliceIndex};
use itertools::Itertools;
use log::debug;

pub fn part1(data: &str) -> u64{
    let (last_char, rules, mut repetitions) = read_template_and_rules(data);
    for i in 0..10{
        println!("=== Vuelta {} ===", i);
        polymerize(&rules, &mut repetitions);
        for repetition in &repetitions{
            println!("{} -> {}", repetition.0, repetition.1);
        }
    }
    let result = get_frequencies(&repetitions);
    println!("Min: {} -> {}", result.0.0, result.0.1);
    println!("Max: {} -> {}", result.1.0, result.1.1);
    println!("{}", last_char);
    result.1.1 - result.0.1
}
fn polymerize(rules: &HashMap<String, (String, String)>, repetitions: &mut HashMap<String, u64>){
    let mut new_items: HashMap<String, u64> = HashMap::new();
    for (key, value) in repetitions.clone().iter(){
        if *value > 0 {
            let (left_pair, right_pair) = rules.get(key).unwrap();
            let mut contador = 0;
            if new_items.contains_key(left_pair){
                contador = *new_items.get(left_pair).unwrap();
            }
            new_items.insert(left_pair.to_string(), contador + value);
            contador = 0;
            if new_items.contains_key(right_pair){
                contador = *new_items.get(right_pair).unwrap();
            }
            new_items.insert(right_pair.to_string(), contador + value);
            repetitions.insert(key.to_string(), 0);
        }
    }
    for (pair, value) in new_items{
        repetitions.insert(pair, value);
    }
}

fn print_template(turn: i32, template: &Vec<char>){
    print!("{} -> ", turn);
    for achar in template{
        print!("{}", achar);
    }
    println!();
}

pub fn part2(data: &str) -> u64 {
    let (last_char, rules, mut repetitions) = read_template_and_rules(data);
    for i in 0..40{
        println!("=== Vuelta {} ===", i);
        polymerize(&rules, &mut repetitions);
        for repetition in &repetitions{
            println!("{} -> {}", repetition.0, repetition.1);
        }
    }
    let result = get_frequencies(&repetitions);
    println!("Min: {} -> {}", result.0.0, result.0.1);
    println!("Max: {} -> {}", result.1.0, result.1.1);
    println!("{}", last_char);
    result.1.1 - result.0.1
}
fn get_frequencies(repetitions: &HashMap<String, u64>) -> ((char, u64), (char, u64)){
    let mut result: HashMap<char, u64> = HashMap::new();
    for repetition in repetitions{
        let mut pair = repetition.0.to_string();
        let left_char = pair.pop().unwrap();
        let mut contador = 0;
        if result.contains_key(&left_char){
            contador = *result.get(&left_char).unwrap();
        }
        result.insert(left_char, contador + repetition.1);
    }
    let mut min_value = 0;
    let mut min_char = '-';
    let mut max_value = 0;
    let mut max_char = '-';
    for (achar, value) in result{
        if value < min_value || min_value == 0{
            min_value = value;
            min_char = achar;
        }
        if value > max_value{
            max_value = value;
            max_char = achar;
        }
    }
    ((min_char, min_value), (max_char, max_value))
}

fn read_template_and_rules(data: &str) -> (char, HashMap<String, (String, String)>, HashMap<String, u64>){
    let mut rules: HashMap<String, (String, String)> = HashMap::new();
    let mut repetitions: HashMap<String, u64> = HashMap::new();
    let mut template: Vec<char> = Vec::new();
    let last_char: char = data.to_string().pop().unwrap();
    for line in data.lines(){
        if !line.to_string().is_empty(){
            if line.contains('>'){
                let izde:Vec<&str> = line.split(" -> ").collect();
                let pair_chars:Vec<char> = izde[0].chars().collect();
                let achar:Vec<char> = izde[1].chars().collect();
                let mut left_pair = String::from(pair_chars[0]);
                left_pair.push(achar[0]);
                let mut right_pair = String::from(achar[0]);
                right_pair.push(pair_chars[1]);
                rules.insert(izde[0].to_string(), (left_pair, right_pair));
                repetitions.insert(izde[0].to_string(), 0);
            }else{
                template = line.chars().collect();
            }
        }
    }
    for index in 0..(template.len() - 1){
        let mut pair = String::from("");
        pair.push(template[index]);
        pair.push(template[index + 1]);
        let value = &repetitions.get(&pair).unwrap().to_owned();
        repetitions.insert(pair, *value + 1);
    }
    (last_char, rules, repetitions)
}
