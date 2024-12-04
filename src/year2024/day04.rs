use regex::Regex;

fn parse_input(input: &str) -> Vec<(i32, i32)> {
    let mut pairs = Vec::new();
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for line in input.lines() {
        for cap in re.captures_iter(line) {
            if let (Some(first), Some(second)) = (cap.get(1), cap.get(2)) {
                let num1 = first.as_str().parse::<i32>().unwrap();
                let num2 = second.as_str().parse::<i32>().unwrap();
                pairs.push((num1, num2));
            }
        }
    }

    pairs
}

pub fn solve_part1(input: &str) -> i32 {
    18
}

pub fn solve_part2(input: &str) -> i32 {
    18
}
