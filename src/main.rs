include!("day1.rs");
include!("day2.rs");
include!("day3.rs");
trait Solution{
    fn part1(&self, input: &str) -> i64;
    fn part2(&self, input: &str) -> i64;
}

fn read_input(file: String) -> String {
    std::fs::read_to_string(file).expect("Failed to read file")
}

fn main() {
    println!("Hello, Advent of Code 2025!");


    let day1 = Day1{};
    let day1_data = read_input("inputs/day1.txt".to_string());
    println!("Day 1 Part 1: {}", day1.part1(&day1_data));
    println!("Day 1 Part 2: {}", day1.part2(&day1_data));

    let day2 = Day2{};
    let day2_data = read_input("inputs/day2.txt".to_string());
    println!("Day 2 Part 1: {}", day2.part1(&day2_data));
    println!("Day 2 Part 2: {}", day2.part2(&day2_data));

    let day3 = Day3{};
    let day3_data = read_input("inputs/day3.txt".to_string());
    println!("Day 3 Part 1: {}", day3.part1(&day3_data));
    println!("Day 3 Part 2: {}", day3.part2(&day3_data));
}
