use std::fs;
use crate::{Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let input_path = "input/day1.txt";
    let input = fs::read_to_string(input_path).expect("Should be able to read the file contents");
    (Solution::from(solve_first(&input)), Solution::from(solve_second(&input)))
}

pub fn solve_first(input: &String) -> String {
    const RADIX: u32 = 10;

    let ans: u32 = input.lines()
        .map(|line| {
            let mut numbers: Vec<u32> = Vec::new();

            line.chars().for_each(|c| {
                if c.is_numeric() {
                    numbers.push(c.to_digit(RADIX).unwrap());
                }
            });

            return format!("{}{}",
                           numbers.first().expect("There should at least be one number in the line"),
                           numbers.last().expect("There should at least be one number in the line"))
                .parse::<u32>().unwrap();
        })
        .sum();

    return ans.to_string();
}

pub fn solve_second(input: &String) -> String {
    const NUMS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    const RADIX: u32 = 10;

    let ans: u32 = input.lines()
        .map(|line| {
            let mut numbers: Vec<u32> = Vec::new();

            let mut line_so_far = String::new();
            line.chars().for_each(|c| {
                if c.is_numeric() {
                    numbers.push(c.to_digit(RADIX).unwrap());
                }

                line_so_far.push(c);
                for num in NUMS {
                    if line_so_far.ends_with(num) {
                        numbers.push((NUMS.iter().position(|&n| n == num).unwrap() + 1) as u32);
                    }
                }
            });

            return format!("{}{}",
                           numbers.first().expect("There should at least be one number in the line"),
                           numbers.last().expect("There should at least be one number in the line"))
                .parse::<u32>().unwrap();
        })
        .sum();

    return ans.to_string();
}