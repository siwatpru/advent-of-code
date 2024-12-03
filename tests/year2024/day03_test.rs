use aoc::year2024::day03;

#[test]
fn test_part1() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(day03::solve_part1(input), 161);
}

#[test]
fn test_part2() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(day03::solve_part2(input), 161);
}