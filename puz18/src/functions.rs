use std::{collections::HashMap, convert::TryInto, hash::Hash, io::repeat, result, slice::SliceIndex};
use itertools::Itertools;
use log::debug;
use regex::{CaptureMatches, Regex, Match};


pub fn part1(data: &str) -> usize{
    0
}

pub fn part2(data: &str) -> usize{
    0
}
fn testeb(){
    let mut result = add(&"[9,9]", &"[9,9]");
    result = add(&result, &"[3,3]");
    result = add(&result, &"[4,4]");
    result = add(&result, &"[4,4]");
    println!("{}", &result);
    assert_eq!(&result, "!!");
}
fn testc(){
    let mut resultado = String::from("");
    let data = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]";
/*
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";
*/
    /*
    let data = "[[[[4,3],4],4],[7,[[8,4],9]]]
[1,1]";
    */
    for (index, line) in data.lines().enumerate(){
        if resultado.is_empty(){
            resultado = line.to_string();
            continue;
        }
        resultado = add(&resultado, line);
        println!("{} -> {}", index, resultado);
    }
    assert_eq!("!!", resultado);
}

fn testea(){
    let mut result = add(&"[1,1]", &"[2,2]");
    result = add(&result, &"[3,3]");
    result = add(&result, &"[4,4]");
    result = add(&result, &"[5,5]");
    result = add(&result, &"[6,6]");
    println!("{}", &result);
    result = add(&"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]", &"[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]");
    println!("{}", &result);
    result = add(&"[[[[4,3],4],4],[7,[[8,4],9]]]", &"[1,1]");
    assert_eq!(&result, "!!");
    /*
    let mut result = add(&"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]", &"[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]");
    result = reduce(&result);
    println!("{}", result);
    result = add(&result, &"[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]");
    result = reduce(&result);
    println!("{}", result);
    result = add(&result, &"[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]");
    result = reduce(&result);
    println!("{}", result);
    result = add(&result, &"[7,[5,[[3,8],[1,4]]]]");
    result = reduce(&result);
    println!("{}", result);
    result = add(&result, &"[[2,[2,2]],[8,[8,1]]]");
    result = reduce(&result);
    println!("{}", result);
    result = add(&result, &"[2,9]");
    result = reduce(&result);
    println!("{}", result);
    result = add(&result, &"[1,[[[9,3],9],[[9,0],[0,7]]]]");
    result = reduce(&result);
    println!("{}", result);
    result = add(&result, &"[[[5,[7,4]],7],1]");
    result = reduce(&result);
    println!("{}", result);
    result = add(&result, &"[[[[4,2],2],6],[8,7]]");
    println!("{}", result);
    println!("==========");
    result = reduce(&result);
    println!("{}", result);
    println!("==========");
    let data = 1;
    assert_eq!(5, data);
    */
}

pub fn add(number1: &str, number2: &str) -> String{
    let mut result = "".to_string();
    result.push('[');
    result.push_str(number1);
    result.push(',');
    result.push_str(number2);
    result.push(']');
    reduce(&result)
}

fn test_can_split(){
    let can = can_split("adfadfas11afadfdafa");
    assert_eq!(can, Some(String::from("11")));
    let can = can_split("adfadfas1afadfdafa");
    assert_eq!(can, None);
}

pub fn can_split(number: &str) -> Option<String>{
    let re = Regex::new(r"\d\d").unwrap();
    re.captures(&number).and_then(
        |capture|Some(capture[0].to_string()))
}

fn test_split(){
    let mut resultado = String::from("");
    let original = "[13,1]";
    if let Some(capture) = can_split(original){
        resultado = split(original, &capture);
    };
    println!("El resultado de splitear {} es {}", original, &resultado);
    assert_eq!("[[6,7],1]", resultado);

}
fn split(number: &str, capture: &str) -> String{
    let mut resultado = String::from("");
    let position = number.find(&capture).unwrap();
    let value = capture.parse::<usize>().unwrap();
    let left = value / 2;
    let right = value - left;
    resultado.push_str(&number[..position]);
    resultado.push('[');
    resultado.push_str(&left.to_string());
    resultado.push(',');
    resultado.push_str(&right.to_string());
    resultado.push(']');
    resultado.push_str(&number[(position + capture.len())..]);
    resultado
}

fn tet_can_explode(){
    let valor = "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[[6,6],[0,5]],[[5,6],[6,[[7,7],8]]]]]";
    assert_ne!(can_explode(&valor), None);
}
fn tests(){
    let text = "Retroactively relinquishing remunerations is reprehensible.";
    for mat in Regex::new(r"\b\w{13}\b").unwrap().find_iter(text) {
        println!("{:?}", mat);
    }
    assert_ne!(can_explode(&text), None);
}

pub fn can_explode(number: &str) -> Option<Match>{
    let re = Regex::new(r"\[\d,\d\]").unwrap();
    let rei = Regex::new(r"\d").unwrap();
    let rec = Regex::new(r"\[\]").unwrap();
    for (index, capture) in re.find_iter(number).enumerate(){
        let left = &number[..capture.start()];
        let right = &number[capture.end()..];
        let mut leftafter = rei.replace_all(&left.replace(",", ""), "").to_string();
        while let Some(_value) = &leftafter.find(']'){
            leftafter = rec.replace_all(&leftafter.to_string(), "").to_string();
        }
        if leftafter.len() > 3{
            return Some(capture);
        }
    }
    None
}

pub fn explode(number: &str, capture: Match) -> String{
    let mut resultado = String::from("");

    let left = &number[..capture.start()];
    let right = &number[capture.end()..];
    let leftpart = number[(capture.start() + 1)..(capture.start() + 2)].parse::<usize>().unwrap();
    let rightpart = number[(capture.end() - 2)..(capture.end() - 1)].parse::<usize>().unwrap();
    if let Some(position) = most_right_number(&left) {
        resultado.push_str(&add_to_position(left, position, leftpart));
    }else{
        resultado.push_str(left);
    };
    resultado.push('0');
    if let Some(position) = most_left_number(&right) {
        resultado.push_str(&add_to_position(right, position, rightpart));
    }else{
        resultado.push_str(right);
    };
    resultado
}

pub fn reduce(number: &str) -> String{
    let mut resultado = "".to_string();
    resultado.push_str(number);
    while let Some(capture) = can_explode(&resultado){
        resultado = explode(&resultado, capture);
        println!("    => Explode: {}", &resultado);
        while let Some(capture) = can_split(&resultado){
            resultado = split(&resultado, &capture);
            println!("    => Split: {}", &resultado);
            if can_explode(&resultado) != None{
                break;
            }
        }
    }
    resultado
}
fn add_to_position(cadena: &str, position: usize, number: usize) -> String{
    let mut resultado = "".to_string();
    let rr = &cadena[..position];
    let nn = cadena[position..(position + 1)].parse::<usize>().unwrap() + number;
    let ll = &cadena[(position + 1)..];
    resultado.push_str(rr);
    resultado.push_str(&nn.to_string());
    resultado.push_str(ll);
    resultado
}

fn test_most_left_number(){
    let result = most_left_number("abcd1234");
    assert_eq!(result, Some(4));
}

fn most_left_number(number: &str) -> Option<usize>{
    let mut lower:Option<usize> = None;
    for i in 0..10{
        let position = number.find(char::from_digit(i, 10).unwrap());
        if position != None && (lower == None || position < lower) {
            lower = position;
        }
    }
    lower
}

fn test_most_right_number(){
    let result = most_right_number("abcd1234");
    assert_eq!(result, Some(7));
}


fn most_right_number(number: &str) -> Option<usize>{
    let mut lower:Option<usize> = None;
    for i in 0..10{
        let position = number.rfind(char::from_digit(i, 10).unwrap());
        if position != None && (lower == None || position > lower) {
            lower = position;
        }
    }
    lower
}

fn read_map(data: &str) -> Vec<Vec<usize>>{
    let mut result: Vec<Vec<usize>> = Vec::new();
    for line in data.lines(){
        let mut row: Vec<usize> = Vec::new();
        for achar in line.chars(){
            let position:usize = achar.to_digit(10).unwrap().try_into().unwrap();
            row.push(position);
        }
        result.push(row);
    }
    result
}
