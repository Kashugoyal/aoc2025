struct Day2;

fn parse_day2_input(input: &str) -> Vec<(i64, i64)> {
    let mut output: Vec<(i64, i64)> = vec![];
    input.split(',').for_each(|line| {
        let range = line.split('-').collect::<Vec<&str>>();
        let start = range[0].trim().parse::<i64>().unwrap();
        let end = range[1].trim().parse::<i64>().unwrap();
        output.push((start, end));
    });
    output
}

impl Solution for Day2 {
    fn part1(&self, input: &str) -> i64 {
        let data = parse_day2_input(input);

        let mut result = 0;
        for d in data.iter() {
            let mut low = d.0;
            let mut high = d.1;
            // find num digits in low
            let mut num_low_digits = low.to_string().len();
            // if digits is odd, then bump low to next smallest even digit number
            if num_low_digits % 2 != 0 {
                low = 10i64.pow((num_low_digits) as u32);
                if low > high {
                    continue;
                }
                num_low_digits += 1;
            }

            let num_high_digits = high.to_string().len();
            // if digits in high is odd, then reduce high to next smallest even digit number
            if num_high_digits % 2 != 0 {
                high = 10i64.pow((num_high_digits - 1) as u32) - 1;
                if low > high {
                    continue;
                }
            };
            // get first half digits of low and check if in range
            // if no, continue
            // else add to result and increment low by 1 and repeat
            let half_digits = num_low_digits / 2;
            while low <= high {
                let half_low = low / 10i64.pow(half_digits as u32);
                let test = half_low * 10i64.pow(half_digits as u32) + half_low;
                if test > high {
                    break;
                }
                if test >= low {
                    // add to result
                    // println!("Found invalid id: {} in range {}-{}", test, low, high);
                    result += test;
                }
                low = (half_low + 1) * 10i64.pow(half_digits as u32);
            }
        }
        result
    }

    fn part2(&self, input: &str) -> i64 {
        let data = parse_day2_input(input);

        let mut result = 0;
        for d in data.iter() {
            let low = d.0;
            let high = d.1;
            for num in low..=high {
                let mut is_invalid = false;
                let num_digits = num.to_string().len();
                for i in (1..=(num_digits / 2)).rev() {
                    if is_invalid {
                        break;
                    }
                    if num_digits % i != 0 {
                        continue;
                    }
                    let splitter = |num: i64, part_size: usize| -> bool {
                        let mut parts: Vec<i64> = vec![];
                        let mut n = num;
                        while n > 0 {
                            let part = n % 10i64.pow(part_size as u32);
                            n = n / 10i64.pow(part_size as u32);
                            parts.push(part);
                        }
                        parts.iter().all(|x| *x == parts[0])
                    };
                    if splitter(num, i) {
                        // println!("Found invalid id: {} in range {}-{}", num, low, high);
                        result += num;
                        is_invalid = true;
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod day2_tests {
    use super::Solution;

    #[test]
    fn test_day2_part1() {
        let day2 = super::Day2 {};
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let result = day2.part1(input);
        let expected = 1227775554;
        assert_eq!(
            result, expected,
            "Expected result {} but got {}",
            expected, result
        );
    }
    #[test]
    fn test_day2_part2() {
        let day2 = super::Day2 {};
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let result = day2.part2(input);
        let expected = 4174379265;
        assert_eq!(
            result, expected,
            "Expected result {} but got {}",
            expected, result
        );
    }
}
