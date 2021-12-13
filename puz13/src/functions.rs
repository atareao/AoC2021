use std::convert::TryInto;
use log::debug;

pub fn part1(data: &str) -> u32{
    let (dots, orders) = read_dots_and_orders(data);
    let new_dots = fold(dots, &orders[0]);
    new_dots.len().try_into().unwrap()
}

pub fn part2(data: &str) -> u32 {
    let (dots, orders) = read_dots_and_orders(data);
    let mut new_dots = dots;
    let mut max_x = 0;
    let mut max_y = 0;
    for order in orders{
        println!("Order: {}", order);
        new_dots = fold(new_dots, &order);
    }
    for dot in &new_dots{
        println!("{}, {}", dot.0, dot.1);
        max_x = if dot.0 > max_x {dot.0} else {max_x};
        max_y = if dot.1 > max_y {dot.1} else {max_y};
    }
    for x in 0..(max_x + 1){
        for y in 0..(max_y + 1){
            let dot = (x, y);
            if new_dots.contains(&dot){
                print!("#");
            }else{
                print!(".");
            }
        }
        println!();
    }
    println!("{}, {}", max_x, max_y);
    2
}

fn fold(dots: Vec<(i32, i32)>, instruction: &str) -> Vec<(i32, i32)>{
    let mut new_dots: Vec<(i32, i32)> = Vec::new();
    let first_order: Vec<&str> = instruction.split('=').collect();
    let position:i32 = first_order[1].to_string().parse().unwrap();
    let flip_vertical = first_order[0].to_string().contains('y');
    if flip_vertical{
        for mut dot in dots{
            if dot.1 < position && !new_dots.contains(&dot){
                new_dots.push(dot);
            }else if dot.1 > position{
                dot.1 = 2 * position - dot.1;
                if !new_dots.contains(&dot){
                    new_dots.push(dot);
                }
            }
        }
    }else{
        for mut dot in dots{
            if dot.0 < position && !new_dots.contains(&dot){
                new_dots.push(dot);
            }else if dot.0 > position{
                dot.0 = 2 * position - dot.0;
                if !new_dots.contains(&dot){
                    new_dots.push(dot);
                }
            }
        }
    }
    new_dots
}


fn read_dots_and_orders(data: &str) -> (Vec<(i32, i32)>, Vec<String>){
    let mut dots: Vec<(i32, i32)> = Vec::new();
    let mut orders: Vec<String> = Vec::new();
    for line in data.lines(){
        if line.to_string().contains('='){
            orders.push(line.to_string());
        }else if line.to_string().contains(',') {
            let xy:Vec<&str> = line.split(',').collect();
            let x:i32 = xy[0].to_string().parse().unwrap();
            let y:i32 = xy[1].to_string().parse().unwrap();
            dots.push((x,y));
        }
    }
    (dots, orders)
}
