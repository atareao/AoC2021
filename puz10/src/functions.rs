use log::debug;

pub fn part1(data: &str) -> u32{
    let mut repetitions = (0, 0, 0, 0); // ), ], }, >
    for line in data.lines(){
        debug!("=== {} ===", line);
        let chars: Vec<char> = line.chars().collect();
        let mut ceros = Vec::new();
        for achar in line.chars(){
            if !is_close_char(achar){
                ceros.push(1);
            }else{
                let close_char = get_close_char(achar);
                let last_position = get_position_of_last_one(&ceros);
                if chars[last_position] == close_char{
                    ceros[last_position] = 0;
                    ceros.push(0);
                }else if achar == ')'{
                    repetitions.0 += 1;
                    break;
                }else if achar == ']'{
                    repetitions.1 += 1;
                    break;
                }else if achar == '}'{
                    repetitions.2 += 1;
                    break;
                }else if achar == '>'{
                    repetitions.3 += 1;
                    break;
                }
            }
        }
    }
    repetitions.0 * 3 + repetitions.1 * 57 + repetitions.2 * 1197 + repetitions.3 * 25137
}
pub fn part2(data: &str) -> i64 {
    let mut scores: Vec<i64> = Vec::new();
    for line in data.lines(){
        if is_corrupted(line){
            debug!("Corrupted: {}", line);
        }else{
            let mut completion: Vec<char> = Vec::new();
            debug!("=== {} ===", line);
            let chars: Vec<char> = line.chars().collect();
            let mut ceros = Vec::new();
            for achar in line.chars(){
                if !is_close_char(achar){
                    ceros.push(1);
                }else{
                    let close_char = get_close_char(achar);
                    let last_position = get_position_of_last_one(&ceros);
                    if chars[last_position] == close_char{
                        ceros[last_position] = 0;
                        ceros.push(0);
                    }
                }
            }
            let mut ceros_string = "".to_string();
            for cero in &ceros{
                ceros_string.push_str(&cero.to_string());
            }
            debug!("=== {} ===", ceros_string);

            for (index, cero) in ceros.iter().enumerate(){
                if cero == &1{
                    let achar = match chars[index] {
                        '{' => '}',
                        '[' => ']',
                        '(' => ')',
                        _ => '>',
                    };
                    completion.push(achar);
                }
            }
            completion.reverse();
            let score = calculate_score(&completion);
            let completion_string: String = completion.into_iter().collect();
            scores.push(score);
            debug!("Completion: {}, Score: {}", completion_string, score);
        }
    }
    scores.sort();
    let middle: usize = scores.len() / 2;
    debug!("Total: {}", middle);
    scores[middle]
}
fn calculate_score(completion: &Vec<char>) -> i64{
    let mut score = 0;
    for item in completion{
        let value = match item{
            ')' => 1,
            ']' => 2,
            '}' => 3,
            _ => 4
        };
        score = score * 5 + value;
    }
    score
}
fn get_position_of_last_one(ceros: &Vec<i32>) -> usize{
    let mut last_position = 0;
    for (index, cero) in ceros.iter().enumerate(){
        if cero == &1{
            last_position = index;
        }
    }
    last_position
}


fn is_close_char(achar: char) -> bool {
    achar == '}' || achar == ']' || achar == ')' || achar == '>'
}

fn get_close_char(achar: char) -> char {
    match achar {
        '}' => '{',
        ']' => '[',
        ')' => '(',
        _ => '<',
    }
}
fn is_corrupted(line: &str) -> bool {
    let chars: Vec<char> = line.chars().collect();
    let mut ceros = Vec::new();
    for achar in line.chars(){
        if !is_close_char(achar){
            ceros.push(1);
        }else{
            let close_char = get_close_char(achar);
            let last_position = get_position_of_last_one(&ceros);
            if chars[last_position] == close_char{
                ceros[last_position] = 0;
                ceros.push(0);
            }else{
                return true;
            }
        }
    }
    false
}
