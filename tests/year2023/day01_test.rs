use aoc::year2023::day01;

#[test]
fn test_part1_example() {
    let example_input = "line1\nline2\nline3";
    assert_eq!(day01::solve_part1(example_input), 42);
}

#[test]
fn test_part2_example() {
    let example_input = "line1\nline2\nline3";
    assert_eq!(day01::solve_part2(example_input), 84);
}
