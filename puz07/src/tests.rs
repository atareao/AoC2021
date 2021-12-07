use crate::functions::{part1, part2};

const TEST_DATA: &str = "16,1,2,0,4,2,7,1,2,14";

#[test]
fn hello() {
    let expected = 1;
    assert_eq!(1, expected);
}

#[test]
fn test_part1(){
    println!("========== PART 1 ==========");
    let result1 = part1(TEST_DATA);
    assert_eq!(result1, 37);
}


#[test]
fn test_part2(){
    println!("========== PART 2 ==========");
    let result2 = part2(TEST_DATA);
    assert_eq!(result2, 168);
}
