use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    println!("Solution to day 1, part 1: {}", solution_part1(&input));
    println!("Solution to day 1, part 2: {}", solution_part2(&input));
}

fn solution_part1(input: &str) -> u32 {}

fn solution_part2(input: &str) -> u32 {}

#[test]
fn test_solution_part1() {
    assert_eq!();
}
