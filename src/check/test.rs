#![cfg(test)]

use crate::*;

#[test]
fn main() {
    let handler = Handler::new(&map);

    handler.check_year(0);

    handler.check_most_recent(2015);
    handler.check_part(2015, 1, 0);
}

#[allow(clippy::match_same_arms)]
fn map(year: Year, day: Day) -> Option<Solver> {
    match day {
        1 => DayNothing::wrap(),
        2..=4 => None,
        5 => DayNothing::wrap(),
        6 => DayNocheck::wrap(),
        7 => DayIncorrectFormat::wrap(),
        8 => DayWrong::wrap(),
        9 => DayText::wrap(),

        _ => None,
    }
}

struct DayNothing;
impl DaySolver for DayNothing {
    fn part_1(input: &str) -> impl Into<SolutionPart> {}
    fn part_2(input: &str) -> impl Into<SolutionPart> {}
}

struct DayNocheck;
impl DaySolver for DayNocheck {
    fn part_1(input: &str) -> impl Into<SolutionPart> {
        1
    }
    fn part_2(input: &str) -> impl Into<SolutionPart> {}
    fn part_1_test_input() -> &'static str {
        "Text"
    }
}

struct DayIncorrectFormat;
impl DaySolver for DayIncorrectFormat {
    fn part_1(input: &str) -> impl Into<SolutionPart> {
        1
    }
    fn part_2(input: &str) -> impl Into<SolutionPart> {
        1.1
    }
    fn part_1_test_input() -> &'static str {
        "Text"
    }
    fn part_1_test_answer() -> impl Into<SolutionPart> {
        "String"
    }
    fn part_2_test_answer() -> impl Into<SolutionPart> {
        "String"
    }
}

struct DayWrong;
impl DaySolver for DayWrong {
    fn part_1(input: &str) -> impl Into<SolutionPart> {
        1
    }
    fn part_2(input: &str) -> impl Into<SolutionPart> {
        -1
    }
    fn part_1_test_input() -> &'static str {
        "Text"
    }
    fn part_1_test_answer() -> impl Into<SolutionPart> {
        0
    }
    fn part_2_test_answer() -> impl Into<SolutionPart> {
        0
    }
}

struct DayText;
impl DaySolver for DayText {
    fn part_1(input: &str) -> impl Into<SolutionPart> {
        "Wrong"
    }
    fn part_2(input: &str) -> impl Into<SolutionPart> {
        input
    }
    fn part_1_test_input() -> &'static str {
        "Correct"
    }
    fn part_1_test_answer() -> impl Into<SolutionPart> {
        "Correct"
    }
    fn part_2_test_answer() -> impl Into<SolutionPart> {
        "Correct"
    }
}
