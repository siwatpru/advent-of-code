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

    // Define directions for searching: (row_offset, col_offset)
    let directions: [(isize, isize); 8] = [
        (1, 0),   // down
        (-1, 0),  // up
        (0, 1),   // right
        (0, -1),  // left
        (1, 1),   // down-right diagonal
        (1, -1),  // down-left diagonal
        (-1, 1),  // up-right diagonal
        (-1, -1), // up-left diagonal
    ];

    for (row_offset, col_offset) in directions.iter() {
        let mut word = String::new();
        for i in 0..SEARCH_WORD.len() {
            let new_row = row as isize + i as isize * *row_offset;
            let new_col = col as isize + i as isize * *col_offset;

            // Check bounds
            if new_row < 0
                || new_row >= strings.len() as isize
                || new_col < 0
                || new_col >= strings[new_row as usize].len() as isize
            {
                break;
            }
            word.push(strings[new_row as usize][new_col as usize]);
        }
        if word == SEARCH_WORD {
            count += 1;
        }
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

pub fn solve_part2(_input: &str) -> i32 {
    18
}
