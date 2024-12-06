use aoc24::days::day01::{puzzle_1, puzzle_2};

const EXAMPLE_INPUT: &str = "
3   4
4   3
2   5
1   3
3   9
3   3
";

#[test]
fn d01_p1_t1() {
    env_logger::init();
    assert_eq!(puzzle_1(self::EXAMPLE_INPUT), Ok(11));
}

#[test]
fn d01_p2_t1() {
    assert_eq!(puzzle_2(self::EXAMPLE_INPUT), Ok(31));
}
