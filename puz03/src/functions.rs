use std::convert::{TryFrom, TryInto};

pub fn part1(data: &str) -> i32{
    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();
    let mut ceros: Vec<i32> = Vec::new();
    let mut unos: Vec<i32> = Vec::new();
    for (i, line) in data.lines().enumerate(){
        if i == 0{
            for(_, _) in line.chars().enumerate(){
                ceros.push(0);
                unos.push(0);
            }
        }
        for(index, char) in line.chars().enumerate(){
            if char == '0'{
                ceros[index] += 1;
            }else{
                unos[index] += 1;
            }
        }
    }
    for (index, cero) in ceros.iter().enumerate() {
        if cero > &unos[index]{
            gamma.push('0');
            epsilon.push('1');
        }else{
            gamma.push('1');
            epsilon.push('0');
        }
    }
    let gammaval = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilonval = isize::from_str_radix(&epsilon, 2).unwrap();
    println!("==== Part 1 ====");
    println!("Gamma: {}, {}", gamma, gammaval);
    println!("Epsilon: {}, {}", epsilon, epsilonval);
    println!("Power consumption: {}", gammaval * epsilonval);
    (gammaval * epsilonval).try_into().unwrap()
}

pub fn part2(data: &str) -> i32 {
    let mut oxigen: Vec<String> = Vec::new();
    let mut co2: Vec<String> = Vec::new();
    for line in data.lines(){
        oxigen.push(line.to_string());
        co2.push(line.to_string());
    }
    let positions = i32::try_from(oxigen[0].len()).unwrap();
    for position in 0..positions{
        if oxigen.len() > 1{
            oxigen = more_repeated(oxigen, position);
        }
        if co2.len() > 1 {
            co2 = less_repeated(co2, position);
        }
    }
    println!("==== Part 2 ====");
    let oxigenval = isize::from_str_radix(&oxigen[0], 2).unwrap();
    let co2val = isize::from_str_radix(&co2[0], 2).unwrap();
    println!("Oxigen: {}, {}", oxigen[0], oxigenval);
    println!("C02: {}, {}", co2[0], co2val);
    println!("Life support {}", oxigenval*co2val);
    (oxigenval * co2val).try_into().unwrap()
}

fn more_repeated(vector: Vec<String>, position: i32) -> Vec<String>{
    let mut result: Vec<String> = Vec::new();
    let mut ceros = 0;
    let mut unos = 0;
    for item in vector.iter(){
        let ch = item.chars().nth(position.try_into().unwrap()).unwrap();
        if ch == '0'{
            ceros += 1;
        }else{
            unos += 1;
        }
    }
    for item in vector.iter(){
        let ch = item.chars().nth(position.try_into().unwrap()).unwrap();
        if (ch == '0' && ceros > unos) || (ch == '1' && unos >= ceros){
            result.push(item.to_string());
        }
    }
    result
}

fn less_repeated(vector: Vec<String>, position: i32) -> Vec<String>{
    let mut result: Vec<String> = Vec::new();
    let mut ceros = 0;
    let mut unos = 0;
    for item in vector.iter(){
        let ch = item.chars().nth(position.try_into().unwrap()).unwrap();
        if ch == '0'{
            ceros += 1;
        }else{
            unos += 1;
        }
    }
    for item in vector.iter(){
        let ch = item.chars().nth(position.try_into().unwrap()).unwrap();
        if (ch == '0' && unos >= ceros) || (ch == '1' && ceros > unos){
            result.push(item.to_string());
        }
    }
    result
}
