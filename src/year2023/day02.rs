use regex::Regex;

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
            let mut current_turn = Turn {
                red: 0,
                blue: 0,
                green: 0,
            };

            for turn_cap in turn_re.captures_iter(turn.trim()) {
                let score: u32 = turn_cap[1].parse().unwrap();
                let color = &turn_cap[2];

                match color {
                    "red" => current_turn.red += score,
                    "blue" => current_turn.blue += score,
                    "green" => current_turn.green += score,
                    _ => panic!("Unexpected color: {}", color),
                }
            }

            game.turns.push(current_turn);
        }

        games.push(game);
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
        for turn in &game.turns {
            if turn.red > max_red || turn.green > max_green || turn.blue > max_blue {
                is_possible = false;
                // println!("Game {}: impossible because {:?}", game.id, turn);
                break;
            }
        }
        if is_possible {
            // println!("Game {}: ok", game.id);
            output += game.id;
        }
    }
    output
}

pub fn solve_part2(input: &str) -> u32 {
    let mut output = 0;

    let games = parse_input(input);

    for game in games {
        let mut min_red = 1;
        let mut min_green = 1;
        let mut min_blue = 1;

        for turn in &game.turns {
            min_red = min_red.max(turn.red);
            min_green = min_green.max(turn.green);
            min_blue = min_blue.max(turn.blue);
        }

        let power = min_red * min_green * min_blue;

        output += power;
    }

    output
}
