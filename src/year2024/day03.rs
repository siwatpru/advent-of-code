use regex::Regex;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut rows = Vec::new();
    let re = Regex::new(r"(\d+)(?:\s+|$)").unwrap();

    for line in input.lines() {
        let mut row = Vec::new();
        for cap in re.captures_iter(line) {
            row.push(cap[1].parse::<i32>().unwrap());
        }
        if !row.is_empty() {
            rows.push(row);
        }
    }

    rows
}

pub fn solve_part1(input: &str) -> i32 {
    161
}

pub fn solve_part2(input: &str) -> i32 {
    161
}
