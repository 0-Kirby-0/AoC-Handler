use crate::{
    SolutionPart,
    execution::{
        Request,
        result::{
            AcquisitionError, CheckReturn, CheckedRunReturn, DayReturn, PartOutput, PartReturn,
            RunReturn, Unchecked,
        },
        solution_part::CheckError,
    },
    time_key::PartInternal,
};

pub fn display_day_chunk(chunk: &[(usize, DayReturn)]) -> String {
    let (day_start, [p1, p2]) = &chunk[0];
    let day_start = format!("{day_start}");

    let (length, day_print) = if chunk.len() == 1 {
        (day_start.len(), format!("[1m{day_start}[22m"))
    } else {
        let (day_end, _) = chunk.last().unwrap();
        let day_end = format!("{day_end}");
        let len = day_start.len() + 1 + day_end.len();

        (
            len,
            format!("[4m[1m{day_start}[22m-[1m{day_end}[22m[24m"),
        )
    };

    let parts_print = if p1 == p2 {
        format!("- {}", display_part_return(p1))
    } else {
        format!(
            "┬ {}\n{:5} └ {}",
            display_part_return(p1),
            "",
            display_part_return(p2)
        )
    };
    let length = 5usize.saturating_sub(length);
    format!("{:length$}{day_print} {parts_print}", "")
}

pub fn display_part_return(pr: &PartReturn) -> String {
    match pr {
        Err(ae) => format!("[1m{ae}[22m"),
        Ok(po) => format!("{po}"),
    }
}

impl std::fmt::Display for AcquisitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotMapped => write!(f, "No solution provided"),
            Self::Unimplemented => write!(f, "Unimplemented"),
        }
    }
}

impl std::fmt::Display for PartOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Checked(cr) => write!(f, "{cr}"),
            Self::CheckedAndRan(crr) => write!(f, "{crr}"),
        }
    }
}

impl std::fmt::Display for CheckReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Passed => write!(f, "[32m[1mPassed[22m[39m"),
            Self::Failed(ce) => write!(f, "[31m[1mFailed:[22m {ce}[39m"),
            Self::Unchecked(uc) => write!(f, "[33m{uc}[39m"),
        }
    }
}

impl std::fmt::Display for CheckedRunReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok(rr) => write!(f, "{rr}"),
            Self::Unchecked { reason, ret } => write!(f, "{ret} [33m[2m{reason}[22m[39m"),
            Self::CheckFailed(ce) => write!(f, "[31m[1mTest Failed:[22m {ce}[39m"),
            Self::RunFailed(ie) => write!(f, "[31m[1mError:[22m {ie}[39m"),
        }
    }
}

impl std::fmt::Display for RunReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[2m{:>8.2?}[22m [1m[32m{:48}[39m[22m",
            self.time_taken, self.solution_part
        )
    }
}

impl std::fmt::Display for Unchecked {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoInput => write!(f, "No test input provided, unable to test"),
            Self::NoOuput(sp) => {
                write!(f, "Test returned [4m{sp}[24m, no answer to check against")
            }
        }
    }
}

impl std::fmt::Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Incorrect(a, b, None) => {
                write!(f, "Result was [4m{a}[24m, should be [4m{b}[24m")
            }
            Self::Incorrect(a, b, Some(std::cmp::Ordering::Greater)) => {
                write!(
                    f,
                    "Result was [4m{a}[24m, which is [1mtoo high[22m. Should be [4m{b}[24m"
                )
            }
            Self::Incorrect(a, b, Some(std::cmp::Ordering::Less)) => {
                write!(
                    f,
                    "Result was [4m{a}[24m, which is [1mtoo low[22m. Should be [4m{b}[24m"
                )
            }
            Self::WrongFormat(a, b) => write!(
                f,
                "Result [4m{a}[24m was of a [1mdifferent format[22m ([4m{}[24m) than the provided solution ([4m{}[24m)",
                a.variant_name(),
                b.variant_name()
            ),
            Self::Incorrect(_, _, Some(std::cmp::Ordering::Equal)) => {
                unreachable!("CheckError was marked as Incorrect, but both candidates were equal.")
            }
        }
    }
}

impl SolutionPart {
    fn variant_name(&self) -> String {
        match self {
            Self::Unimplemented => "Unimplemented".to_string(), //Should definitionally never be called
            Self::Number(_) => "Number".to_string(),
            Self::String(_) => "String".to_string(),
        }
    }
}

impl std::fmt::Display for SolutionPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desired_width = f.width().unwrap_or(0);
        match self {
            Self::Unimplemented => write!(f, "Unimplemented"),
            Self::Number(n) => write!(f, "{n:<desired_width$}"),
            Self::String(s) => write!(f, "{s:<desired_width$}"),
        }
    }
}

impl std::fmt::Display for PartInternal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::One => write!(f, "One"),
            Self::Two => write!(f, "Two"),
        }
    }
}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Check => write!(f, "[1m[4mTesting[22m[24m"),
            Self::CheckAndRun => write!(f, "[1m[4mRunning[22m[24m"),
        }
    }
}
