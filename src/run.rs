use crate::{
    Handler, SolutionPart,
    check::CheckError,
    time::{Day, Part, Year, get_day_range, validate_day, validate_part, validate_year},
};

impl Handler<'_> {
    pub fn run_year_range(&self, year_range: std::ops::RangeInclusive<Year>) {
        year_range.for_each(|year| self.run_year(year));
    }
    pub fn run_year(&self, year: Year) {
        if let Err(e) = validate_year(year) {
            println!("{e}");
            return;
        }
        self.run_day_range(year, get_day_range(year));
    }
    #[allow(clippy::missing_panics_doc)]
    pub fn run_day_range(&self, year: Year, day_range: std::ops::RangeInclusive<Day>) {
        if let Err(e) = validate_day(year, *day_range.start())
            .and_then(|()| validate_day(year, *day_range.end()))
        {
            println!("{e}");
            return;
        }

        let rest_list = day_range
            .map(|day| (day, self.run_day_raw(year, day)))
            .collect::<Vec<_>>();

        println!("{year} runs:");
        rest_list
            .chunk_by(|left, right| left.1.is_none() && right.1.is_none())
            .for_each(|slice| {
                if slice.len() == 1 {
                    let (day, rets) = slice[0].clone();
                    println!("Day {day}: {}", display_run_day_return(rets));
                } else {
                    let (first, _) = slice[0];
                    let (last, _) = slice.last().unwrap(); //Safety: Slice is guaranteed to contain at least two elements.
                    println!("Days {first}-{last}: Unimplemented.");
                }
            });
    }

    //Single Days
    pub fn run_most_recent(&self, year: Year) {
        if let Err(e) = validate_year(year) {
            println!("{e}");
            return;
        }
        let last_implemented = get_day_range(year)
            .filter_map(|day| (self.mapper)(year, day).map(|_| day))
            .last();
        match last_implemented {
            None => println!("Failed to run, {year} has no implemented days."),
            Some(day) => {
                self.run_day(year, day);
            }
        }
    }
    pub fn run_day(&self, year: Year, day: Day) {
        if let Err(e) = validate_day(year, day) {
            println!("{e}");
            return;
        }
        let rets = self.run_day_raw(year, day);
        println!("Running {year}/{day}: {}", display_run_day_return(rets));
    }

    //Part
    pub fn run_part(&self, year: Year, day: Day, part: Part) {
        if let Err(e) = validate_day(year, day).and_then(|()| validate_part(part)) {
            println!("{e}");
            return;
        }
        let ret = self.run_part_raw(year, day, part);
        println!("Running {year}/{day}/{part}: {}", display_run_return(ret));
    }
    //Internal
    fn run_day_raw(&self, year: Year, day: Day) -> Option<(RunReturn, RunReturn)> {
        self.run_part_raw(year, day, 1).and_then(|part_1| {
            self.run_part_raw(year, day, 1)
                .map(|part_2| (part_1, part_2))
        })
    }
    fn run_part_raw(&self, year: Year, day: Day, part: Part) -> Option<RunReturn> {
        let check_result = self.check_part_raw(year, day, part)?.err();

        let run_return = if check_result
            .as_ref()
            .is_none_or(CheckError::implies_real_results_are_valuable)
        {
            let (solution, time_taken) = (self.mapper)(year, day)
                .expect("Mapper provided implementation when testing, but not running.")
                .run_part(part, &self.input.get_day_input(year, day));
            RunReturn {
                solution: Some(solution),
                time_taken: Some(time_taken),
                check_result,
            }
        } else {
            RunReturn {
                solution: None,
                time_taken: None,
                check_result,
            }
        };

        Some(run_return)
    }
}

impl crate::Solver {
    fn run_part(&self, part: Part, input: &str) -> (SolutionPart, std::time::Duration) {
        match part {
            1 => self.part_1.run(input),
            2 => self.part_2.run(input),
            _ => unreachable!(),
        }
    }
}

impl crate::SolverPart {
    fn run(&self, input: &str) -> (SolutionPart, std::time::Duration) {
        let start = std::time::Instant::now();
        let solution = (self.solver)(input);
        let time_taken = start.elapsed();
        (solution, time_taken)
    }
}

#[derive(Clone)]
struct RunReturn {
    solution: Option<SolutionPart>,
    time_taken: Option<std::time::Duration>,
    check_result: Option<CheckError>,
}

fn display_run_day_return(rets: Option<(RunReturn, RunReturn)>) -> String {
    match rets {
        None => "Unimplemented.".to_string(),
        Some((
            RunReturn {
                solution: _,
                time_taken: _,
                check_result: Some(CheckError::Unfinished),
            },
            RunReturn {
                solution: _,
                time_taken: _,
                check_result: Some(CheckError::Unfinished),
            },
        )) => "Unfinished".to_string(),
        Some((part_1, part_2)) => format!(
            "\n    Part 1: {}\n    Part 2: {}",
            display_run_return(Some(part_1)),
            display_run_return(Some(part_2))
        ),
    }
}

fn display_run_return(ret: Option<RunReturn>) -> String {
    match ret {
        None => "Unimplemented.".to_string(),
        Some(RunReturn {
            solution: _,
            time_taken: _,
            check_result: Some(CheckError::Unfinished),
        }) => "Unfinished".to_string(),
        Some(RunReturn {
            solution,
            time_taken,
            check_result,
        }) => {
            let solution_print = solution
                .and_then(|solution| {
                    time_taken
                        .map(|time_takem| format!("{solution:<20}Time taken: {time_taken:<8.2?}"))
                })
                .unwrap_or_default();
            let error_print = check_result
                .map(|check_error| format!("Test: {check_error}"))
                .unwrap_or_default();

            solution_print + &error_print
        }
    }
}
