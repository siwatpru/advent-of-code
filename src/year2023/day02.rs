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

pub fn solve_part1(input: &str) -> u32 {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut output = 0;

    let games = parse_input(input);

    for game in games {
        let mut is_possible = true;
        for turn in game.turns {
            if turn.red > max_red || turn.green > max_green || turn.blue > max_blue {
                is_possible = false;
                println!("Game {}: impossible because {:?}", game.id, turn);
                break;
            }
        }
        if is_possible {
            println!("Game {}: ok", game.id);
            output += game.id;
        }
    }
    output
}

pub fn solve_part2(_input: &str) -> u32 {
    0
}
