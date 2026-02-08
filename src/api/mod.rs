use crate::{
    Day, Handler, Part, Year,
    execution::Request,
    time_key::{TimeDetailDay, TimeDetailDayAndPart, TimeDetailNone, TimeKey},
};

pub mod display;
pub mod traits;

impl Handler<'_> {
    /// Checks and runs all provided years, after deduplicating and ordering them for bulk validation/benchmarking.
    /// Checking attempts to run the provided `test_input()` and compare it to `test_answer()`; running repeats that and, if it passes, fetches real Advent of Code input and executes it.
    pub fn run_year_range(&self, years: impl Iterator<Item = Year>) {
        self.process_year_range(Request::CheckAndRun, years);
    }

    /// Checks and runs a single year.
    /// Checking attempts to run the provided `test_input()` and compare it to `test_answer()`; running repeats that and, if it passes, fetches real Advent of Code input and executes it.
    pub fn run_year(&self, year: Year) {
        self.process_year(Request::CheckAndRun, year);
    }

    /// Checks and runs the most recent implemented day for a year, choosing the highest numbered day with an implementation.
    /// Checking attempts to run the provided `test_input()` and compare it to `test_answer()`; running repeats that and, if it passes, fetches real Advent of Code input and executes it.
    pub fn run_most_recent_day(&self, year: Year) {
        self.process_most_recent_day(Request::CheckAndRun, year);
    }

    /// Checks and runs a specific day for a year.
    /// Checking attempts to run the provided `test_input()` and compare it to `test_answer()`; running repeats that and, if it passes, fetches real Advent of Code input and executes it.
    pub fn run_day(&self, year: Year, day: Day) {
        self.process_day(Request::CheckAndRun, year, day);
    }

    /// Checks and runs the most recent implemented part for a year, selecting the highest implemented part on the most recent implemented day.
    /// Checking attempts to run the provided `test_input()` and compare it to `test_answer()`; running repeats that and, if it passes, fetches real Advent of Code input and executes it.
    pub fn run_most_recent_part(&self, year: Year) {
        self.process_most_recent_part(Request::CheckAndRun, year);
    }

    /// Checks and runs a specific part of a specific day.
    /// Checking attempts to run the provided `test_input()` and compare it to `test_answer()`; running repeats that and, if it passes, fetches real Advent of Code input and executes it.
    pub fn run_part(&self, year: Year, day: Day, part: Part) {
        self.process_part(Request::CheckAndRun, year, day, part);
    }

    /// Checks all provided years, after deduplicating and ordering them for bulk validation/benchmarking.
    /// Checking attempts to run the provided `test_input()` and compare it to `test_answer()`.
    pub fn check_year_range(&self, years: impl Iterator<Item = Year>) {
        self.process_year_range(Request::Check, years);
    }

    /// Checks a single year.
    /// Checking attempts to run the provided `test_input()` and compare it to `test_answer()`.
    pub fn check_year(&self, year: Year) {
        self.process_year(Request::Check, year);
    }

    /// Checks the most recent implemented day for a year, choosing the highest numbered day with an implementation.
    /// Checking attempts to run the provided `test_input()` and compare it to `test_answer()`.
    pub fn check_most_recent_day(&self, year: Year) {
        self.process_most_recent_day(Request::Check, year);
    }

    /// Checks a specific day for a year.
    /// Checking attempts to run the provided `test_input()` and compare it to `test_answer()`.
    pub fn check_day(&self, year: Year, day: Day) {
        self.process_day(Request::Check, year, day);
    }

    /// Checks the most recent implemented part for a year, selecting the higher implemented part on the most recent implemented day.
    /// Checking attempts to run the provided `test_input()` and compare it to `test_answer()`.
    pub fn check_most_recent_part(&self, year: Year) {
        self.process_most_recent_part(Request::Check, year);
    }

    /// Checks a specific part of a specific day.
    /// Checking attempts to run the provided `test_input()` and compare it to `test_answer()`.
    pub fn check_part(&self, year: Year, day: Day, part: Part) {
        self.process_part(Request::Check, year, day, part);
    }

    fn process_year_range(&self, request: Request, years: impl Iterator<Item = Year>) {
        let (years, errors): (std::collections::BTreeSet<_>, std::collections::HashSet<_>) =
            itertools::Itertools::partition_result(years.map(TimeKey::<TimeDetailNone>::new));

        if !errors.is_empty() {
            eprint!("Provided invalid years:");
            for tke in errors {
                eprintln!("{tke}");
            }
        }
        if years.is_empty() {
            eprintln!("All provided years were invalid.");
            std::process::exit(0);
        }

        let year_returns = self.execute_year_range(request, years.into_iter());

        println!(
            "{} {}",
            request,
            display::render_multiple_year_results(year_returns)
        );
    }

    fn process_year(&self, request: Request, year: Year) {
        let time_key = TimeKey::<TimeDetailNone>::new(year).unwrap_or_else(|tke| {
            eprintln!("{tke}");
            std::process::exit(0)
        });

        let day_returns = self.execute_year(request, time_key);

        println!(
            "{} {}",
            request,
            display::render_year_returns(time_key, day_returns)
        );
    }

    fn process_most_recent_day(&self, request: Request, year: Year) {
        let time_key = TimeKey::<TimeDetailNone>::new(year).unwrap_or_else(|tke| {
            eprintln!("{tke}");
            std::process::exit(0)
        });

        let (day, day_return) = self.execute_most_recent_day(request, time_key);

        let time_key = TimeKey::<TimeDetailDay>::new(year, day)
            .expect("Internal error: Most recent day was invalid.");

        println!(
            "{} {}",
            request,
            display::render_day_return(time_key, &day_return)
        );
    }

    fn process_day(&self, request: Request, year: Year, day: Day) {
        let time_key = TimeKey::<TimeDetailDay>::new(year, day).unwrap_or_else(|tke| {
            eprintln!("{tke}");
            std::process::exit(0)
        });

        let day_return = self.execute_day(request, time_key);

        println!(
            "{} {}",
            request,
            display::render_day_return(time_key, &day_return)
        );
    }

    fn process_most_recent_part(&self, request: Request, year: Year) {
        let time_key = TimeKey::<TimeDetailNone>::new(year).unwrap_or_else(|tke| {
            eprintln!("{tke}");
            std::process::exit(0)
        });

        let (day, part, part_return) = self.execute_most_recent_part(request, time_key);

        let time_key = TimeKey::<TimeDetailDayAndPart>::new(year, day, part)
            .expect("Internal error: Most recent part was invalid.");

        println!(
            "{} {}",
            request,
            display::render_part_return(time_key, &part_return)
        );
    }

    fn process_part(&self, request: Request, year: Year, day: Day, part: Part) {
        let time_key =
            TimeKey::<TimeDetailDayAndPart>::new(year, day, part).unwrap_or_else(|tke| {
                eprintln!("Unable to check part: {tke}");
                std::process::exit(0)
            });

        let request = Request::Check;

        let part_return = self.execute_part(request, time_key, &std::cell::OnceCell::new());

        println!(
            "{} {}",
            request,
            display::render_part_return(time_key, &part_return)
        );
    }
}
