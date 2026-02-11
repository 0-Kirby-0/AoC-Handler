use crate::{SolutionPart, Solver, SolverPart, execution::test_input::TestInput};

/// A template for a zero-size type providing solution functions for a given day.
/// Test functions can be overridden.
///
/// ```
/// impl DaySolver for Day {
///     // Example: "Count the lines in the input."
///     fn part_1(input: &str) -> impl Into<SolutionPart> {
///         let parsed= parse(input);   // Invoking external helper functions.
///         parsed.iter().count()       // Return is automatically inferred and accepted as the solution.
///     }
///     // Example: Not yet unlocked.
///     fn part_2(input: &str) -> impl Into<SolutionPart> {
///         // No return is valid, and will be treated as "Not finished"
///     }
///     fn part_1_test_input() -> &'static str {
///         "Line 1
///         Line 2
///         Line 3
///         Line 4"
///     }
///     fn part_1_test_answer() -> impl Into<SolutionPart> {
///         4
///     }
///     // Test input for part 2 is the same as part 1, so left blank.
///     //Correct solution for part 2 is not available yet, so left blank too.
///
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
