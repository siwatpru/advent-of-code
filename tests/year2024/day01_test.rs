use aoc::year2024::day01;

#[test]
fn test_part1() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";
    assert_eq!(day01::solve_part1(input), 11);
}

#[test]
fn test_part2() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";
    assert_eq!(day01::solve_part2(input), 31);
}
