mod utils;
mod year2023;
mod year2024;

fn main() {
    let years = vec![(2023, vec![1]), (2024, vec![1])];

    for &(year, ref days) in &years {
        for &day in days {
            println!("Running Year {} Day {}", year, day);

            let input = match utils::read_input(year, day) {
                Ok(content) => content,
                Err(_) => {
                    println!("No input found for Year {} Day {}", year, day);
                    continue;
                }
            };

            match (year, day) {
                (2023, 1) => {
                    println!("Part 1: {}", year2023::day01::solve_part1(&input));
                    println!("Part 2: {}", year2023::day01::solve_part2(&input));
                }
                (2024, 1) => {
                    println!("Part 1: {}", year2024::day01::solve_part1(&input));
                    println!("Part 2: {}", year2024::day01::solve_part2(&input));
                }
                _ => {
                    println!("Solution for Year {} Day {} not implemented", year, day);
                }
            }
        }
    }
}
