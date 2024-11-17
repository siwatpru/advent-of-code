use regex::Regex;
use std::collections::HashMap;

pub fn solve_part1(input: &str) -> i32 {
    let re = Regex::new(r".*?(\d)").unwrap();
    let mut sum: i32 = 0;

    for line in input.lines() {
        let mut digits = Vec::new();

        // Capture all digits
        for cap in re.captures_iter(line) {
            digits.push(cap[1].parse::<i32>().unwrap());
        }

        // Handle cases where we have fewer than 2 digits
        let combined = match digits.len() {
            0 => {
                println!("No digits in line: {}", line);
                continue;
            }
            _ => digits[0] * 10 + digits[digits.len() - 1],
        };

        sum += combined;
    }

    sum
}

pub fn solve_part2(input: &str) -> i32 {
    let words_to_digits: HashMap<&str, i32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .cloned()
    .collect();

    let mut sum: i32 = 0;

    // Helper function to parse concatenated words into digits
    fn parse_digits(s: &str, words_to_digits: &HashMap<&str, i32>) -> Vec<i32> {
        let mut digits = Vec::new();
        let mut start = 0;
        while start < s.len() {
            // If can parse into a digit, add to the output
            if s.as_bytes()[start].is_ascii_digit() {
                let digit = (s.as_bytes()[start] - b'0') as i32;
                digits.push(digit);
                start += 1;
                continue;
            }

            for (word, &digit) in words_to_digits.iter() {
                if s[start..].starts_with(word) {
                    digits.push(digit);
                    break;
                }
            }

            start += 1;
        }
        digits
    }

    for line in input.lines() {
        let mut digits = Vec::new();

        // Extract word digits
        digits.extend(parse_digits(line, &words_to_digits));

        // Handle cases where we have fewer than 2 digits
        let combined = match digits.len() {
            0 => {
                println!("No digits in line: {}", line);
                continue;
            }
            _ => digits[0] * 10 + digits[digits.len() - 1],
        };

        sum += combined;
    }

    sum
}
