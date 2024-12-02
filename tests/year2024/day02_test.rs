use aoc::year2024::day02;

#[test]
fn test_part1() {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    assert_eq!(day02::solve_part1(input), 2);
}

#[test]
fn test_part2() {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    assert_eq!(day02::solve_part2(input), 2);
}
