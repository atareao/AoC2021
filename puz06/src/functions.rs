use log::debug;


pub fn part1(data: &str, days: usize) -> usize{
    let mut lanternfishes: Vec<i32> = Vec::new();
    let mut lines = data.lines();
    let initial_state = lines.next().unwrap().split(',');
    for is in initial_state {
        if let Ok(value) = is.parse(){
            lanternfishes.push(value);
        }
    }
    for i in 1..(days + 1) {
        for index in 0..lanternfishes.len(){
            if lanternfishes[index] == 0 {
                lanternfishes[index] = 6;
                lanternfishes.push(8);
            }else{
                lanternfishes[index] -= 1;
            }
        }
    }
    lanternfishes.len()
}

pub fn part2(data: &str, days:usize) -> i64 {
    let mut grupos: [i64; 9] = [0, 0, 0, 0, 0 ,0, 0, 0, 0];
    let mut lines = data.lines();
    let initial_state = lines.next().unwrap().split(',');
    for is in initial_state {
        if let Ok(value) = is.parse::<usize>(){
            grupos[value] += 1;
        }
    }
    for i in 1..(days + 1) {
        let initial = grupos[0];
        for i in 0..8{
            grupos[i] =  grupos[i + 1];
        }
        grupos[6] += initial;
        grupos[8] = initial;
        let mut peces = 0;
        for item in &grupos{
            peces += item;
        }
        debug!("day {} ({})", i, peces);
    }
    let mut peces:i64 = 0;
    for item in &grupos{
        peces += item;
    }
    peces
}

