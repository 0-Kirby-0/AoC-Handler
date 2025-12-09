use crate::SolutionPart;

pub struct Solver {
    part_1: SolverPart,
    part_2: SolverPart,
}

impl Solver {
    pub fn run_part(&self, part: u8, input: &str) {
        self.get_part_solver(part).run(input);
    }

    fn get_part_solver(&self, index: u8) -> &SolverPart {
        match index {
            1 => &self.part_1,
            2 => &self.part_2,
            _ => panic!("Invalid part index."),
        }
    }
}

impl<DS> From<DS> for Solver
where
    DS: crate::DaySolver,
{
    fn from(ds: DS) -> Self {
        Self {
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

impl SolverPart {
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
        println!("Solution: {solution:<15}\tTime taken: {time:<8.2?}");
    }
}

fn compare_part_solutions(tested: &SolutionPart, correct: &SolutionPart) -> TestResult {
    use std::cmp::Ordering;
    match (tested, correct) {
        (SolutionPart::Unfinished, _) => TestResult::Unfinished,
        (_, SolutionPart::Unfinished) => TestResult::Unchecked(tested.clone()),
        (a, b) if std::mem::discriminant(a) != std::mem::discriminant(b) => {
            TestResult::WrongFormat(tested.clone())
        }
        (SolutionPart::Integer(a), SolutionPart::Integer(b)) => {
            fn compare_strings_as_ints(a: &str, b: &str) -> std::cmp::Ordering {
                fn split_sign(s: &str) -> (bool, &str) {
                    s.strip_prefix('-')
                        .map_or((false, s), |digits| (true, digits))
                }

                let (neg_a, a) = split_sign(a);
                let (neg_b, b) = split_sign(b);

                // Different signs: negative < positive
                if neg_a != neg_b {
                    return if neg_a {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    };
                }

                // Same sign: compare absolute values
                let abs_cmp = match a.len().cmp(&b.len()) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Equal => a.cmp(b),
                };

                // For negatives, larger absolute value means smaller number
                if neg_a { abs_cmp.reverse() } else { abs_cmp }
            }

            match compare_strings_as_ints(a, b) {
                Ordering::Equal => TestResult::Correct,
                Ordering::Less => TestResult::TooLow(tested.clone(), correct.clone()),
                Ordering::Greater => TestResult::TooHigh(tested.clone(), correct.clone()),
            }
        }
        (SolutionPart::Real(a), SolutionPart::Real(b)) => a.partial_cmp(b).map_or_else(
            || {
                if a.is_normal() | a.is_subnormal() {
                    TestResult::Unchecked(tested.clone())
                } else {
                    TestResult::WrongFormat(tested.clone())
                }
            },
            |cmp| match cmp {
                Ordering::Equal => TestResult::Correct,
                Ordering::Less => TestResult::TooLow(tested.clone(), correct.clone()),
                Ordering::Greater => TestResult::TooHigh(tested.clone(), correct.clone()),
            },
        ),
        (SolutionPart::String(a), SolutionPart::String(b)) => {
            if a == b {
                TestResult::Correct
            } else {
                TestResult::Incorrect(tested.clone())
            }
        }
        (_, _) => unreachable!(),
    }
}
pub enum TestResult {
    Correct,
    Unfinished,
    Unchecked(SolutionPart),
    WrongFormat(SolutionPart),
    Incorrect(SolutionPart),
    TooLow(SolutionPart, SolutionPart),
    TooHigh(SolutionPart, SolutionPart),
}
impl TestResult {
    pub fn should_be_printed(&self) -> bool {
        !matches!(self, Self::Correct | Self::Unfinished)
    }
    pub fn implies_real_results_are_valuable(&self) -> bool {
        matches!(self, Self::Correct | Self::Unfinished | Self::Unchecked(_))
    }
}

impl core::fmt::Display for TestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Correct | Self::Unfinished => Ok(()),
            Self::Unchecked(output) => {
                write!(f, "Test: {output}, no answer to check against.")
            }
            Self::WrongFormat(ouput) => {
                write!(f, "Test: {ouput}. Test answer has wrong format to compare.",)
            }
            Self::Incorrect(output) => {
                write!(f, "Test: {output} is incorrect.",)
            }
            Self::TooHigh(output, correct) => {
                write!(f, "Test: {output} is too high. Should be {correct}.",)
            }
            Self::TooLow(output, correct) => {
                write!(f, "Test: {output} is too low. Should be {correct}.",)
            }
        }
    }
}
