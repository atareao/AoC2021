use std::{collections::HashMap, convert::TryInto, hash::Hash, io::repeat, isize, slice::SliceIndex};
use itertools::Itertools;
use log::debug;

struct Packet{
    data: String,
    length: u32,
    header: String,
    version: u32,
    ttype: u32,
    operator_type: u32,
    length_of_subpackets: u32,
    number_of_subpackets: u32,
    value: u32,
    ceros: u32,
}
impl Packet {
    fn new(cadena: &str) -> Packet{
        let mut packet = Packet {data: cadena.to_string(),
                                 length: 0,
                                 header: "".to_string(),
                                 version: 0,
                                 ttype: 0,
                                 operator_type: 0,
                                 length_of_subpackets: 0,
                                 number_of_subpackets: 0,
                                 value: 0,
                                 ceros: 0};
        packet.init();
        packet
    }
    fn init(&mut self){
        // init
        self.length = self.data.len().try_into().unwrap();
        self.header = to_binary(&self.data[..2]);
        self.version = binary_to_decimal(&self.header[..3]);
        self.ttype = binary_to_decimal(&self.header[3..6]);
        if self.ttype == 4 { // literal value
            /*
            let subpackets = &self.data[7..];
            let mut value: String = "".to_string();
            for index in (0..subpackets.len()).step_by(5){
                let subpacket = &subpackets[index + 1..index + 5];
                value.push_str(subpacket);
                let last_subpacket = &subpackets[index..index + 1] == "0";
                if last_subpacket{
                    break;
                }
            }
            self.value = binary_to_decimal(&value);
            */
        }else{ // operator
            self.operator_type = if &self.header[6..7] == "0" {0} else {1};
            let long_header = to_binary(&self.data[..6]);
            if self.operator_type == 0{
                println!("=> First_chars: {}", &long_header[7..22]);
                self.length_of_subpackets = binary_to_decimal(&long_header[7..22]);
                self.ceros = self.length * 4 - (3 + 3 + 1 + 15) - self.length_of_subpackets;
            }else{
                println!("=> First_chars: {}", &long_header[7..18]);
                self.number_of_subpackets = binary_to_decimal(&long_header[7..18]);
            }
        }
    }
    fn print(self){
        println!("Packet: {}", self.data);
        println!("Length: {}", self.length);
        println!("Header: {}", self.header);
        println!("Version: {}", self.version);
        println!("Type: {}", self.ttype);
        println!("Ceros: {}", self.ceros);
        println!("Value: {}", self.value);
        if self.ttype != 4{
            println!("Operator type: {}", self.operator_type);
            if self.operator_type == 0{
                println!("Length of subpackets: {}", self.length_of_subpackets);
            }else{
                println!("Number of subpackets: {}", self.number_of_subpackets);
            }
        }
    }
}

#[test]
fn main_test(){
    let packet = "D2FE28";
    println!("============");
    println!("Packet: {}", &packet);
    println!("Length: {}", &packet.len());
    println!("Binary: {}", to_binary(&packet));
    let start: usize = 5;
    let end: usize = 15;
    let start_string = (0..start).map(|_| "_").collect::<String>();
    let end_string = (end..packet.len()).map(|_| "_").collect::<String>();
    let middle = get_binary(packet, start, end);
    println!("Binary: {}{}{}", start_string, middle, end_string);

    /*
    println!("============");
    let packet = Packet::new("D2FE28");
    packet.print();
    println!("============");
    let packet = Packet::new("38006F45291200");
    packet.print();
    println!("============");
    let packet = Packet::new("EE00D40C823060");
    packet.print();
    */
    let number = 10;
    assert_eq!(number, 27);
}

pub fn part1(data: &str) -> usize{
    0
}

pub fn part2(data: &str) -> usize{
    0
}
fn binary_to_decimal(cadena: &str) -> u32{
    u32::from_str_radix(&cadena, 2).unwrap()
}

fn get_binary(cadena: &str, start: usize, end: usize) -> String{
    let first4 = start / 4;
    let first4r = start % 4;
    let end4 = (end + 1) / 4;
    let end4r = (end + 1) % 4;
    //println!("{} - {} | {} - {}", first4, first4r, end4, end4r);
    let cadena = to_binary(&cadena[first4..(end4)]);
    cadena[first4r..(cadena.len() - end4r)].to_string()
}


fn to_binary(packet: &str) -> String{
    let mut binary:String = "".to_string();
    for index in 0..packet.len(){
        let binary_char = char2binary(&packet[index..(index + 1)]);
        binary.push_str(&binary_char);
    }
    binary
}

fn char2binary(achar: &str) -> String{
    let binary_char = match achar {
        "0" => "0000",
        "1" => "0001",
        "2" => "0010",
        "3" => "0011",
        "4" => "0100",
        "5" => "0101",
        "6" => "0110",
        "7" => "0111",
        "8" => "1000",
        "9" => "1001",
        "A" => "1010",
        "B" => "1011",
        "C" => "1100",
        "D" => "1101",
        "E" => "1110",
        "F" => "1111",
        _ => ""};
    binary_char.to_string()
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
