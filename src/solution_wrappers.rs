pub struct Solution {
    pub part_1: SolutionPart,
    pub part_2: SolutionPart,
}

#[derive(Clone, Copy)]
pub enum SolutionPart {
    Unfinished,
    Number(usize),
}
impl From<()> for SolutionPart {
    fn from(value: ()) -> Self {
        Self::Unfinished
    }
}
impl From<i32> for SolutionPart {
    fn from(value: i32) -> Self {
        assert!(
            value >= 0,
            "Handling negative solution values is not implemented."
        );
        Self::Number(value as usize)
    }
}
impl From<usize> for SolutionPart {
    fn from(value: usize) -> Self {
        Self::Number(value)
    }
}
impl From<u128> for SolutionPart {
    fn from(value: u128) -> Self {
        Self::Number(value as usize)
    }
}
impl From<u64> for SolutionPart {
    fn from(value: u64) -> Self {
        Self::Number(value as usize)
    }
}
impl From<u32> for SolutionPart {
    fn from(value: u32) -> Self {
        Self::Number(value as usize)
    }
}
impl From<u16> for SolutionPart {
    fn from(value: u16) -> Self {
        Self::Number(value as usize)
    }
}
impl From<u8> for SolutionPart {
    fn from(value: u8) -> Self {
        Self::Number(value as usize)
    }
}

impl core::fmt::Display for SolutionPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unfinished => write!(f, "Unfinished"),
            Self::Number(n) => write!(f, "'{n}'"),
        }
    }
}

impl SolutionPart {
    pub fn is_unfinished(&self) -> bool {
        matches!(self, Self::Unfinished)
    }
}
