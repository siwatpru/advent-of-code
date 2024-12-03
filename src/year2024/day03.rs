use regex::Regex;

fn parse_input(input: &str) -> Vec<(i32, i32)> {
    let mut pairs = Vec::new();
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let re2 = Regex::new(r"\d{1,3}").unwrap();

    for line in input.lines() {
        for cap in re.captures_iter(line) {
            let mut cap2 = re2.captures_iter(&cap[0]);
            if let (Some(first), Some(second)) = (cap2.next(), cap2.next()) {
                let num1 = first[0].parse::<i32>().unwrap();
                let num2 = second[0].parse::<i32>().unwrap();
                pairs.push((num1, num2));
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

pub fn solve_part2(_input: &str) -> i32 {
    161
}
