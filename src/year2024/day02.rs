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

pub fn solve_part1(input: &str) -> usize {
    let rows = parse_input(input);

    let mut count: usize = 0;
    for row in rows {
        let is_increasing = row.windows(2).all(|w| w[0] < w[1] && w[1] - w[0] <= 3);
        let is_decreasing = row.windows(2).all(|w| w[0] > w[1] && w[0] - w[1] <= 3);

        if is_increasing || is_decreasing {
            count += 1;
        }
    }

    count
}

pub fn solve_part2(_input: &str) -> i32 {
    10
}
