use aoc24::days::day02::{puzzle_1, puzzle_2};

const EXAMPLE_INPUT: &str = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

const EXAMPLE_INPUT_2: &str = "
15 16 17 20 22 24 26 26
5 4 3 2 1 1
";

#[test]
fn d02_p1_t1() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_1(self::EXAMPLE_INPUT), Ok(2));
}

#[test]
fn d02_p1_t2() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_1(self::EXAMPLE_INPUT_2), Ok(0));
}

#[test]
fn d02_p2_t1() {
    assert_eq!(puzzle_2(self::EXAMPLE_INPUT), Ok(4));
}
