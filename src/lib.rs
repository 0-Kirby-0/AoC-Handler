#![allow(dead_code, unused_variables)]
mod api_traits;
mod input_handler;
mod solution_wrappers;
mod solver;

use input_handler::InputHandler;

pub use api_traits::{DayMapper, DaySolver};
pub use solution_wrappers::SolutionPart;
pub use solver::Solver;

pub type Day = u8;
pub type Year = u16;

pub struct Handler<T: DayMapper> {
    input_handler: InputHandler,
    day_mapper: T,
}

impl<T: DayMapper> Handler<T> {
    pub fn new(year: Year, day_mapper: T) -> Self {
        Self {
            input_handler: InputHandler::new(year),
            day_mapper,
        }
    }
    /// Verifies the solution with test input and answer, then runs and displays the result for the main input if correct.
    pub fn run(&self, day: Day) {
        let solver = self.day_mapper.map(day);
        let input = self.input_handler.get_day_input(day);

        println!("Day {}:", day);
        for part in 1..=2 {
            print!("Part {}: ", part);
            solver.run_part(part, &input);
        }
    }
}
