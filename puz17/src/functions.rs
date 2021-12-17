use std::{collections::HashMap, convert::TryInto, hash::Hash, io::repeat, isize, slice::SliceIndex};
use itertools::Itertools;
use log::debug;

struct Target {
    x0: i32,
    x1: i32,
    y0: i32,
    y1: i32,
}

impl Target {
    fn new(x0: i32, x1: i32, y0: i32, y1: i32) -> Self{
        Self {x0, x1, y0, y1}
    }
    fn print(&self){
        println!("{}, {} -> {}, {}", self.x0, self.y0, self.x1, self.y1);
    }
}

struct Probe{
    vx: i32,
    vy: i32,
    x: i32,
    y: i32
}

impl Probe{
    fn new(x: i32, y: i32, vx: i32, vy: i32) -> Self{
        Self {vx, vy, x, y}
    }
    fn movement(&mut self){
        match self.vx.cmp(&0){
            std::cmp::Ordering::Greater => {
                self.x +=  self.vx;
                self.vx -= 1;
            },
            std::cmp::Ordering::Less => {
                self.x -=  self.vx;
                self.vx += 1;
            },
            _ => ()
        };
        self.y += self.vy;
        self.vy -= 1;
    }
    fn pass_target(&self, target: &Target) -> bool{
        self.x > target.x1 || self.y < target.y1
    }

    fn in_target(&self, target: &Target) -> bool{
        self.x >= target.x0 && self.x <= target.x1 &&
            self.y <= target.y0 && self.y >= target.y1
    }
    fn print(&self, target: &Target){
        println!("{}, {} ({}, {}) in: {}, pass: {}", self.x, self.y, self.vx, self.vy, self.in_target(target), self.pass_target(target));
    }
}
/*
#[test]
fn test1(){
    let target = Target::new(20, 30, -5, -10);
    target.print();
    let mut probe = Probe::new(0, 0, 7, 2);
    loop{
        probe.movement();
        probe.print(&target);
        if probe.in_target(&target) || probe.pass_target(&target){
            break;
        }
    }
    let result = 1;
    assert_eq!(2, result);
}

#[test]
fn test2(){
    let target = Target::new(20, 30, -5, -10);
    target.print();
    let mut probe = Probe::new(0, 0, 6, 3);
    loop{
        probe.movement();
        probe.print(&target);
        if probe.in_target(&target) || probe.pass_target(&target){
            break;
        }
    }
    let result = 1;
    assert_eq!(2, result);
}

#[test]
fn test3(){
    let target = Target::new(20, 30, -5, -10);
    target.print();
    let mut probe = Probe::new(0, 0, 9, 0);
    loop{
        probe.movement();
        probe.print(&target);
        if probe.in_target(&target) || probe.pass_target(&target){
            break;
        }
    }
    let result = 1;
    assert_eq!(2, result);
}

#[test]
fn test4(){
    let target = Target::new(20, 30, -5, -10);
    target.print();
    let mut probe = Probe::new(0, 0, 17, -4);
    loop{
        probe.movement();
        probe.print(&target);
        if probe.in_target(&target) || probe.pass_target(&target){
            break;
        }
    }
    let result = 1;
    assert_eq!(2, result);
}
#[test]
fn test5(){
    let target = Target::new(20, 30, -5, -10);
    target.print();
    let mut maxv = (0, 0);
    let mut maxp = (0, 0);
    for vx in 0..30{
        for vy in -10..10{
            let mut probe = Probe::new(0, 0, vx, vy);
            let mut max_x = 0;
            let mut max_y = 0;
            'movements: loop{
                probe.movement();
                if probe.y > max_y{
                    max_x = probe.x;
                    max_y = probe.y;
                }
                if probe.in_target(&target) || probe.pass_target(&target){
                    break 'movements;
                }
            }
            if probe.in_target(&target) && maxp.1 < max_y{
                println!("============");
                maxv.0 = vx;
                maxv.1 = vy;
                maxp.0 = max_x;
                maxp.1 = max_y;
                println!("Position: {}, {}", maxp.0, maxp.1);
                println!("Velocity: {}, {}", maxv.0, maxv.1);
            }
        }
    }
    println!("============");
    println!("Position: {}, {}", maxp.0, maxp.1);
    println!("Velocity: {}, {}", maxv.0, maxv.1);
    assert_eq!(maxp.1, 45);
}

#[test]
fn test6(){
    println!("==== test6 ====");
    let target = Target::new(179, 201, -63, -109);
    target.print();
    let mut maxv = (0, 0);
    let mut maxp = (0, 0);
    for vx in 0..201{
        for vy in -300..300{
            let mut probe = Probe::new(0, 0, vx, vy);
            let mut max_x = 0;
            let mut max_y = 0;
            'movements: loop{
                probe.movement();
                if probe.y > max_y{
                    max_x = probe.x;
                    max_y = probe.y;
                }
                if probe.in_target(&target) || probe.pass_target(&target){
                    break 'movements;
                }
            }
            if probe.in_target(&target) && maxp.1 < max_y{
                println!("============");
                maxv.0 = vx;
                maxv.1 = vy;
                maxp.0 = max_x;
                maxp.1 = max_y;
                println!("Position: {}, {}", maxp.0, maxp.1);
                println!("Velocity: {}, {}", maxv.0, maxv.1);
            }
        }
    }
    println!("============");
    println!("Position: {}, {}", maxp.0, maxp.1);
    println!("Velocity: {}, {}", maxv.0, maxv.1);
    assert_eq!(maxp.1, 45);
}
*/
pub fn part1(data: &str) -> usize{
    0
}

pub fn part2(data: &str) -> usize{
    0
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
#[test]
fn test8(){
    let target = Target::new(179, 201, -63, -109);
    target.print();
    let mut contador = 0;
    let mut maxvx = (0, 0);
    let mut maxvy = (0, 0);
    for vx in 0..401{
        for vy in -3000..3000{
            let mut probe = Probe::new(0, 0, vx, vy);
            'movements: loop{
                probe.movement();
                if probe.in_target(&target) || probe.pass_target(&target){
                    break 'movements;
                }
            }
            if probe.in_target(&target){
                //println!("vx: {}, vy: {}", vx, vy);
                if probe.vx > maxvx.0{
                    maxvx.0 = probe.vx;
                }
                if probe.vx < maxvx.1{
                    maxvx.1 = probe.vx;
                }
                if probe.vy > maxvy.0{
                    maxvy.0 = probe.vy;
                }
                if probe.vy < maxvy.1{
                    maxvy.1 = probe.vy;
                }
                contador += 1;
            }
        }
    }
    println!("============");
    println!("VX: {},{}", maxvx.0, maxvx.1);
    println!("VY: {},{}", maxvy.0, maxvy.1);
    assert_eq!(contador, 45);
}
/*
#[test]
fn test7(){
    let target = Target::new(20, 30, -5, -10);
    target.print();
    let mut contador = 0;
    for vx in 0..31{
        for vy in -200..200{
            println!("vx: {}, vy: {}", vx, vy);
            let mut probe = Probe::new(0, 0, vx, vy);
            'movements: loop{
                probe.movement();
                if probe.in_target(&target) || probe.pass_target(&target){
                    break 'movements;
                }
            }
            if probe.in_target(&target){
                contador += 1;
            }
        }
    }
    println!("============");
    assert_eq!(contador, 45);
}
*/
