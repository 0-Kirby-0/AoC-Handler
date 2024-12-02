use crate::SolutionPart;

pub struct Solver {
    part_1: SolverPart,
    part_2: SolverPart,
}

impl<DS> From<DS> for Solver
where
    DS: crate::DaySolver,
{
    fn from(ds: DS) -> Self {
        Solver {
            part_1: SolverPart {
                solver: Box::new(move |input| DS::part_1(input).into()),
                test_input: DS::part_1_test_input(),
                test_answer: DS::part_1_test_answer().into(),
            },
            part_2: SolverPart {
                solver: Box::new(move |input| DS::part_2(input).into()),
                test_input: DS::part_2_test_input(),
                test_answer: DS::part_2_test_answer().into(),
            },
        }
    }
}

struct SolverPart {
    solver: Box<dyn Fn(&str) -> SolutionPart>,
    test_input: &'static str,
    test_answer: SolutionPart,
}

impl Solver {
    fn get_part_solver(&self, index: u8) -> &SolverPart {
        match index {
            1 => &self.part_1,
            2 => &self.part_2,
            _ => panic!("Invalid part index."),
        }
    }

    pub fn run_part(&self, part: u8, input: &str) {
        self.get_part_solver(part).run(input);
    }
}

impl SolverPart {
    fn is_testable(&self) -> bool {
        !self.test_input.is_empty()
    }
    fn test(&self) -> TestResult {
        let test_output = (self.solver)(self.test_input);
        compare_part_solutions(&test_output, &self.test_answer)
    }
    fn time(&self, input: &str) -> (SolutionPart, std::time::Duration) {
        let start = std::time::Instant::now();
        let solution = (self.solver)(input);
        let time_taken = start.elapsed();
        (solution, time_taken)
    }

    fn print_time(&self, input: &str) {
        let (solution, time) = self.time(input);
        if solution.is_unfinished() {
            println!("Solution not yet implemented.");
            return;
        }
        println!("Solution: {:<15}\tTime taken: {:<8.2?}", solution, time);
    }

    fn run(&self, input: &str) {
        if self.is_testable() {
            let test_result = self.test();
            match (
                test_result.should_be_printed(),
                test_result.implies_real_results_are_valuable(),
            ) {
                (false, false) => (),
                (true, false) => println!("{test_result}"),
                (false, true) => self.print_time(input),
                (true, true) => {
                    println!("{test_result}");
                    print!("{:8}", " \"");
                    self.print_time(input);
                }
            }
        } else {
            self.print_time(input);
        }
    }
}

pub enum NamePending {}

fn compare_part_solutions(tested: &SolutionPart, correct: &SolutionPart) -> TestResult {
    if tested.is_unfinished() {
        TestResult::Unfinished
    } else if correct.is_unfinished() {
        TestResult::Unchecked(*tested)
    } else if std::mem::discriminant(tested) != std::mem::discriminant(correct) {
        TestResult::WrongFormat(*tested)
    } else if let (SolutionPart::Number(tested_num), SolutionPart::Number(correct_num)) =
        (tested, correct)
    {
        use std::cmp::Ordering::*;
        match tested_num.cmp(correct_num) {
            Equal => TestResult::Correct,
            Less => TestResult::TooLow(*tested, *correct),
            Greater => TestResult::TooHigh(*tested, *correct),
        }
    } else {
        unreachable!()
    }
}
pub enum TestResult {
    Correct,
    Unfinished,
    Unchecked(SolutionPart),
    WrongFormat(SolutionPart),
    TooLow(SolutionPart, SolutionPart),
    TooHigh(SolutionPart, SolutionPart),
}
impl TestResult {
    fn is_wrong(&self) -> bool {
        matches!(
            self,
            Self::TooHigh(_, _) | Self::TooLow(_, _) | Self::WrongFormat(_)
        )
    }
    fn is_unchecked(&self) -> bool {
        matches!(self, Self::Unchecked(_))
    }

    pub fn should_be_printed(&self) -> bool {
        self.is_wrong() || self.is_unchecked()
    }
    pub fn implies_real_results_are_valuable(&self) -> bool {
        !self.is_wrong() || self.is_unchecked()
    }
}

impl core::fmt::Display for TestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Correct => Ok(()),
            Self::Unfinished => Ok(()),
            Self::Unchecked(output) => {
                write!(f, "Test: {}, no answer to check against.", output)
            }
            Self::WrongFormat(ouput) => write!(
                f,
                "Test: {}. Test answer has wrong format to compare.",
                ouput
            ),
            Self::TooHigh(output, correct) => write!(
                f,
                "Test: {} which is too high. Should be {}.",
                output, correct
            ),
            Self::TooLow(output, correct) => write!(
                f,
                "Test: {} which is too low. Should be {}.",
                output, correct
            ),
        }
    }
}
