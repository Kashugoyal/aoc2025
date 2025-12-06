use crate::solution::Solution;

pub struct Day5;

fn parse_day5_input(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut iter = input.lines();
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    let mut isrange = true;
    while let Some(line) = iter.next() {
        if line.trim().is_empty() {
            isrange = false;
            continue;
        }
        if isrange {
            let parts: Vec<&str> = line.trim().split('-').collect();
            let start: i64 = parts[0].parse().unwrap();
            let end: i64 = parts[1].parse().unwrap();
            ranges.push((start, end));
        } else {
            let id: i64 = line.trim().parse().unwrap();
            ids.push(id);
        }
    }
    (ranges, ids)
}

fn merge_intervals(ranges: &mut Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    if ranges.is_empty() {
        return vec![];
    }
    ranges.sort_by_key(|k| k.0);
    let mut merged = vec![ranges[0]];
    for &range in ranges.iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if last.1 >= range.0 {
            last.1 = last.1.max(range.1);
        } else {
            merged.push(range);
        }
    }
    merged
}

impl Solution for Day5 {
    fn part1(&self, input: &str) -> i64 {
        let (mut ranges, ids) = parse_day5_input(input);
        // sort and merge intervals
        let merged_ranges = merge_intervals(&mut ranges);
        let mut fresh_count = 0;
        for id in ids {
            for (start, end) in &merged_ranges {
                if *start <= id && id <= *end {
                    fresh_count += 1;
                }
            }
        }
        fresh_count
    }

    fn part2(&self, input: &str) -> i64 {
        let (mut ranges, _) = parse_day5_input(input);
        // sort and merge intervals
        let merged_ranges = merge_intervals(&mut ranges);
        let mut count = 0;
        for (start, end) in &merged_ranges {
            count += end - start + 1;
        }
        count as i64
    }
}

#[cfg(test)]
mod day5_tests {
    use crate::{day5::Day5, solution::Solution};

    #[test]
    fn test_day5_part1() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let day5 = Day5;
        let result = day5.part1(input);
        assert_eq!(result, 3, "Expected 3 but got {}", result); // Replace 0 with the expected result
    }

    #[test]
    fn test_day5_part2() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let day5 = Day5;
        let result = day5.part2(input);
        assert_eq!(result, 14, "Expected 14 but got {}", result);
    }
}
