use crate::{SolutionPart, Solver, SolverPart};

pub trait WrapSolver: Sized + crate::DaySolver {
    fn wrap() -> Option<Solver> {
        Some(Solver {
            part_1: SolverPart {
                solver: &move |input| Self::part_1(input).into(),
                test_input: Self::part_1_test_input(),
                test_answer: Self::part_1_test_answer().into(),
            },
            part_2: SolverPart {
                solver: &move |input| Self::part_2(input).into(),
                test_input: Self::part_2_test_input(),
                test_answer: Self::part_2_test_answer().into(),
            },
        })
    }
}
impl<DS: crate::DaySolver> WrapSolver for DS {}

/* Unfinished */
impl From<()> for SolutionPart {
    fn from(value: ()) -> Self {
        Self::Unfinished
    }
}
//? The only reason this needs nightly: Allowing the user to close their solution function with todo!() in peace
impl From<!> for SolutionPart {
    fn from(value: !) -> Self {
        Self::Unfinished
    }
}

/* Integer */
macro_rules! impl_from_int {
        ($($t:ty),* $(,)?) => {
            $(
            impl From<$t> for SolutionPart {
                fn from(value: $t) -> Self {
                    Self::Integer(value.to_string())
                }
            }
            )*
        };
    }

impl_from_int!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

/* Real */
impl From<f32> for SolutionPart {
    fn from(value: f32) -> Self {
        Self::Real(value.into())
    }
}
impl From<f64> for SolutionPart {
    fn from(value: f64) -> Self {
        Self::Real(value)
    }
}

/* String */
impl From<String> for SolutionPart {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
impl From<&str> for SolutionPart {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}
impl From<char> for SolutionPart {
    fn from(value: char) -> Self {
        Self::String(value.to_string())
    }
}
impl From<bool> for SolutionPart {
    fn from(value: bool) -> Self {
        Self::String(value.to_string())
    }
}
