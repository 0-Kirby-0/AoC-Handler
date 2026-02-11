use crate::{
    execution::result::{CheckedRunReturn, DayReturn, PartReturn, Unchecked},
    input_handler::InputError,
    time_key::{
        Day, Part, PartInternal, TimeDetailDay, TimeDetailDayAndPart, TimeDetailNone, TimeKey,
    },
};

pub mod solution_part;
pub use solution_part::SolutionPart;
mod test_input;
pub use test_input::TestInput;
pub mod result;
use result::{AcquisitionError, CheckReturn, PartOutput, RunReturn};

#[derive(Debug, Clone, Copy)]
pub enum Request {
    Check,
    CheckAndRun,
}

impl super::Handler<'_> {
    pub(super) fn execute_year_range(
        &self,
        request: Request,
        years: impl Iterator<Item = TimeKey<TimeDetailNone>>,
    ) -> impl Iterator<Item = (TimeKey<TimeDetailNone>, impl Iterator<Item = DayReturn>)> {
        years.map(move |key| (key, self.execute_year(request, key)))
    }

    pub(super) fn execute_year(
        &self,
        request: Request,
        time_key: TimeKey<TimeDetailNone>,
    ) -> impl Iterator<Item = DayReturn> {
        time_key
            .iterate()
            .map(move |key| self.execute_day(request, key))
    }

    pub(super) fn execute_most_recent_day(
        &self,
        request: Request,
        time_key: TimeKey<TimeDetailNone>,
    ) -> (Day, DayReturn) {
        let mut days_rev = time_key.iterate().rev().map(move |key| {
            let (_, day) = key.to_primitive();
            (day, self.execute_day(request, key))
        });
        days_rev
            .find(|(_, dr)| !matches!(dr, [Err(_), Err(_)]))
            .unwrap_or_else(|| days_rev.last().unwrap()) //Safety: Even if the user doesn't provide any mapping, the elements will just be AcquisitionError::NotMapped
    }

    pub(super) fn execute_day(
        &self,
        request: Request,
        time_key: TimeKey<TimeDetailDay>,
    ) -> DayReturn {
        //? Persistent access to the run input, since it's shared between parts, and getting it isn't free
        let run_input = std::cell::OnceCell::new();

        time_key
            .both()
            .map(|key| self.execute_part(request, key, &run_input))
    }

    pub(super) fn execute_most_recent_part(
        &self,
        request: Request,
        time_key: TimeKey<TimeDetailNone>,
    ) -> (Day, Part, PartReturn) {
        let mut parts_rev = time_key.iterate().rev().flat_map(move |key| {
            let run_input = std::cell::OnceCell::new();
            key.both().into_iter().rev().map(move |key| {
                let (_, day, part) = key.to_primitive();
                (day, part, self.execute_part(request, key, &run_input))
            })
        });

        parts_rev
            .find(|(_, _, pr)| pr.is_ok())
            .unwrap_or_else(|| parts_rev.last().unwrap())
    }

    pub(super) fn execute_part(
        &self,
        request: Request,
        time_key: TimeKey<TimeDetailDayAndPart>,
        run_input: &std::cell::OnceCell<Result<String, InputError>>,
    ) -> PartReturn {
        /*
        Flow:
        Top Level:
            Check time key, short circuit //? There was thought about letting the user define custom days, as long as they map them correctly, and don't try to run them. Unnecessary overhead and silly
            Fetch Solver, short circuit
        Check:
            See if we have input. If not, flag, but trial run real input. If it returns unit, we can drop back out and give a "Not implemented". //? This is only really needed when we *just* want a check
            See if we have output. If not, run the test anyway, but it does mean that if a real run was requested, we still have to do one, we can't prove the solution is wrong.
            Run the test, compare the output, and return it
        Run:
            Only run if the test either passed or there wasn't test input or output.
            Run and time, return the result. Easy. */
        let solver = {
            let (year, day, _) = time_key.to_primitive();
            (self.mapper)(year, day)
        }
        .ok_or(AcquisitionError::NotMapped)?;

        let solver_part = match time_key.detail.part {
            PartInternal::One => solver.part_1,
            PartInternal::Two => solver.part_2,
        };

        //* Checking

        let check_return = match solver_part.test_input {
            TestInput::None => CheckReturn::Unchecked(Unchecked::MissingInput),
            TestInput::Empty => match solver_part.test_answer {
                SolutionPart::Unimplemented => CheckReturn::Unchecked(Unchecked::Elided),
                sp => CheckReturn::Unchecked(Unchecked::ElideMismatch(sp)),
            },
            TestInput::Input(i) => {
                let ret = (solver_part.solver)(&i);
                match ret.check_against(&solver_part.test_answer) {
                    None => return Err(AcquisitionError::Unimplemented),
                    Some(check_return) => check_return,
                }
            }
        };

        //? Return: Only checking
        if matches!(request, Request::Check) {
            //? Special case: No test input, but real input, and real input returned Unimplemented
            if matches!(
                check_return,
                CheckReturn::Unchecked(Unchecked::MissingInput)
            ) && let Ok(input) =
                run_input.get_or_init(|| self.input.get_day_input(time_key.deref()))
                && matches!((solver_part.solver)(input), SolutionPart::Unimplemented)
            {
                return Err(AcquisitionError::Unimplemented);
            }
            return Ok(PartOutput::Checked(check_return));
        }

        //* running
        //? Return: Check failed
        if let CheckReturn::Failed(e) = check_return {
            return Ok(PartOutput::CheckedAndRan(CheckedRunReturn::CheckFailed(e)));
        }

        let checked_run_return = run_input
            .get_or_init(|| self.input.get_day_input(time_key.deref()))
            .as_ref()
            .map(|input| {
                let time_start = std::time::Instant::now();
                let solution_part = (solver_part.solver)(input);
                let time_taken = time_start.elapsed();

                RunReturn {
                    solution_part,
                    time_taken,
                }
            })
            .map_or_else(
                |input_error| CheckedRunReturn::RunFailed(input_error.clone()),
                |run_return| match check_return {
                    CheckReturn::Passed => CheckedRunReturn::Ok(run_return),
                    CheckReturn::Unchecked(uc) => CheckedRunReturn::Unchecked {
                        reason: uc,
                        ret: run_return,
                    },
                    CheckReturn::Failed(_) => unreachable!(),
                },
            );

        match checked_run_return {
            CheckedRunReturn::Ok(RunReturn {
                solution_part: SolutionPart::Unimplemented,
                time_taken: _,
            }) //Test passed, but rr is unimplemented? Weird
            | CheckedRunReturn::Unchecked {
                reason: _,
                ret:
                    RunReturn {
                        solution_part: SolutionPart::Unimplemented,
                        time_taken: _,
                    },
            } => return Err(AcquisitionError::Unimplemented),
            _ => (),
        }

        Ok(PartOutput::CheckedAndRan(checked_run_return))
    }
}
