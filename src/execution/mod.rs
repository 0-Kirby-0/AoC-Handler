use crate::{
    execution::result::{CheckedRunReturn, DayReturn, PartReturn, Unchecked},
    input_handler::InputError,
    time_key::{
        Day, Part, PartInternal, TimeDetailDay, TimeDetailDayAndPart, TimeDetailNone, TimeKey,
    },
};
pub mod result;
pub mod solution_part;
use result::{AcquisitionError, CheckReturn, PartOutput, RunReturn};
pub use solution_part::SolutionPart;

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

        //? Special early return
        if matches!(request, Request::Check) // We only want to check, but
            && solver_part.test_input.is_empty() //We don't have a check input, but
            && let Ok(input) = run_input.get_or_init(|| self.input.get_day_input(time_key.deref()))
        //We do have a real input
        {
            let ret = (solver_part.solver)(input);
            return if matches!(ret, SolutionPart::Unimplemented) {
                //Which reveals the part is unimplemented
                Err(AcquisitionError::Unimplemented)
            } else {
                //Or there is real code, but we were only asked to check
                Ok(PartOutput::Checked(CheckReturn::Unchecked(
                    Unchecked::NoInput,
                )))
            };
        }

        //* Checking
        let check_return = if solver_part.test_input.is_empty() {
            CheckReturn::Unchecked(Unchecked::NoInput)
        } else {
            let ret = (solver_part.solver)(solver_part.test_input);
            match ret.check_against(&solver_part.test_answer) {
                None => return Err(AcquisitionError::Unimplemented),
                Some(check_return) => check_return,
            }
        };

        //? Early return, we're only checking
        if matches!(request, Request::Check) {
            return Ok(PartOutput::Checked(check_return));
        }

        let checked_run_return = if let CheckReturn::Failed(e) = check_return {
            CheckedRunReturn::CheckFailed(e)
        } else {
            match run_input.get_or_init(|| self.input.get_day_input(time_key.deref())) {
                Err(ie) => CheckedRunReturn::RunFailed(ie.clone()),
                Ok(input) => {
                    let time_start = std::time::Instant::now();
                    let solution_part = (solver_part.solver)(input);
                    let time_taken = time_start.elapsed();
                    CheckedRunReturn::Ok(RunReturn {
                        solution_part,
                        time_taken,
                    })
                }
            }
        };

        Ok(PartOutput::CheckedAndRan(checked_run_return))
    }
}
