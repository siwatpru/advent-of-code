use aoc::year2023::day01;

#[test]
fn test_part1() {
    let input = "1abc2\n\
        pqr3stu8vwx\n\
        a1b2c3d4e5f\n\
        treb7uchet";
    assert_eq!(day01::solve_part1(input), 142);
}

#[test]
fn test_part2() {
    let input = "two1nine\n\
        eightwothree\n\
        abcone2threexyz\n\
        xtwone3four\n\
        4nineeightseven2\n\
        zoneight234\n\
        7pqrstsixteen";
    assert_eq!(day01::solve_part2(input), 281);
}
