use std::collections::HashMap;
use std::convert::TryInto;
use log::debug;

pub fn part1(data: &str) -> u32{
    let caves: HashMap<String, Vec<String>> = get_caves(data);
    let mut paths: Vec<Vec<String>> = Vec::new();
    let mut first_path = Vec::new();
    let mut end: bool;
    first_path.push("start".to_string());
    paths.push(first_path);
    'main: loop{
        end = true;
        'check: for apath in &paths{
            if !apath.contains(&"end".to_string()){
                end = false;
                break 'check;
            }
        }
        if end == true{
            break 'main;
        }
        let mut new_paths: Vec<Vec<String>> = Vec::new();
        for apath in &paths{
            for new_path in get_paths(&caves, apath){
                new_paths.push(new_path);
            }
        }
        paths = new_paths;
    }
    let mut contador = 0;
    for apath in &paths{
        contador += 1;
        for cave in apath{
            print!("{}-", cave);
        }
        println!();
    }
    contador
}
pub fn part2(data: &str) -> i64 {
    let caves: HashMap<String, Vec<String>> = get_caves(data);
    let mut paths: Vec<Vec<String>> = Vec::new();
    let mut first_path = Vec::new();
    let mut end: bool;
    first_path.push("start".to_string());
    paths.push(first_path);
    'main: loop{
        end = true;
        'check: for apath in &paths{
            if !apath.contains(&"end".to_string()){
                end = false;
                break 'check;
            }
        }
        if end{
            break 'main;
        }
        let mut new_paths: Vec<Vec<String>> = Vec::new();
        for apath in &paths{
            for new_path in get_paths_v2(&caves, apath){
                new_paths.push(new_path);
            }
        }
        paths = new_paths;
        println!("========================");
        for apath in &paths{
            for cave in apath{
                print!("{}-", cave);
            }
            println!();
        }
    }
    let mut contador = 0;
    for apath in &paths{
        contador += 1;
        for cave in apath{
            print!("{}-", cave);
        }
        println!();
    }
    contador
}


fn get_caves(data: &str) -> HashMap<String, Vec<String>> {
    let mut caves: HashMap<String, Vec<String>> = HashMap::new();
    for line in data.lines(){
        let mut izde:Vec<String> = Vec::new();
        for item in line.split('-'){
            izde.push(item.to_string());
        }
        if !caves.contains_key(&izde[0]){
            let children = vec![izde[1].to_string()];
            caves.insert(izde[0].clone(), children);
        }else if !caves[&izde[0]].contains(&izde[1].to_string()){
            let mut children: Vec<String> = Vec::new();
            for child in caves.get(&izde[0]).unwrap(){
                children.push(child.to_string());
            }
            children.push(izde[1].clone());
            caves.insert(izde[0].clone(), children);
        }
        if !caves.contains_key(&izde[1]){
            let children = vec![izde[0].to_string()];
            caves.insert(izde[1].clone(), children);
        }else if !caves[&izde[1]].contains(&izde[0].to_string()){
            let mut children: Vec<String> = Vec::new();
            for child in caves.get(&izde[1]).unwrap(){
                children.push(child.to_string());
            }
            children.push(izde[0].clone());
            caves.insert(izde[1].clone(), children);
        }
    }
    caves
}
fn get_paths(caves: &HashMap<String, Vec<String>>, leftpath: &Vec<String>) -> Vec<Vec<String>> {
    let mut paths: Vec<Vec<String>> = Vec::new();
    let last_cave = leftpath.last().unwrap();
    if last_cave == "end" {
        paths.push(leftpath.to_vec());
    }else{
        for child_cave in &caves[last_cave]{
            let mut apath = leftpath.to_owned();
            if child_cave != last_cave && (child_cave.to_lowercase() != child_cave.to_string() || !leftpath.contains(child_cave) ){
                apath.push(child_cave.to_string());
                paths.push(apath);
            }
        }
    }
    paths
}
fn get_paths_v2(caves: &HashMap<String, Vec<String>>, leftpath: &Vec<String>) -> Vec<Vec<String>> {
    let mut paths: Vec<Vec<String>> = Vec::new();
    let last_cave = leftpath.last().unwrap();
    if last_cave == "end"{
        paths.push(leftpath.to_vec());
    }else{
        for child_cave in &caves[last_cave]{
            let mut apath = leftpath.to_owned();
            let revisit = can_revisit(leftpath);
            if child_cave != last_cave && child_cave != "start" && (
                child_cave.to_lowercase() != *child_cave || !leftpath.contains(child_cave) || revisit){
                apath.push(child_cave.to_string());
                paths.push(apath);
            }
        }
    }
    paths
}

fn can_revisit(visited_caves: &Vec<String>) -> bool{
    let mut map: HashMap<String, usize> = HashMap::new();
    for visited_cave in visited_caves{
        if map.contains_key(visited_cave) {
            return false;
        }else if visited_cave.to_lowercase() == *visited_cave{
            map.insert(visited_cave.to_string(), 1);
        }
    }
    true
}
