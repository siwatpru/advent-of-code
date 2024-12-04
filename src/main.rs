use std::env;
mod utils;
mod year2023;
mod year2024;

fn main() {
    // Define available years and days
    let years = vec![(2023, vec![1, 2, 3]), (2024, vec![1, 2, 3, 4])];

    // Read command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // No arguments provided, run all solutions
        println!("No arguments provided. Running all solutions...\n");
        for &(year, ref days) in &years {
            for &day in days {
                run_solution(year, day);
            }
        }
    } else if args.len() == 3 {
        // Parse year and day from arguments
        let year: u32 = match args[1].parse() {
            Ok(y) => y,
            Err(_) => {
                eprintln!("Invalid year: {}", args[1]);
                return;
            }
        };

        let day: u8 = match args[2].parse() {
            Ok(d) if d > 0 && d <= 25 => d,
            _ => {
                eprintln!("Invalid day: {}. Day must be between 1 and 25.", args[2]);
                return;
            }
        };

        // Run the specified solution
        run_solution(year, day);
    } else {
        eprintln!("Usage: {} <year> <day>", args[0]);
    }
}

fn run_solution(year: u32, day: u8) {
    println!("Running Year {} Day {}", year, day);

    // Read the input file
    let input = match utils::read_input(year, day) {
        Ok(content) => content,
        Err(_) => {
            println!("No input found for Year {} Day {}", year, day);
            return;
        }
    };

    // Match the year and day to run the appropriate solution
    match (year, day) {
        (2023, 1) => {
            println!("Part 1: {}", year2023::day01::solve_part1(&input));
            println!("Part 2: {}", year2023::day01::solve_part2(&input));
        }
        (2023, 2) => {
            println!("Part 1: {}", year2023::day02::solve_part1(&input));
            println!("Part 2: {}", year2023::day02::solve_part2(&input));
        }
        (2023, 3) => {
            println!("Part 1: {}", year2023::day03::solve_part1(&input));
            println!("Part 2: {}", year2023::day03::solve_part2(&input));
        }
        (2024, 1) => {
            println!("Part 1: {}", year2024::day01::solve_part1(&input));
            println!("Part 2: {}", year2024::day01::solve_part2(&input));
        }
        (2024, 2) => {
            println!("Part 1: {}", year2024::day02::solve_part1(&input));
            println!("Part 2: {}", year2024::day02::solve_part2(&input));
        }
        (2024, 3) => {
            println!("Part 1: {}", year2024::day03::solve_part1(&input));
            println!("Part 2: {}", year2024::day03::solve_part2(&input));
        }
        (2024, 4) => {
            println!("Part 1: {}", year2024::day04::solve_part1(&input));
            println!("Part 2: {}", year2024::day04::solve_part2(&input));
        }
        _ => {
            println!("Solution for Year {} Day {} not implemented", year, day);
        }
    }
}
