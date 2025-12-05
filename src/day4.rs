struct Day4;

fn parse_day4_input(input: &str) -> Vec<Vec<String>> {
    let mut output: Vec<Vec<String>> = Vec::new();
    for line in input.lines() {
        let row: Vec<String> = line.trim().chars().map(|c| c.to_string()).collect();
        output.push(row);
        println!("{:?}", line);
    }
    output
}

fn get_neighbors(x: i32, y: i32, grid: &Vec<Vec<String>>) -> i32 {
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
            && grid[new_x as usize][new_y as usize] == "@"
        {
            neighbors += 1;
        }
    }
    neighbors
}

impl Day4 {
    pub fn part1(&self, input: &str) -> i64 {
        // Implementation for part 1
        let data = parse_day4_input(input);
        let mut result = 0;
        for i in 0..data.len() {
            for j in 0..data[0].len() {
                if data[i][j] == "." {
                    print!(".");
                    continue;
                }
                let val = get_neighbors(i as i32, j as i32, &data);
                if val < 4 {
                    print!("x");
                    result += 1;
                } else {
                    print!("@");
                }
            }
            println!();
        }
        result as i64
    }

    pub fn part2(&self, _: &str) -> i64 {
        // Implementation for part 2
        0
    }
}

#[cfg(test)]
mod day4_tests {
    use super::*;

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
        assert_eq!(result, 0); // Replace 0 with the expected result
    }
}
