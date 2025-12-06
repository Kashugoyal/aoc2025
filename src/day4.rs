use crate::solution::Solution;

pub struct Day4;

    fn parse_day4_input(input: &str) -> Vec<Vec<char>> {
        let mut output: Vec<Vec<char>> = Vec::new();
        for line in input.lines() {
            let row: Vec<char> = line.trim().chars().collect();
            output.push(row);
            // println!("{:?}", line);
        }
        output
    }

    fn get_neighbors(x: i32, y: i32, grid: &Vec<Vec<char>>) -> i32 {
        // let mut neighbors = Vec::new();
        let mut neighbors = 0;
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for (dx, dy) in directions.iter() {
            let new_x = x + dx;
            let new_y = y + dy;
            if new_x >= 0
                && new_x < grid.len() as i32
                && new_y >= 0
                && new_y < grid[0].len() as i32
                && grid[new_x as usize][new_y as usize] == '@'
            {
                neighbors += 1;
            }
        }
        neighbors
    }

    impl Solution for Day4 {
        fn part1(&self, input: &str) -> i64 {
            let data = parse_day4_input(input);
            let mut result = 0;
            for i in 0..data.len() {
                for j in 0..data[0].len() {
                    if data[i][j] == '.' {
                        // print!(".");
                        continue;
                    }
                    let val = get_neighbors(i as i32, j as i32, &data);
                    if val < 4 {
                        // print!("x");
                        result += 1;
                    } else {
                        // print!("@");
                    }
                }
                // println!();
            }
            result as i64
        }

        fn part2(&self, input: &str) -> i64 {
            let mut data = parse_day4_input(input);
            //  top level loop for keep removing rolls
            let mut did_remove = true;
            let mut result = 0;
            while did_remove {
                did_remove = false;
                let mut locations = Vec::new();
                for i in 0..data.len() {
                    for j in 0..data[0].len() {
                        if data[i][j] == '.' {
                            // print!(".");
                            continue;
                        }
                        let val = get_neighbors(i as i32, j as i32, &data);
                        if val < 4 {
                            // print!("x");
                            result += 1;
                            locations.push((i, j));
                            did_remove = true;
                        } else {
                            // print!("@");
                        }
                    }
                    // println!();
                }
                locations.iter().for_each(|(i, j)| {
                    data[*i][*j] = '.';
                });
                // println!("---");
            }
            result as i64
        }
    }

#[cfg(test)]
mod day4_tests {
    use crate::{day4::Day4, solution::Solution};


    #[test]
    fn test_day4_part1() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let day4 = Day4;
        let result = day4.part1(input);
        assert_eq!(result, 13, "Expected 13 but got {}", result); // Replace 0 with the expected result
    }

    #[test]
    fn test_day4_part2() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let day4 = Day4;
        let result = day4.part2(input);
        assert_eq!(result, 43, "Expected 43 but got {}", result);
    }
}
