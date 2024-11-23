use regex::Regex;
// use std::collections::HashMap;

#[derive(Debug)]
struct Turn {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    turns: Vec<Turn>,
}

fn parse_input(input: &str) -> Vec<Game> {
    // Capture Game x: (score team)
    let re =
        Regex::new(r"Game\s+(\d+):\s*((?:\d+\s+\w+(?:,\s*)?)+(?:;\s*(?:\d+\s+\w+(?:,\s*)?)+)*)")
            .unwrap();

    let mut games: Vec<Game> = vec![];

    for cap in re.captures_iter(input) {
        let turns_data = &cap[2];

        let mut game = Game {
            id: cap[1].parse().unwrap(),
            turns: Vec::new(),
        };

        let turn_re = Regex::new(r"(\d+)\s+(\w+)").unwrap();
        for turn in turns_data.split(";") {
            for turn_cap in turn_re.captures_iter(turn.trim()) {
                let score: u32 = turn_cap[1].parse().unwrap();
                let color = &turn_cap[2];

                let turn = Turn {
                    red: if color == "red" { score } else { 0 },
                    blue: if color == "blue" { score } else { 0 },
                    green: if turn == "green" { score } else { 0 },
                };

                game.turns.push(turn)
            }
        }

        games.push(game)
    }

    games
}

fn sum_turn(turns: &Vec<Turn>) -> Turn {
    let mut sum = Turn {
        red: 0,
        blue: 0,
        green: 0,
    };

    for turn in turns {
        sum.red += turn.red;
        sum.green += turn.green;
        sum.blue += turn.blue;
    }

    sum
}

pub fn solve_part1(input: &str) -> u32 {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut output = 0;

    let games = parse_input(input);

    for game in games {
        let sum = sum_turn(&game.turns);
        println!("{}, {:?}", game.id, sum);

        if sum.red > max_red || sum.green > max_green || sum.blue > max_blue {
            continue;
        } else {
            output += game.id;
        }
    }
    output
}

pub fn solve_part2(_input: &str) -> u32 {
    0
}
