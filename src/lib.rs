#![allow(dead_code, unused_variables)]
#![warn(clippy::pedantic, clippy::nursery)]
//#![warn(clippy::todo)]
#![allow(clippy::must_use_candidate, clippy::missing_const_for_fn)]
#![feature(never_type)]

mod input_handler;

mod api_traits;
pub use api_traits::DaySolver;

mod check;
mod run;

mod time;
pub use time::{Day, Part, Year};

mod type_conversions;
pub use type_conversions::WrapSolver;

pub struct Handler<'a> {
    input: input_handler::Client,
    mapper: &'a dyn Fn(Year, Day) -> Option<Solver>,
}

impl<'a> Handler<'a> {
    pub fn new(mapper: &'a dyn Fn(Year, Day) -> Option<Solver>) -> Self {
        Self {
            input: input_handler::Client::new(),
            mapper,
        }
    }
}

pub struct Solver {
    part_1: SolverPart,
    part_2: SolverPart,
}

struct SolverPart {
    solver: &'static dyn Fn(&str) -> SolutionPart,
    test_input: &'static str,
    test_answer: SolutionPart,
}

#[derive(Debug, Clone)]
pub enum SolutionPart {
    Unfinished,
    Integer(String),
    Real(f64),
    String(String),
}

impl SolutionPart {
    pub fn variant_name(&self) -> String {
        match self {
            Self::Unfinished => "Unfinished".to_string(),
            Self::Integer(_) => "Integer".to_string(),
            Self::Real(_) => "Real".to_string(),
            Self::String(_) => "String".to_string(),
        }
    }
}

impl core::fmt::Display for SolutionPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unfinished => write!(f, "Unfinished"),
            Self::Integer(n) => write!(f, "'{n}'"),
            Self::Real(r) => write!(f, "'{r}'"),
            Self::String(s) => write!(f, "'{s}'"),
        }
    }
}
