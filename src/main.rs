mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod solution;

use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use solution::Solution;

use std::time::Instant;


fn read_input(file: &str) -> String {
    std::fs::read_to_string(file).expect("Failed to read file")
}

fn time_wrapper<F, R>(func: F) -> (R, std::time::Duration)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let output = func();
    let duration = start.elapsed();
    (output, duration)
}


fn main() {
    println!("Hello, Advent of Code 2025!");

    let day1 = Day1;
    let day1_data = read_input("inputs/day1.txt");
    let (part1_result, part1_duration) = time_wrapper(|| day1.part1(&day1_data));
    println!("Day 1 Part 1 Duration: {:?}, Ans: {},", part1_duration, part1_result);
    let (part2_result, part2_duration) = time_wrapper(|| day1.part2(&day1_data));
    println!("Day 1 Part 2 Duration: {:?}, Ans: {},", part2_duration, part2_result);

    let day2 = Day2;
    let day2_data = read_input("inputs/day2.txt");
    let (part1_result, part1_duration) = time_wrapper(|| day2.part1(&day2_data));
    println!("Day 2 Part 1 Duration: {:?}, Ans: {},", part1_duration, part1_result);
    let (part2_result, part2_duration) = time_wrapper(|| day2.part2(&day2_data));
    println!("Day 2 Part 2 Duration: {:?}, Ans: {},", part2_duration, part2_result);

    let day3 = Day3;
    let day3_data = read_input("inputs/day3.txt");
    let (part1_result, part1_duration) = time_wrapper(|| day3.part1(&day3_data));
    println!("Day 3 Part 1 Duration: {:?}, Ans: {},", part1_duration, part1_result);
    let (part2_result, part2_duration) = time_wrapper(|| day3.part2(&day3_data));
    println!("Day 3 Part 2 Duration: {:?}, Ans: {},", part2_duration, part2_result);

    let day4 = Day4;
    let day4_data = read_input("inputs/day4.txt");
    let (part1_result, part1_duration) = time_wrapper(|| day4.part1(&day4_data));
    println!("Day 4 Part 1 Duration: {:?}, Ans: {},", part1_duration, part1_result);
    let (part2_result, part2_duration) = time_wrapper(|| day4.part2(&day4_data));
    println!("Day 4 Part 2 Duration: {:?}, Ans: {},", part2_duration, part2_result);

    let day5 = Day5;
    let day5_data = read_input("inputs/day5.txt");
    let (part1_result, part1_duration) = time_wrapper(|| day5.part1(&day5_data));
    println!("Day 5 Part 1 Duration: {:?}, Ans: {},", part1_duration, part1_result);
    let (part2_result, part2_duration) = time_wrapper(|| day5.part2(&day5_data));
    println!("Day 5 Part 2 Duration: {:?}, Ans: {},", part2_duration, part2_result);

    let day6 = Day6;
    let day6_data = read_input("inputs/day6.txt");
    let (part1_result, part1_duration) = time_wrapper(|| day6.part1(&day6_data));
    println!("Day 6 Part 1 Duration: {:?}, Ans: {},", part1_duration, part1_result);
    let (part2_result, part2_duration) = time_wrapper(|| day6.part2(&day6_data));
    println!("Day 6 Part 2 Duration: {:?}, Ans: {},", part2_duration, part2_result);

    let day7 = Day7;
    let day7_data = read_input("inputs/day7.txt");
    let (part1_result, part1_duration) = time_wrapper(|| day7.part1(&day7_data));
    println!("Day 7 Part 1 Duration: {:?}, Ans: {},", part1_duration, part1_result);
    let (part2_result, part2_duration) = time_wrapper(|| day7.part2(&day7_data));
    println!("Day 7 Part 2 Duration: {:?}, Ans: {},", part2_duration, part2_result);

    let day8 = Day8::default();
    let day8_data = read_input("inputs/day8.txt");
    let (part1_result, part1_duration) = time_wrapper(|| day8.part1(&day8_data));
    println!("Day 8 Part 1 Duration: {:?}, Ans: {},", part1_duration, part1_result);
    let (part2_result, part2_duration) = time_wrapper(|| day8.part2(&day8_data));
    println!("Day 8 Part 2 Duration: {:?}, Ans: {},", part2_duration, part2_result);
}
