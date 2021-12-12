use crate::functions::{part1, part2};

const TEST_DATA: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
const TEST_DATA_B: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

const TEST_DATA_C: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

#[test]
fn hello() {
    let expected = 1;
    assert_eq!(1, expected);
}

#[test]
fn test_part1a(){
    println!("========== PART 1A ==========");
    let result1 = part1(TEST_DATA);
    assert_eq!(result1, 10);
}

#[test]
fn test_part1b(){
    println!("========== PART 1B ==========");
    let result1 = part1(TEST_DATA_B);
    assert_eq!(result1, 19);
}

#[test]
fn test_part1c(){
    println!("========== PART 1C ==========");
    let result1 = part1(TEST_DATA_C);
    assert_eq!(result1, 226);
}

#[test]
fn test_part2a(){
    println!("========== PART 2A ==========");
    let result2 = part2(TEST_DATA);
    assert_eq!(result2, 36);
}

#[test]
fn test_part2b(){
    println!("========== PART 2B ==========");
    let result1 = part2(TEST_DATA_B);
    assert_eq!(result1, 103);
}

#[test]
fn test_part2c(){
    println!("========== PART 2C ==========");
    let result1 = part2(TEST_DATA_C);
    assert_eq!(result1, 3509);
}

