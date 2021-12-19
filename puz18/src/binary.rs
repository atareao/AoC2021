use log::debug;
use std::{collections::HashMap, convert::TryInto, os::unix::prelude::OpenOptionsExt, usize};

struct Element{
    index: usize,
    parent: Option<usize>,
    level: usize,
    value: Option<u32>,
}
impl Element{
    fn new(index: usize, parent: Option<usize>, level: usize, value: Option<u32>) -> Element{
        Element {index, parent, level, value}
    }
    fn reclone(&mut self) -> Element{
        Element {
            index: self.index,
            parent: self.parent,
            level: self.level,
            value: self.value
        }
    }
    fn print(&self){
        if let Some(valor) = self.value{
            if let Some(parent_index) = self.parent{
                println!("Index: {} Parent: {}, Level: {}, Valor: {}",
                        self.index, parent_index, self.level, valor);
            }else{
                println!("Index: {} Parent: RAIZ, Level: {}, Valor: {}",
                        self.index, self.level, valor);
            }
        }else if let Some(parent_index) = self.parent{
            println!("Index: {} Parent: {}, Level: {}",
                    self.index, parent_index, self.level);
        }else{
            println!("Index: {} Parent: RAIZ, Level: {}",
                    self.index,self.level);
        }
    }
}
impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool{
        self.index == other.index &&
            self.parent == other.parent &&
            self.level == other.level &&
            self.value == other.value
    }
}
struct Number{
    nodes: Vec<Element>,
}

impl Number{
    fn len(&self) -> usize{
        return self.nodes.len()
    }
    fn empty() -> Number{
        Number{nodes: Vec::new()}
    }
    fn new(cadena: &str) -> Number{
        Number{nodes: parser(cadena)}
    }
    fn print(&self){
        for node in &self.nodes{
            node.print();
        }
    }
    fn add_node(&mut self, parent: Option<usize>, level: usize, value: Option<u32>){
        let new_node = Element{
            index: self.nodes.len(),
            parent,
            level,
            value
        };
        self.nodes.push(new_node);
    }
    fn remove_node(&mut self, position: usize){
        for index in position..self.len(){
            self.nodes[index].index -= 1;
        }
        self.nodes.remove(position);
    }
    fn remove_node_for_calculate(&mut self, position: usize){
        for index in position..self.len(){
            self.nodes[index].index -= 1;
            if let Some(parent_index) = self.nodes[index].parent{
                if parent_index > 0 && parent_index >= position{
                    self.nodes[index].parent = Some(parent_index - 1);
                }
            };
        }
        self.nodes.remove(position);
    }
    fn get_explodeable(&self) -> Vec<(usize, Option<usize>, usize, Option<u32>)>{
        //index, parent, level, value
        let mut resultado: Vec<(usize, Option<usize>, usize, Option<u32>)> = Vec::new();
        for node in &self.nodes{
            if node.value != None && node.level == 6{
                if !resultado.is_empty(){
                    let last_item = resultado[resultado.len() - 1];
                    if node.parent != last_item.1 || node.level != last_item.2{
                        resultado.push((node.index, node.parent, node.level, node.value));
                    }
                }else if node.index < &self.nodes.len() - 1{
                    let next_item = &self.nodes[node.index + 1];
                    if next_item.level == node.level &&
                            next_item.parent == node.parent &&
                            next_item.value != None{
                        println!("El nodo {} es explodeable", node.index);
                        //self.print();
                        resultado.push((node.index, node.parent, node.level, node.value));
                    }
                }
            }
        }
        resultado
    }
    fn get_spliteable(&self) -> Vec<usize>{
        let mut resultado = Vec::new();
        for node in &self.nodes{
            if node.value > Some(9){
                resultado.push(node.index);
            }
        }
        resultado
    }

    fn reduce(&mut self){
        let mut explodable = self.get_explodeable();
        let mut spliteable:Vec<usize> = Vec::new();
        while explodable.len() + spliteable.len() > 0{
            if !explodable.is_empty(){
                self.explode_node(explodable[0].0);
            }else if !spliteable.is_empty(){
                self.split_node(spliteable[0]);
            }
            spliteable = self.get_spliteable();
            explodable = self.get_explodeable();
        }
    }
    fn calculate_node(&mut self, position: usize){
        let current_value = self.nodes[position].value;
        let next_value = self.nodes[position + 1].value;
        if current_value != None && next_value != None{
            let parent_index = self.nodes[position].parent.unwrap();
            self.nodes[parent_index].value = Some(current_value.unwrap() * 3 + next_value.unwrap() * 2);
            self.remove_node_for_calculate(position + 1);
            self.remove_node_for_calculate(position);
        }
    }
    fn calculate(&mut self){
        let mut calculable = self.get_calculable();
        while !calculable.is_empty(){
            self.calculate_node(calculable[0].0);
            calculable = self.get_calculable();
        }
        println!("RESULTADO");
        self.print();
    }
    fn get_calculable(&self) -> Vec<(usize, Option<usize>, usize, Option<u32>)>{
        //index, parent, level, value
        let mut resultado: Vec<(usize, Option<usize>, usize, Option<u32>)> = Vec::new();
        for node in &self.nodes{
            if node.value != None{
                if !resultado.is_empty(){
                    let last_item = resultado[resultado.len() - 1];
                    if (node.parent != last_item.1 || node.level != last_item.2) &&
                            node.index < &self.nodes.len() - 1{
                        let next_item = &self.nodes[node.index + 1];
                        if next_item.level == node.level &&
                                next_item.parent == node.parent &&
                                next_item.value != None{
                            resultado.push((node.index, node.parent, node.level, node.value));
                        }
                    }
                }else{
                    if node.index < &self.nodes.len() - 1{
                        let next_item = &self.nodes[node.index + 1];
                        if next_item.level == node.level &&
                                next_item.parent == node.parent &&
                                next_item.value != None{
                            resultado.push((node.index, node.parent, node.level, node.value));
                        }
                    }
                }
            }
        }
        resultado
    }

    fn split_node(&mut self, position: usize){
        if position < self.len(){
            let current_value = self.nodes[position].value.unwrap();
            if current_value > 9 {
                for i in (position + 1)..self.len(){
                    self.nodes[i].index += 1;
                }
            }
            self.nodes[position].level += 1;
            self.nodes[position].value = Some(current_value / 2);
            let new_node = Element{
                index: position + 1,
                parent: self.nodes[position].parent,
                level: self.nodes[position].level,
                value: Some(current_value - current_value / 2)};
            self.nodes.insert(position + 1, new_node);
        }
    }
    fn explode_node(&mut self, position: usize){
            let current_value = self.nodes[position].value;
            let next_value = self.nodes[position + 1].value;
            if current_value != None && next_value != None{
                for i in (0..position).rev(){
                    if self.nodes[i].value != None{
                        self.nodes[i].value = Some(
                            self.nodes[i].value.unwrap() +
                            current_value.unwrap());
                        break;
                    }
                }
                for i in (position + 2)..self.len(){
                    if self.nodes[i].value != None{
                        self.nodes[i].value = Some(
                            self.nodes[i].value.unwrap() +
                            next_value.unwrap());
                        break;
                    }
                }
                self.nodes[position].level -= 1;
                self.nodes[position].value = Some(0);
                self.remove_node(position + 1);
            }
    }
    fn add(&self, number: Number) -> Number{
        let mut new_number = Number::empty();
        new_number.add_node(None, 1, None);
        for node in &self.nodes{
            let mut parent = Some(0);
            if node.parent != None{
                parent = Some(node.parent.unwrap() + 1);
            }
            let level = node.level + 1;
            let value = node.value;
            new_number.add_node(parent, level, value);
        }
        let base = new_number.len();
        for node in number.nodes{
            let mut parent = Some(0);
            if node.parent != None{
                parent = Some(node.parent.unwrap() + base);
            }
            let level = node.level + 1;
            let value = node.value;
            new_number.add_node(parent, level, value);
        }
        new_number
    }
}

fn otro_test(){
    let mut suma = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";
    println!("=== |-| ===");
    println!("{}", &suma);
    let mut number1 = Number::new(suma);
    number1.print();
    number1.calculate();
}

#[test]
fn mi_test(){
    let mut suma = Number::new("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]");
let data = "[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]][1,1]";
    for line in data.lines(){
        suma = suma.add(Number::new(&line));
        suma.reduce();
    }
    println!("==========================================");
    suma.print();
    println!("==========================================");
    //suma.calculate();
    //suma.print();

    let value = 2;
    assert_eq!(1, value);
}


fn parser(line: &str) -> Vec<Element>{
    let mut resultado: Vec<Element> = Vec::new();
    let mut parent = None;
    let mut level = 0;
    let mut opened = 0;
    let mut closed = 0;
    for achar in line.chars(){
        match achar{
            '[' => {
                opened += 1;
                level += 1;
                let node = Element::new(resultado.len(), parent, level, None);
                resultado.push(node);
                parent = Some(resultado.len() - 1);
            },
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                level += 1;
                let node = Element::new(resultado.len(), parent, level,
                    Some(achar.to_digit(10).unwrap().try_into().unwrap()));
                resultado.push(node);
            },
            ',' => {
                level -= 1;
            },
            ']' => {
                closed += 1;
                level -= 1;
                if let Some(value) = parent {
                    parent = resultado[value].parent;
                };
            },
            _ => ()
        }
    }
    resultado
}
