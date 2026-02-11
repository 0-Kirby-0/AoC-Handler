use super::solution_part::{CheckError, SolutionPart};
/*
General:
    Not mapped (no solver available)
    Not implemented (returns unit, only known after running *something*)
    Okay

Test:
    No check input
    No check output
    Checked with error
    Check passed

Run:
    No run input
    Run success

Final structure:
    May only be a top level error.
    May be a check return, if only that was requested.
    May be a check return if failed and no run return, despite request
    May be a check return (success or specific error) and a run return
*/

pub type DayReturn = [PartReturn; 2];
pub type PartReturn = Result<PartOutput, AcquisitionError>;

///Trying to get/run the solution code led to a universal issue
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AcquisitionError {
    NotMapped,     //The user didn't provide code
    Unimplemented, //The mapped code returned (), implying it was left blank.
}

#[derive(Debug, Clone)]
pub enum PartOutput {
    Checked(CheckReturn),
    CheckedAndRan(CheckedRunReturn),
}

#[derive(Debug, Clone)]
pub enum CheckReturn {
    Passed,
    Unchecked(Unchecked),
    Failed(CheckError),
}

#[derive(Debug, Clone)]
pub enum CheckedRunReturn {
    Ok(RunReturn),
    Unchecked { reason: Unchecked, ret: RunReturn },
    CheckFailed(CheckError),
    RunFailed(crate::input_handler::InputError),
}

#[derive(Debug, Clone)]
pub struct RunReturn {
    pub solution_part: SolutionPart,
    pub time_taken: std::time::Duration,
}

#[derive(Debug, Clone)]
pub enum Unchecked {
    Elided,
    ElideMismatch(SolutionPart),
    MissingInput,
    MissingOuput(SolutionPart),
}

impl PartialEq for PartOutput {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Checked(a), Self::Checked(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialEq for CheckReturn {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Passed, Self::Passed)
                | (
                    Self::Unchecked(Unchecked::MissingInput),
                    Self::Unchecked(Unchecked::MissingInput)
                )
        )
    }
}
