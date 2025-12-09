use crate::solution::Solution;

pub struct Day7;

impl Solution for Day7 {
    fn part1(&self, input: &str) -> i64 {
        let mut data: Vec<Vec<char>> = vec![];
        for line in input.lines() {
            data.push(line.chars().collect());
        }
        let beam_start_index = data[0].iter().position(|&c| c == 'S').unwrap();
        let mut beam_indices = vec![beam_start_index];
        let mut result = 0;
        for i in 2..data.len() {
            // get splitter indices
            let beams = beam_indices.clone();
            // get matching indices
            beam_indices.clear();
            for &beam_index in &beams {
                if data[i][beam_index] == '^' {
                    result += 1;
                    let (split_left, split_right) = (beam_index - 1, beam_index + 1);
                    if beam_indices.len() == 0 || beam_indices[beam_indices.len() - 1] != split_left
                    {
                        beam_indices.push(split_left);
                    }
                    if split_right < data[0].len() {
                        beam_indices.push(split_right);
                    }
                } else if beam_indices.len() == 0
                    || beam_indices[beam_indices.len() - 1] != beam_index
                {
                    beam_indices.push(beam_index);
                }
            }
        }
        result
    }

    fn part2(&self, input: &str) -> i64 {
        let mut data: Vec<Vec<char>> = vec![];
        for line in input.lines() {
            data.push(line.chars().collect());
        }
        let beam_start_index = data[0].iter().position(|&c| c == 'S').unwrap();
        let mut permutations = vec![0; data[0].len()];
        let mut beam_indices = vec![beam_start_index];
        permutations[beam_start_index] = 1;
        for i in 2..data.len() {
            // get splitter indices
            let beams = beam_indices.clone();
            // get matching indices
            beam_indices.clear();
            for &beam_index in &beams {
                if data[i][beam_index] == '^' {
                    let (split_left, split_right) = (beam_index - 1, beam_index + 1);
                    permutations[split_left] += permutations[beam_index];
                    permutations[split_right] += permutations[beam_index];
                    if beam_indices.len() == 0 || beam_indices[beam_indices.len() - 1] != split_left
                    {
                        beam_indices.push(split_left);
                    }
                    if split_right < data[0].len() {
                        beam_indices.push(split_right);
                    }
                    permutations[beam_index] = 0;
                } else if beam_indices.len() == 0
                    || beam_indices[beam_indices.len() - 1] != beam_index
                {
                    beam_indices.push(beam_index);
                }
            }
        }
        permutations.iter().sum()
    }
}

#[cfg(test)]
mod day7_tests {
    use crate::{day7::Day7, solution::Solution};
    #[test]
    fn test_day7_part1() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let day7 = Day7;
        let result = day7.part1(input);
        assert_eq!(result, 21, "Expected 21 but got {}", result); // Replace 0 with the expected result
    }

    #[test]
    fn test_day7_part2() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let day7 = Day7;
        let result = day7.part2(input);
        assert_eq!(result, 40, "Expected 40 but got {}", result); // Replace 0 with the expected result
    }
}

// 1, 2, 3, 4, 6, 7
// 2, 4, 7,
