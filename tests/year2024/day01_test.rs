use aoc::year2024::day01;

#[test]
fn test_part1() {
    let input = "line1\nline2\nline3";
    assert_eq!(day01::solve_part1(input), 42);
}

#[test]
fn test_part2() {
    let input = "line1\nline2\nline3";
    assert_eq!(day01::solve_part2(input), 84);
}
