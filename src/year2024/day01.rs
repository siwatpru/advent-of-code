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
    let (mut left, mut right) = parse_input(input);

    left.sort();
    right.sort();

    let mut sum: i32 = 0;
    for (index, l) in left.iter().enumerate() {
        let r = right
            .get(index)
            .expect("left and right should be the same size");
        let distance = (r - l).abs();
        sum += distance;
    }

    sum
}

pub fn solve_part2(input: &str) -> i32 {
    let (left, right) = parse_input(input);

    let mut sum: i32 = 0;
    for l in left.iter() {
        let count = right.iter().filter(|&&item| item == *l).count() as i32;
        sum += count * l;
    }

    sum
}
