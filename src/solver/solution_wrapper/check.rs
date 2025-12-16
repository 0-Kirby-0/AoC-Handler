use crate::solver::SolutionPart;

#[derive(Debug)]
pub enum CheckResult {
    Correct,
    Unfinished,
    Unchecked(SolutionPart),
    WrongFormat(SolutionPart),
    Incorrect(SolutionPart, SolutionPart),
    TooLow(SolutionPart, SolutionPart),
    TooHigh(SolutionPart, SolutionPart),
}
impl CheckResult {
    pub fn should_be_printed(&self) -> bool {
        !matches!(self, Self::Correct | Self::Unfinished)
    }
    pub fn implies_real_results_are_valuable(&self) -> bool {
        matches!(self, Self::Correct | Self::Unchecked(_))
    }
}

impl core::fmt::Display for CheckResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Correct | Self::Unfinished => Ok(()),
            Self::Unchecked(output) => {
                write!(f, "Test: {output}, no answer to check against.")
            }
            Self::WrongFormat(ouput) => {
                write!(f, "Test: {ouput}. Test answer has wrong format to compare.",)
            }
            Self::Incorrect(output, correct) => {
                write!(f, "Test: {output} is incorrect. Should be {correct}.",)
            }
            Self::TooHigh(output, correct) => {
                write!(f, "Test: {output} is too high. Should be {correct}.",)
            }
            Self::TooLow(output, correct) => {
                write!(f, "Test: {output} is too low. Should be {correct}.",)
            }
        }
    }
}
