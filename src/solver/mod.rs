mod solution_wrapper;
pub use solution_wrapper::SolutionPart;

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
    fn test(&self) -> solution_wrapper::CheckResult {
        let test_output = (self.solver)(self.test_input);
        test_output.check(&self.test_answer)
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
