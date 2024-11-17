use regex::Regex;

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
            _ => digits[0] * 10 + digits[digits.len() - 1], // Use the first and last digits
        };

        sum += combined;
    }

    sum
}
// Your solution logic here
pub fn solve_part2(_input: &str) -> usize {
    84
}
