#![allow(dead_code, unused_variables)]
#![warn(clippy::pedantic, clippy::nursery)]
#![warn(clippy::todo)]
#![allow(
    clippy::must_use_candidate,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc
)]
#![feature(never_type, bool_to_result)]

mod api;
pub use api::traits::{DaySolver, WrapSolver};

mod time_key;
pub use time_key::{Day, Part, Year};

mod execution;
pub use execution::SolutionPart;

mod input_handler;

pub struct Handler<'a> {
    input: std::cell::LazyCell<input_handler::Client>,
    mapper: &'a dyn Fn(Year, Day) -> Option<Solver>,
}

impl<'a> Handler<'a> {
    pub fn new(mapper: &'a dyn Fn(Year, Day) -> Option<Solver>) -> Self {
        Self {
            input: std::cell::LazyCell::new(input_handler::Client::new),
            mapper,
        }
    }
}

pub struct Solver {
    part_1: SolverPart,
    part_2: SolverPart,
}

struct SolverPart {
    solver: &'static dyn Fn(&str) -> execution::SolutionPart,
    test_input: &'static str,
    test_answer: execution::SolutionPart,
}

