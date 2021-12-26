use std::{collections::HashMap, convert::TryInto, hash::Hash, io::repeat, isize, slice::SliceIndex, num, any::Any};
use itertools::Itertools;
use log::debug;



struct Packet{
    version: u32,
    type_id: u32,
    subpackets: Vec<Packet>,
    value: Option<u64>,
    length: usize,
}
impl Packet {
    fn new(cadena: &str) -> Packet{
        let mut packet = Packet {
            version: 0,
            type_id: 0,
            subpackets: Vec::new(),
            value: None,
            length: 0};
        packet.init(cadena);
        packet
    }
    fn init(&mut self, binary: &str){
        self.version = binary_to_decimal(&binary[0..3]).try_into().unwrap();
        self.type_id = binary_to_decimal(&binary[3..6]).try_into().unwrap();
        println!("{}, {}", self.version, self.type_id);
        if self.type_id == 4 { // literal value
            let mut value = String::from("");
            let mut position = 6;
            while position < binary.len(){
                let group = &binary[position..(position + 5)];
                value.push_str(&group[1..]);
                position += 5;
                if &group[..1] == "0"{
                    break;
                }
            }
            println!("!!!! {} !!!! => {}", &value, binary_to_decimal(&value));
            self.value = Some(binary_to_decimal(&value).try_into().unwrap());
            self.length = position;
        }else{ // operator
            if binary.len() == 6{
                println!("Resto: {}", binary);
                self.length = 6;
                return;
            }
            let operator_type = &binary[6..7];
            if operator_type == "0"{
                let length_of_subpackets: usize = binary_to_decimal(&binary[7..22]).try_into().unwrap();
                self.length = 22 + length_of_subpackets;
                let mut position = 22;
                while position < self.length {
                    if binary[position..].is_empty(){
                        println!("||| {} => {} |||", self.length, position);
                        println!("{}", binary);
                        break;
                    }
                    let subpacket = Packet::new(&binary[position..]);
                    position += subpacket.length;
                    self.subpackets.push(subpacket);
                }
            }else{
                let number_of_subpackets = binary_to_decimal(&binary[7..18]);
                let mut subpackets = 0;
                let mut position = 18;
                while subpackets < number_of_subpackets{
                    println!("=== {}/{} ===", subpackets, number_of_subpackets);
                    println!("{} {} {} {}", &binary[0..3], &binary[3..6], &binary[6..7], &binary[7..18]);
                    let subpacket = Packet::new(&binary[position..]);
                    position += subpacket.length;
                    self.subpackets.push(subpacket);
                    subpackets += 1;
                }
                self.length = position;
            }
        }
    }
    fn suma(&mut self){
        let mut resultado: u64 = 0;
        for subpacket in &self.subpackets{
            resultado += subpacket.value.unwrap();
        }
        self.value = Some(resultado);
    }
    fn producto(&mut self){
        let mut resultado: u64 = 1;
        for subpacket in &self.subpackets{
            resultado *= subpacket.value.unwrap();
        }
        self.value = Some(resultado);
    }
    fn minimo(&mut self){
        let mut minimo: Option<u64> = None;
        for subpacket in &self.subpackets{
            if let Some(valor) = subpacket.value{
                if let Some(minimo_valor) = minimo{
                    if minimo_valor > valor{
                        minimo = Some(valor);
                    }
                }else{
                    minimo = Some(valor);
                }
            }
        }
        self.value = minimo;
    }

    fn maximo(&mut self){
        let mut maximo: u64 = 0;
        for subpacket in &self.subpackets{
            if subpacket.value.unwrap() > maximo{
                maximo = subpacket.value.unwrap();
            }
        }
        self.value = Some(maximo);
    }
    fn gt(&mut self){
        let value1 = &self.subpackets[0].value.unwrap();
        let value2 = &self.subpackets[1].value.unwrap();
        if value1 > value2{
            self.value = Some(1);
        }else{
            self.value = Some(0);
        }
    }
    fn lt(&mut self){
        let value1 = &self.subpackets[0].value.unwrap();
        let value2 = &self.subpackets[1].value.unwrap();
        if value1 < value2{
            self.value = Some(1);
        }else{
            self.value = Some(0);
        }
    }
    fn eq(&mut self){
        let value1 = &self.subpackets[0].value.unwrap();
        let value2 = &self.subpackets[1].value.unwrap();
        if value1 == value2{
            self.value = Some(1);
        }else{
            self.value = Some(0);
        }
    }
    fn operate(&mut self){
        for subpacket in &mut self.subpackets{
            subpacket.operate();
        }
        if self.type_id == 0{
            self.suma();
        }else if self.type_id == 1{
            self.producto();
        }else if self.type_id == 2{
            self.minimo();
        }else if self.type_id == 3{
            self.maximo();
        }else if self.type_id == 5{
            self.gt();
        }else if self.type_id == 6{
            self.lt();
        }else if self.type_id == 7{
            self.eq();
        }
    }
    fn get_version(&self) -> u32{
        let mut version = self.version;
        for subpacket in self.subpackets.iter(){
            version += subpacket.get_version();
        }
        version
    }
    fn print(&self){
        println!("Version: {}", self.version);
        println!("Type: {}", self.type_id);
        println!("Value: {}", self.value.unwrap());
    }
}

#[test]
fn test_version_1(){
    let cadena = "8A004A801A8002F478";
    let packet = Packet::new(&to_binary(cadena));
    assert_eq!(packet.get_version(), 16);
}

#[test]
fn test_version_2(){
    let cadena = "620080001611562C8802118E34";
    let packet = Packet::new(&to_binary(cadena));
    assert_eq!(packet.get_version(), 12);
}

#[test]
fn test_version_3(){
    let cadena = "C0015000016115A2E0802F182340";
    let packet = Packet::new(&to_binary(cadena));
    assert_eq!(packet.get_version(), 23);
}

#[test]
fn test_version_4(){
    let cadena = "A0016C880162017C3686B18A3D4780";
    let packet = Packet::new(&to_binary(cadena));
    assert_eq!(packet.get_version(), 31);
}

#[test]
fn test_caso_1(){
    let cadena = "C200B40A82";
    let mut packet = Packet::new(&to_binary(cadena));
    packet.operate();
    assert_eq!(packet.value.unwrap(), 3);
}

#[test]
fn test_caso_2(){
    let cadena = "04005AC33890";
    let mut packet = Packet::new(&to_binary(cadena));
    packet.operate();
    assert_eq!(packet.value.unwrap(), 54);
}

#[test]
fn test_caso_3(){
    let cadena = "880086C3E88112";
    let mut packet = Packet::new(&to_binary(cadena));
    packet.operate();
    assert_eq!(packet.value.unwrap(), 7);
}

#[test]
fn test_caso_4(){
    let cadena = "CE00C43D881120";
    let mut packet = Packet::new(&to_binary(cadena));
    packet.operate();
    assert_eq!(packet.value.unwrap(), 9);
}

#[test]
fn test_caso_5(){
    let cadena = "D8005AC2A8F0";
    let mut packet = Packet::new(&to_binary(cadena));
    packet.operate();
    assert_eq!(packet.value.unwrap(), 1);
}

#[test]
fn test_caso_6(){
    let cadena = "F600BC2D8F";
    let mut packet = Packet::new(&to_binary(cadena));
    packet.operate();
    assert_eq!(packet.value.unwrap(), 0);
}

#[test]
fn test_caso_7(){
    let cadena = "9C005AC2F8F0";
    let mut packet = Packet::new(&to_binary(cadena));
    packet.operate();
    assert_eq!(packet.value.unwrap(), 0);
}

#[test]
fn test_caso_8(){
    let cadena = "9C0141080250320F1802104A08";
    let mut packet = Packet::new(&to_binary(cadena));
    packet.operate();
    assert_eq!(packet.value.unwrap(), 1);
}

#[test]
fn test_value(){
    let cadena = "D2FE28";
    let packet = Packet::new(&to_binary(cadena));
    assert_eq!(packet.version, 6);
    assert_eq!(packet.type_id, 4);
    assert_eq!(packet.value, Some(2021));
}

fn test_length_operator(){
    let cadena = "38006F45291200";
    let packet = Packet::new(&to_binary(cadena));
    assert_eq!(packet.version, 1);
    assert_eq!(packet.type_id, 6);
}

fn test_number_operator(){
    let cadena = "EE00D40C823060";
    let packet = Packet::new(&to_binary(cadena));
    assert_eq!(packet.version, 7);
    assert_eq!(packet.type_id, 3);
}

pub fn part1(data: &str) -> u32{
    let binary = &to_binary(data);
    println!("{}", data);
    println!("{}", binary);
    let packet = Packet::new(binary);
    packet.get_version()
}

pub fn part2(data: &str) -> u64{
    let binary = &to_binary(data);
    let mut packet = Packet::new(binary);
    packet.operate();
    packet.value.unwrap()
}
fn binary_to_decimal(cadena: &str) -> u64{
    println!("=========== {} ============", cadena);
    u64::from_str_radix(cadena, 2).unwrap()
}

fn get_binary(cadena: &str, start: usize, end: usize) -> String{
    let first4 = start / 4;
    let first4r = start % 4;
    let end4 = (end + 1) / 4;
    let end4r = (end + 1) % 4;
    let last = end4 + if end4r > 0 {1} else {0};
    let cadena_binary = to_binary(&cadena[first4..last]);
    cadena_binary[first4r..(end + 1 - first4 * 4)].to_string()
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
