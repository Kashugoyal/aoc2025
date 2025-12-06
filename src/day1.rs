use crate::solution::Solution;

pub struct Day1;

fn parse_day1_input(input: &str) -> Vec<(char, i64)> {
    input
        .lines()
        .map(|line| {
            (
                line.chars().next().unwrap(),
                line[1..].trim().parse::<i64>().unwrap(),
            )
        })
        .collect()
}

impl Solution for Day1 {
    fn part1(&self, input: &str) -> i64 {
        let data = parse_day1_input(input);
        let mut position = 50;
        let dial_size = 100;
        let mut result = 0;
        for (direction, steps) in data {
            let multiplier;
            match direction {
                'R' => multiplier = 1,
                'L' => multiplier = -1,
                _ => panic!("Invalid direction"),
            }
            position = (dial_size + position + (multiplier * steps)) % dial_size;
            if position == 0 {
                result += 1;
            }
        }
        result
    }

    // https://bsky.app/profile/did:plc:hzlj6uyb2xjvwh7qaz3tunry/post/3m6yxvxgkgc2u
    fn part2(&self, input: &str) -> i64 {
        let data = parse_day1_input(input);
        let mut position = 50;
        let dial_size = 100;
        let mut result = 0;
        for (direction, count) in data {
            // mirror the dial if turning left
            match direction {
                'L' => position = (dial_size - position) % dial_size,
                'R' => {}
                _ => panic!("Invalid direction"),
            }
            position += count;
            result += position / dial_size;
            position = position % dial_size;

            // mirror back if turned left
            match direction {
                'L' => position = (dial_size - position) % dial_size,
                'R' => {}
                _ => panic!("Invalid direction"),
            }
        }
        result
    }
}

#[cfg(test)]
mod day1_tests {
    use crate::{day1::Day1, solution::Solution};

    const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        let day1 = Day1 {};
        println!("Testing Part 1 with input:\n{}", TEST_INPUT);
        assert_eq!(
            day1.part1(TEST_INPUT),
            3,
            "Part 1 test failed, expected 3, got {}",
            day1.part1(TEST_INPUT)
        );
    }
    #[test]
    fn test_part2() {
        let day1 = Day1 {};
        assert_eq!(
            day1.part2(TEST_INPUT),
            6,
            "Part 2 test failed, expected 6, got {}",
            day1.part2(TEST_INPUT)
        );
    }
}
