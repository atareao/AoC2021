use crate::functions::{part1, part2};

const TEST_DATA: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

#[test]
fn hello() {
    let expected = 1;
    assert_eq!(1, expected);
}

#[test]
fn test_part1(){
    println!("========== PART 1 ==========");
    let result1 = part1(TEST_DATA);
    assert_eq!(result1, 17);
}

#[test]
fn test_part2(){
    println!("========== PART 2 ==========");
    let result1 = part2(TEST_DATA);
    assert_eq!(result1, 1);
}
