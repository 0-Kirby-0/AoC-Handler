use crate::{
    Day, Handler, Part, Year,
    time::{get_day_range, validate_day, validate_part, validate_year},
};
mod check_error;
pub use check_error::CheckError;

mod test;

impl Handler<'_> {
    pub fn check_year_range(&self, year_range: std::ops::RangeInclusive<Year>) {
        year_range.for_each(|year| self.check_year(year));
    }
    pub fn check_year(&self, year: Year) {
        if let Err(e) = validate_year(year) {
            println!("{e}");
            return;
        }
        self.check_day_range(year, get_day_range(year));
    }
    #[allow(clippy::missing_panics_doc)]
    pub fn check_day_range(&self, year: Year, day_range: std::ops::RangeInclusive<Day>) {
        if let Err(e) = validate_day(year, *day_range.start())
            .and_then(|()| validate_day(year, *day_range.end()))
        {
            println!("{e}");
            return;
        }

        let rest_list = day_range
            .map(|day| (day, self.check_day_raw(year, day)))
            .collect::<Vec<_>>();

        println!("Running {year}:");
        rest_list
            .chunk_by(|left, right| left.1.is_none() && right.1.is_none())
            .for_each(|slice| {
                if slice.len() == 1 {
                    let (day, rets) = slice[0].clone();
                    println!("Day {day}: {}", display_check_day_return(rets));
                } else {
                    let (first, _) = slice[0];
                    let (last, _) = slice.last().unwrap(); //Safety: Slice is guaranteed to contain at least two elements.
                    println!("Days {first}-{last}: Unimplemented.");
                }
            });
    }

    //Single Days
    pub fn check_most_recent(&self, year: Year) {
        if let Err(e) = validate_year(year) {
            println!("{e}");
            return;
        }
        let last_implemented = get_day_range(year)
            .filter_map(|day| (self.mapper)(year, day).map(|_| day))
            .last();
        match last_implemented {
            None => println!("Failed to check, {year} has no implemented days."),
            Some(day) => {
                self.check_day(year, day);
            }
        }
    }
    pub fn check_day(&self, year: Year, day: Day) {
        if let Err(e) = validate_day(year, day) {
            println!("{e}");
            return;
        }
        let rets = self.check_day_raw(year, day);
        println!("Checking {year}/{day}: {}", display_check_day_return(rets));
    }

    //Part
    pub fn check_part(&self, year: Year, day: Day, part: Part) {
        if let Err(e) = validate_day(year, day).and_then(|()| validate_part(part)) {
            println!("{e}");
            return;
        }
        let ret = self.check_part_raw(year, day, part);
        println!(
            "Checking {year}/{day}/{part}: {}",
            display_check_return(ret)
        );
    }

    //Internal
    pub(super) fn check_day_raw(
        &self,
        year: Year,
        day: Day,
    ) -> Option<(Result<(), CheckError>, Result<(), CheckError>)> {
        (self.mapper)(year, day).map(|solver| solver.check_both_parts())
    }
    pub(super) fn check_part_raw(
        &self,
        year: Year,
        day: Day,
        part: Part,
    ) -> Option<Result<(), CheckError>> {
        (self.mapper)(year, day).map(|solver| solver.check_part(part))
    }
}

impl crate::Solver {
    fn check_part(&self, part: Part) -> Result<(), CheckError> {
        match part {
            1 => self.part_1.check(),
            2 => self.part_2.check(),
            _ => unreachable!(),
        }
    }

    fn check_both_parts(&self) -> (Result<(), CheckError>, Result<(), CheckError>) {
        (self.part_1.check(), self.part_2.check())
    }
}

impl crate::SolverPart {
    fn check(&self) -> Result<(), CheckError> {
        if self.test_input.is_empty() {
            return Err(CheckError::NoTestInput);
        }
        let checked = (self.solver)(self.test_input);
        CheckError::compare(checked, &self.test_answer)
    }
}

fn display_check_day_return(
    rets: Option<(Result<(), CheckError>, Result<(), CheckError>)>,
) -> String {
    match rets {
        None => "Unimplemented.".to_string(),
        Some((Ok(()), Ok(()))) => "Succeeded.".to_string(),
        Some((part_1, part_2)) => format!(
            "\n    Part 1: {}\n    Part 2: {}",
            display_check_return(Some(part_1)),
            display_check_return(Some(part_2))
        ),
    }
}

fn display_check_return(ret: Option<Result<(), CheckError>>) -> String {
    match ret {
        None => "Unimplemented.".to_string(),
        Some(Ok(())) => "Succeeded.".to_string(),
        Some(Err(check_error)) => format!("{check_error}"),
    }
}
