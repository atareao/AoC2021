use crate::functions::{part1, part2};

const TEST_DATA: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

#[test]
fn hello() {
    let expected = 1;
    assert_eq!(1, expected);
}

#[test]
fn test_part1(){
    println!("========== PART 1 ==========");
    let result1 = part1(TEST_DATA);
    assert_eq!(result1, 1588);
}

#[test]
fn test_part2(){
    println!("========== PART 2 ==========");
    let result1 = part2(TEST_DATA);
    assert_eq!(result1, 2188189693529);
}
