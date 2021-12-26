use crate::functions::{part1, part2};

const TEST_DATA: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

fn hello() {
    let expected = 1;
    assert_eq!(1, expected);
}

fn test_part1(){
    println!("========== PART 1 ==========");
    let result1 = 41;//part1(TEST_DATA);
    assert_eq!(result1, 40);
}

fn test_part2(){
    println!("========== PART 2 ==========");
    let result1 = 315;//part2(TEST_DATA);
    assert_eq!(result1, 315);
}
