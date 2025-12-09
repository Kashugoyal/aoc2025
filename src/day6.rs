use crate::solution::Solution;

pub struct Day6;


impl Solution for Day6 {
    fn part1(&self, input: &str) -> i64 {
        let mut data = vec![];
        for line in input.lines() {
            let items: Vec<&str> = line.split_whitespace().collect();
            data.push(items);
        }
        let rows = data.len();
        let mut result: i64 = 0;
        for i in 0..data[0].len() {
            match data[rows-1][i] {
               "*" => {
                let mut value: i64 = 1;
                for j in 0..rows-1 {
                    value *= data[j][i].parse::<i64>().unwrap();
                }
                result += value;
               }
               "+" => {
                let mut value: i64 = 0;
                for j in 0..rows-1 {
                    value += data[j][i].parse::<i64>().unwrap();
                }
                result += value;
               }
               _ => {
                panic!("unexpected value");
               }
            }
        }
        result
    }

    fn part2(&self, input: &str) -> i64 {
        let mut data = vec![];
        for line in input.lines() {
            let items: Vec<char> = line.chars().collect();
            data.push(items);
        }
        let mut result = 0;
        let mut product: i64 = 1;
        let mut sum: i64 = 0;
        for i in (0..data[0].len()).rev() {
            let mut number: String = String::from("");
            for j in 0..data.len()-1 {
                number.push(data[j][i]);
            }
            if number.trim().is_empty() {
                continue;
            }
            product *= number.trim().parse::<i64>().unwrap();
            sum += number.trim().parse::<i64>().unwrap();
            match data[data.len()-1][i] {
                '*' => {
                    result += product;
                    product = 1;
                    sum = 0;
                } 
                '+' => {
                    result += sum;
                    product = 1;
                    sum = 0;
                }
                ' ' => {}
                _ => panic!("unexpected")
            }
        }
        result
    }
}

#[cfg(test)]
mod day6_tests {
    use crate::{day6::Day6, solution::Solution};
    #[test]
    fn test_day6_part1() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let day6 = Day6;
        let result = day6.part1(input);
        assert_eq!(result, 4277556, "Expected 4277556 but got {}", result); // Replace 0 with the expected result
    }   

    #[test]
    fn test_day6_part2() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let day6 = Day6;
        let result = day6.part2(input);
        assert_eq!(result, 3263827, "Expected 3263827 but got {}", result); // Replace 0 with the expected result
    }
}