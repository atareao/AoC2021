use log::debug;


pub fn part1(data: &str) -> i32{
    let mut crabs: Vec<(i32, i32)> = Vec::new();
    let mut lines = data.lines();
    let initial_state = lines.next().unwrap().split(',');
    let mut max_position = 0;
    for is in initial_state {
        if let Ok(value) = is.parse(){
            crabs.push((value, 0));
            if max_position < value{
                max_position = value;
            }
        }
    }
    let mut min_fuel = -1;
    for i in 0..(max_position + 1){
        let mut fuel = 0;
        for j in 0..crabs.len(){
            let dist = crabs[j].0 - i;
            let crab_fuel = dist.abs();
            fuel += crab_fuel;
        }
        if min_fuel > fuel || min_fuel == -1{
            min_fuel = fuel;
        }
    }
    min_fuel
}

pub fn part2(data: &str) -> i32 {
    let mut crabs: Vec<(i32, i32)> = Vec::new();
    let mut lines = data.lines();
    let initial_state = lines.next().unwrap().split(',');
    let mut max_position = 0;
    for is in initial_state {
        if let Ok(value) = is.parse(){
            crabs.push((value, 0));
            if max_position < value{
                max_position = value;
            }
        }
    }
    let mut min_fuel = -1;
    for i in 0..(max_position + 1){
        let mut fuel = 0;
        for j in 0..crabs.len(){
            let dist = crabs[j].0 - i;
            let crab_fuel = calc_fuel(dist.abs());
            fuel += crab_fuel;
        }
        if min_fuel > fuel || min_fuel == -1{
            min_fuel = fuel;
        }
    }
    min_fuel
}

pub fn calc_fuel(dist:i32) -> i32{
    (1+dist)*dist/2
}

