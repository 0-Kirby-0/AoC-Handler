use super::{
    Day,
    solver::{SolutionPart, Solver},
};

pub trait DayMapper {
    /// Provides access to the various day solutions.
    ///
    /// ```
    /// match day {
    ///     1 => day_1::Day.into(),
    ///     2 => day_2::Day.into(),
    ///     0 | 25.. => panic!("Invalid day, AoC only lasts from 1st to 25th of December."),
    ///     _ => unimplemented!("Not yet solved."),
    /// }
    /// ```
    fn map(&self, day: Day) -> Solver;
}

/// A template for a zero-size type providing solution functions for a given day.
/// Test input functions can be overridden.
///
/// ```
/// impl DaySolver for Day {
///     // Example: "Count the lines in the input."
///     fn part_1(input: &str) -> impl Into<PartSolution> {
///         let parsed= parse(input);   // Invoking external helper functions.
///         parsed.iter().count()       // Return is automatically inferred and accepted as the solution.
///     }
///     // Example: Not yet unlocked.
///     fn part_2(input: &str) -> impl Into<PartSolution> {
///         // No return is valid, and will be treated as "Not finished"
///     }
///     fn part_1_test_input() -> &'static str {
///         r"Line 1
///         Line 2
///         Line 3
///         Line 4"
///     }
///     fn part_1_test_answer() -> impl Into<PartSolution> {
///         4
///     }
///     // Test input for part 2 is the same as part 1, so left blank.
///     //Correct solution for part 2 is not available yet, so left blank too.
///
/// }
pub trait DaySolver {
    fn part_1(input: &str) -> impl Into<SolutionPart>;
    fn part_2(input: &str) -> impl Into<SolutionPart>;
    fn part_1_test_input() -> &'static str {
        r""
    }
    fn part_1_test_answer() -> impl Into<SolutionPart> {}
    fn part_2_test_input() -> &'static str {
        Self::part_1_test_input()
    }
    fn part_2_test_answer() -> impl Into<SolutionPart> {}
}
