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

fn parse_input_with_do_dont(input: &str) -> Vec<(i32, i32)> {
    let mut pairs = Vec::new();
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))").unwrap();

    let start_command = "do()";
    let stop_command = "don't()";

    let mut is_stop = false;

    for line in input.lines() {
        for cap in re.captures_iter(line) {
            let matched_str = cap.get(0).unwrap().as_str();
            if matched_str == stop_command {
                is_stop = true;
            } else if matched_str == start_command {
                is_stop = false;
            } else if !is_stop {
                if let Some(num1) = cap.get(2) {
                    if let Some(num2) = cap.get(3) {
                        let num1 = num1.as_str().parse::<i32>().unwrap();
                        let num2 = num2.as_str().parse::<i32>().unwrap();
                        pairs.push((num1, num2));
                    }
                }
            }
        }
    }

    pairs
}

pub fn solve_part1(input: &str) -> i32 {
    let pairs = parse_input(input);
    let mut sum = 0;
    for pair in pairs {
        sum += pair.0 * pair.1;
    }
    sum
}

pub fn solve_part2(input: &str) -> i32 {
    let pairs = parse_input_with_do_dont(input);
    let mut sum = 0;
    for pair in pairs {
        sum += pair.0 * pair.1;
    }
    sum
}
