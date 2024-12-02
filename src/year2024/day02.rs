use regex::Regex;

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();

    for line in input.lines() {
        if let Some(cap) = re.captures(line) {
            left.push(cap[1].parse::<i32>().unwrap());
            right.push(cap[2].parse::<i32>().unwrap());
        }
    }
    (left, right)
}

pub fn solve_part1(input: &str) -> i32 {
    2
}

pub fn solve_part2(input: &str) -> i32 {
    2
}
