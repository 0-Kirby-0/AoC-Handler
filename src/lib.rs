#![allow(dead_code, unused_variables)]
#![warn(clippy::pedantic, clippy::nursery, clippy::todo)]
#![allow(clippy::must_use_candidate, clippy::missing_const_for_fn)]
#![feature(never_type)]

mod api_traits;
mod input_handler;

mod mapper;
mod solver;

use input_handler::Client;

pub use api_traits::{DayMapper, DaySolver};
pub use solver::{SolutionPart, Solver};
pub type Day = u8;
pub type Year = u16;

pub struct Handler {
    year: Year,
    input: Client,
    day_mapper: mapper::Mapper,
}

impl Handler {
    pub fn new(year: Year, day_mapper: mapper::Mapper) -> Self {
        Self {
            year,
            input: Client::new(),
            day_mapper,
        }
    }
    /// Verifies the solution with test input and answer, then runs and displays the result for the main input if correct.
    pub fn run(&self, day: Day) {
        let solver = (self.day_mapper.map)(day);
        let input = self.input.get_day_input(self.year, day);

        println!("Day {day}:");
        for part in 1..=2 {
            print!("Part {part}: ");
            solver.run_part(part, &input);
        }
    }
}
