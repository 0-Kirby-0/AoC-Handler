use crate::execution::result::CheckReturn;

#[derive(Debug, Clone)]
pub enum SolutionPart {
    Unimplemented,
    Number(rust_decimal::Decimal),
    String(String),
}

#[derive(Debug, Clone)]
pub enum CheckError {
    WrongFormat(SolutionPart, SolutionPart),
    Incorrect(SolutionPart, SolutionPart, Option<std::cmp::Ordering>),
}

impl SolutionPart {
    pub fn check_against(self, correct: &Self) -> Option<CheckReturn> {
        if matches!(self, Self::Unimplemented) {
            return None;
        }
        let check_return = match (&self, correct) {
            (Self::Unimplemented, _) => unreachable!(),
            (_, Self::Unimplemented) => {
                CheckReturn::Unchecked(crate::execution::result::Unchecked::MissingOuput(self))
            }
            (Self::Number(a), Self::Number(b)) => match a.cmp(b) {
                std::cmp::Ordering::Equal => CheckReturn::Passed,
                unequal => {
                    CheckReturn::Failed(CheckError::Incorrect(self, correct.clone(), Some(unequal)))
                }
            },
            (Self::String(a), Self::String(b)) => {
                if a == b {
                    CheckReturn::Passed
                } else {
                    CheckReturn::Failed(CheckError::Incorrect(self, correct.clone(), None))
                }
            }
            (_, _) => CheckReturn::Failed(CheckError::WrongFormat(self, correct.clone())),
        };
        Some(check_return)
    }
}

/* Unimplemented */
impl From<()> for SolutionPart {
    fn from(_value: ()) -> Self {
        Self::Unimplemented
    }
}
//? The only reason this needs nightly: Allowing the user to close their solution function with todo!() in peace
impl From<!> for SolutionPart {
    fn from(_value: !) -> Self {
        Self::Unimplemented
    }
}

/* Integer */
macro_rules! impl_from_int {
        ($($t:ty),* $(,)?) => {
            $(
            impl From<$t> for SolutionPart {
                fn from(value: $t) -> Self {
                    Self::Number(rust_decimal::Decimal::from(value))
                }
            }
            )*
        };
    }

impl_from_int!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

/* Float */
macro_rules! impl_from_float {
   ($($t:ty),* $(,)?) => {
            $(
            impl From<$t> for SolutionPart {
                fn from(value: $t) -> Self {
                    if value.is_finite(){
                        Self::Number(rust_decimal::Decimal::try_from(value)
                            .expect("Internal handling of solution pats is limited to -2^96 < m < 2^96. Your float exceeded this range. Double-check that AoC is really asking for a value bigger than that. If so, let me know."))
                    } else {
                        Self::String(value.to_string()) //NaN or INF
                    }



                }
            }
            )*
        };
}

impl_from_float!(f32, f64);

/* Direct Number */
impl From<rust_decimal::Decimal> for SolutionPart {
    fn from(value: rust_decimal::Decimal) -> Self {
        Self::Number(value)
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
