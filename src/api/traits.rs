use crate::{SolutionPart, Solver, SolverPart, execution::test_input::TestInput};

/// A template for a zero-size type providing solution functions for a given day.
/// Test functions can be overridden.
///
/// ```
/// use aoc_handler::{DaySolver, SolutionPart, TestInput};
///
/// struct DayImpl;
///
/// impl DaySolver for DayImpl {
///     // Example: "Count the lines in the input."
///     fn part_1(input: &str) -> impl Into<SolutionPart> {
///         let parsed = parse(input);   // Invoking external helper functions.
///         parsed.iter().count()        // Return is inferred and accepted as the solution.
///     }
///
///     // Example: Not yet unlocked.
///     fn part_2(_input: &str) -> impl Into<SolutionPart> {
///         // Returning unit () is valid, and will be treated as "Unimplemented"
///     }
///
///     fn part_1_test_input() -> impl Into<TestInput> {
///         "Line 1
///         Line 2  
///         Line 3
///         Line 4"
///         // Leading whitespace on lines in the input is automatically removed
///     }
///
///     fn part_1_test_answer() -> impl Into<SolutionPart> {
///         4
///     }
///
///     // If there is no useable test (as in some older iterations of AoC),
///     // testing can be skipped by providing an empty string.
///     fn part_2_test_input() -> impl Into<TestInput> {
///         ""
///     }
/// }
/// ```
pub trait DaySolver {
    fn part_1(input: &str) -> impl Into<SolutionPart>;
    fn part_2(input: &str) -> impl Into<SolutionPart>;
    fn part_1_test_input() -> impl Into<TestInput> {}
    fn part_1_test_answer() -> impl Into<SolutionPart> {}
    fn part_2_test_input() -> impl Into<TestInput> {
        Self::part_1_test_input()
    }
    fn part_2_test_answer() -> impl Into<SolutionPart> {}
}

pub trait WrapSolver: Sized + DaySolver {
    fn wrap() -> Option<Solver> {
        Some(Solver {
            part_1: SolverPart {
                solver: &move |input| Self::part_1(input).into(),
                test_input: Self::part_1_test_input().into(),
                test_answer: Self::part_1_test_answer().into(),
            },
            part_2: SolverPart {
                solver: &move |input| Self::part_2(input).into(),
                test_input: Self::part_2_test_input().into(),
                test_answer: Self::part_2_test_answer().into(),
            },
        })
    }
}
impl<DS: DaySolver> WrapSolver for DS {}
