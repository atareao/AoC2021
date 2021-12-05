use crate::functions::{part1, part2};

const TEST_DATA: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

#[test]
fn hello() {
    let expected = 1;
    assert_eq!(1, expected);
}

#[test]
fn test_part1(){
    println!("========== PART 1 ==========");
    let result1 = part1(TEST_DATA);
    assert_eq!(result1, 5);
}

#[test]
fn test_part2(){
    println!("========== PART 2 ==========");
    let result2 = part2(TEST_DATA);
    assert_eq!(result2, 12);
}
