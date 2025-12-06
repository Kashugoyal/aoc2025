struct Day3;

fn parse_day3_input(input: &str) -> Vec<Vec<i32>> {
    let mut output = vec![];

    input
        .lines()
        .for_each(|line| 
            output.push(
            line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect()
        ));
    output
}


impl Solution for Day3 {
    fn part1(&self, input: &str) -> i64 {
        let data = parse_day3_input(input);
        let mut result = 0;
        for row in data.iter() {
            let mut first = 0;
            let mut second = 0;
            for i in 1..row.len()  {
                if row[i-1] > first {
                    first = row[i-1];
                    second = row[i];
                } else if row[i] >= second {
                    second = row[i];
                }
            }
            result += first*10 + second;
            // println!("{:?}", row);
            // println!("Found top two: {} and {}, partial sum: {}", first, second, result);
        }
        result as i64
    }

    fn part2(&self, input: &str) -> i64 {
        let data = parse_day3_input(input);
        let mut result = 0;
        for row in data.iter() {
            let mut number = row.to_vec()[0..12].to_vec();
            for digit in 0..=row.len()-12 {
                for i in 0..12 {
                    if number[i] < row[digit+i] {
                        number[i..].copy_from_slice(&row.to_vec()[(digit+i)..digit+12]);
                        break;
                    }
                }
            }
            result += number.iter().fold(0, |acc:i64, &x| acc * 10 + x as i64) as i64;
            // println!("{:?}", row);
            // println!("Found number: {:?}", number.iter().fold(0, |acc:i64, &x| acc * 10 + x as i64) as i64);
        }
        result
    }
}

#[cfg(test)]
mod day3_tests {
    use super::*;

    #[test]
    fn test_day3_part1() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let day3 = Day3;
        let result = day3.part1(input);
        assert_eq!(result, 357, "Expected 357 but got {}", result);
    }

    #[test]
    fn test_day3_part2() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let day3 = Day3;
        let result = day3.part2(input);
        assert_eq!(result, 3121910778619, "Expected 3121910778619 but got {}", result);
    }
}
