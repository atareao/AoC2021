use crate::functions::{part1, part2};

const TEST_DATA: &str = "3,4,3,1,2";

#[test]
fn hello() {
    let expected = 1;
    assert_eq!(1, expected);
}

#[test]
fn test_part1_18(){
    println!("========== PART 1 ==========");
    let result1 = part1(TEST_DATA, 18);
    assert_eq!(result1, 26);
}

#[test]
fn test_part1_80(){
    println!("========== PART 1 ==========");
    let result1 = part1(TEST_DATA, 80);
    assert_eq!(result1, 5934);
}

#[test]
fn test_part2(){
    println!("========== PART 2 ==========");
    let mut result2 = part2(TEST_DATA, 18);
    assert_eq!(result2, 26);
    result2 = part2(TEST_DATA, 80);
    assert_eq!(result2, 5934);
}
