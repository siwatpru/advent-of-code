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

fn is_row_increasing(row: &[i32]) -> bool {
    row.windows(2).all(|w| w[0] < w[1] && w[1] - w[0] <= 3)
}

fn is_row_decreasing(row: &[i32]) -> bool {
    row.windows(2).all(|w| w[0] > w[1] && w[0] - w[1] <= 3)
}

pub fn solve_part1(input: &str) -> usize {
    let rows = parse_input(input);

    let mut count: usize = 0;
    for row in rows {
        let is_increasing = is_row_increasing(&row);
        let is_decreasing = is_row_decreasing(&row);

        if is_increasing || is_decreasing {
            count += 1;
        }
    }

    count
}

pub fn solve_part2(input: &str) -> usize {
    let rows = parse_input(input);

    let mut count: usize = 0;
    for row in rows {
        let is_increasing = is_row_increasing(&row);
        let is_decreasing = is_row_decreasing(&row);

        let can_be_increasing = (0..row.len()).any(|i| {
            let mut modified_row = row.clone();
            modified_row.remove(i);
            is_row_increasing(&modified_row)
        });

        let can_be_decreasing = (0..row.len()).any(|i| {
            let mut modified_row = row.clone();
            modified_row.remove(i);
            is_row_decreasing(&modified_row)
        });

        if is_increasing || is_decreasing || can_be_increasing || can_be_decreasing {
            count += 1;
        }
    }

    count
}
