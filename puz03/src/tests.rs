use crate::functions::{part1, part2};

const TEST_DATA: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

#[test]
fn hello() {
    let expected = 1;
    assert_eq!(1, expected);
}

#[test]
fn test_part1(){
    println!("========== PART 1 ==========");
    let result1 = part1(TEST_DATA);
    assert_eq!(result1, 198);
}

#[test]
fn test_part2(){
    println!("========== PART 2 ==========");
    let result2 = part2(TEST_DATA);
    assert_eq!(result2, 230);
}
