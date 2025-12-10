use crate::solution::Solution;
use ordered_float::NotNan;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::default::Default;
use std::usize;

pub struct Day8 {
    connections: i32,
}

impl Default for Day8 {
    fn default() -> Self {
        Day8 { connections: 1000 }
    }
}

fn parse_day8_input(input: &str) -> Vec<Vec<i64>> {
    let mut output = vec![];
    input
        .lines()
        .for_each(|line| output.push(line.split(',').map(|f| f.parse::<i64>().unwrap()).collect()));
    output
}

fn euclidean_distance(a: &Vec<i64>, b: &Vec<i64>) -> f64 {
    let val = (a[0] - b[0]).pow(2) + (a[1] - b[1]).pow(2) + (a[2] - b[2]).pow(2);
    let fval = val as f64;
    fval.sqrt()
}

fn find_parent(x: usize, parents: &mut Vec<usize>) -> usize {
    let mut i = x;
    while parents[i] != i {
        let old = i;
        i = parents[i];
        parents[old] = parents[i];
    }
    i
}

impl Solution for Day8 {
    fn part1(&self, input: &str) -> i64 {
        // parse input
        let data = parse_day8_input(input);
        let mut parents: Vec<usize> = (0..data.len()).collect();

        let mut distances = BinaryHeap::new();
        for i in 0..data.len() {
            for j in i + 1..data.len() {
                let distance = euclidean_distance(&data[i], &data[j]);
                distances.push(Reverse((NotNan::new(distance).unwrap(), i, j)));
            }
        }

        for _ in 0..self.connections {
            let item = distances.pop().unwrap();
            let a = item.0.1;
            let b = item.0.2;

            let parent_a = find_parent(a, &mut parents);
            let parent_b = find_parent(b, &mut parents);
            if parent_a != parent_b {
                parents[parent_a] = parent_b;
            }
        }

        // get the top three
        let mut counts = vec![0; data.len()];
        for i in 0..counts.len() {
            let p = find_parent(i, &mut parents);
            counts[p] += 1;
        }

        counts.sort();
        counts.reverse();
        counts[0] * counts[1] * counts[2]
    }

    fn part2(&self, input: &str) -> i64 {
        let data = parse_day8_input(input);
        let mut parents: Vec<usize> = (0..data.len()).collect();
        let mut sizes: Vec<usize> = vec![1; data.len()];

        let mut distances = BinaryHeap::new();
        for i in 0..data.len() {
            for j in i + 1..data.len() {
                let distance = euclidean_distance(&data[i], &data[j]);
                distances.push(Reverse((NotNan::new(distance).unwrap(), i, j)));
            }
        }

        let mut result = 0;
        while distances.len() != 0 {
            let item = distances.pop().unwrap();
            let a = item.0.1;
            let b = item.0.2;

            let mut parent_a = find_parent(a, &mut parents);
            let mut parent_b = find_parent(b, &mut parents);
            if parent_a == parent_b {
                continue;
            }
            if sizes[parent_a] <= sizes[parent_b] {
                std::mem::swap(&mut parent_a, &mut parent_b);
            }
            parents[parent_a] = parent_b;
            sizes[parent_b] += sizes[parent_a];

            if sizes[parent_b] == data.len() {
                result = data[a][0] * data[b][0];
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod day8_tests {
    use crate::{day8::Day8, solution::Solution};
    #[test]
    fn test_day8_part1() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let day8 = Day8 { connections: 10 };
        let result = day8.part1(input);
        assert_eq!(result, 40, "Expected 40 but got {}", result); // Replace 0 with the expected result
    }

    #[test]
    fn test_day8_part2() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let day8 = Day8::default();
        let result = day8.part2(input);
        assert_eq!(result, 25272, "Expected 25272 but got {}", result); // Replace 0 with the expected result
    }
}
