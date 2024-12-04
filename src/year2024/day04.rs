fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut strings = Vec::new();

    for line in input.lines() {
        strings.push(line.chars().collect());
    }

    strings
}

const SEARCH_WORD: &str = "XMAS";

fn search(strings: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let mut count = 0;

    //Search up
    let mut word = String::new();
    for i in 0..SEARCH_WORD.len() {
        if row < i {
            break;
        }
        word.push(strings[row - i][col]);
    }
    if word == SEARCH_WORD {
        count += 1;
    }

    // Search down
    let mut word = String::new();
    for i in 0..SEARCH_WORD.len() {
        if row + i >= strings.len() {
            break;
        }
        word.push(strings[row + i][col]);
    }
    if word == SEARCH_WORD {
        count += 1;
    }

    // Search left
    let mut word = String::new();
    for i in 0..SEARCH_WORD.len() {
        if col < i {
            break;
        }
        word.push(strings[row][col - i]);
    }
    if word == SEARCH_WORD {
        count += 1;
    }

    // Search right
    let mut word = String::new();
    for i in 0..SEARCH_WORD.len() {
        if col + i >= strings[row].len() {
            break;
        }
        word.push(strings[row][col + i]);
    }
    if word == SEARCH_WORD {
        count += 1;
    }

    // Search up+right diagonal
    let mut word = String::new();
    for i in 0..SEARCH_WORD.len() {
        if row < i || col + i >= strings[row].len() {
            break;
        }
        word.push(strings[row - i][col + i]);
    }
    if word == SEARCH_WORD {
        count += 1;
    }

    // Search up+left diagonal
    let mut word = String::new();
    for i in 0..SEARCH_WORD.len() {
        if row < i || col < i {
            break;
        }
        word.push(strings[row - i][col - i]);
    }
    if word == SEARCH_WORD {
        count += 1;
    }

    // Search down+right diagonal
    let mut word = String::new();
    for i in 0..SEARCH_WORD.len() {
        if row + i >= strings.len() || col + i >= strings[row].len() {
            break;
        }
        word.push(strings[row + i][col + i]);
    }
    if word == SEARCH_WORD {
        count += 1;
    }

    // Search down+left diagonal
    let mut word = String::new();
    for i in 0..SEARCH_WORD.len() {
        if row + i >= strings.len() || col < i {
            break;
        }
        word.push(strings[row + i][col - i]);
    }
    if word == SEARCH_WORD {
        count += 1;
    }

    count
}

pub fn solve_part1(input: &str) -> i32 {
    let strings = parse_input(input);
    let row_count = strings.len();
    let col_count = strings[0].len();

    let mut count = 0;

    for row in 0..row_count {
        for col in 0..col_count {
            if strings[row][col] == SEARCH_WORD.chars().nth(0).unwrap() {
                count += search(&strings, row, col);
            }
        }
    }

    count
}

pub fn solve_part2(input: &str) -> i32 {
    18
}
