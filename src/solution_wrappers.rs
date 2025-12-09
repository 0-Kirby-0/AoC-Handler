pub struct Solution {
    pub part_1: SolutionPart,
    pub part_2: SolutionPart,
}

#[derive(Clone)]
pub enum SolutionPart {
    Unfinished,
    Integer(String),
    Real(f64),
    String(String),
}

impl SolutionPart {
    pub fn is_unfinished(&self) -> bool {
        matches!(self, Self::Unfinished)
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
        Self::String(value.to_owned())
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
