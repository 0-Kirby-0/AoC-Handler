use crate::SolutionPart;

#[derive(thiserror::Error, Debug, Clone)]
pub enum CheckError {
    Unfinished,
    NoTestInput,
    Unchecked(SolutionPart),
    WrongFormat(SolutionPart, SolutionPart),
    Incorrect(SolutionPart, SolutionPart, Option<std::cmp::Ordering>),
    Corrupted(SolutionPart),
}

impl CheckError {
    pub fn implies_real_results_are_valuable(&self) -> bool {
        match self {
            Self::NoTestInput | Self::Unchecked(_) => true,
            Self::Unfinished
            | Self::WrongFormat(_, _)
            | Self::Incorrect(_, _, _)
            | Self::Corrupted(_) => false,
        }
    }

    pub fn compare(test: SolutionPart, correct: &SolutionPart) -> Result<(), Self> {
        match (&test, correct) {
            (SolutionPart::Unfinished, _) => Err(Self::Unfinished),
            (_, SolutionPart::Unfinished) => Err(Self::Unchecked(test)),
            (a, b) if std::mem::discriminant(a) != std::mem::discriminant(b) => {
                Err(Self::WrongFormat(test, correct.clone()))
            }
            (SolutionPart::Integer(a), SolutionPart::Integer(b)) => {
                match compare_strings_as_ints(a, b) {
                    std::cmp::Ordering::Equal => Ok(()),
                    ord => Err(Self::Incorrect(test, correct.clone(), Some(ord))),
                }
            }
            (SolutionPart::Real(a), SolutionPart::Real(b)) => match a.partial_cmp(b) {
                None => {
                    if a.is_finite() {
                        Err(Self::Unchecked(test))
                    } else {
                        Err(Self::Corrupted(test)) //test is ither NAN or Infinity, neither is likely to be the intended solution.
                    }
                }
                Some(ord) => match ord {
                    std::cmp::Ordering::Equal => Ok(()),
                    incorrect => Err(Self::Incorrect(test, correct.clone(), Some(incorrect))),
                },
            },

            (SolutionPart::String(a), SolutionPart::String(b)) => {
                if a == b {
                    Ok(())
                } else {
                    Err(Self::Incorrect(test, correct.clone(), None))
                }
            }
            (_, _) => unreachable!(),
        }
    }
}

fn compare_strings_as_ints(a: &str, b: &str) -> std::cmp::Ordering {
    fn split_sign(s: &str) -> (bool, &str) {
        s.strip_prefix('-')
            .map_or((false, s), |digits| (true, digits))
    }

    let (neg_a, a) = split_sign(a);
    let (neg_b, b) = split_sign(b);

    // Different signs: negative < positive
    if neg_a != neg_b {
        return if neg_a {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        };
    }

    // Same sign: compare absolute values
    let abs_cmp = match a.len().cmp(&b.len()) {
        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        std::cmp::Ordering::Equal => a.cmp(b),
    };

    // For negatives, larger absolute value means smaller number
    if neg_a { abs_cmp.reverse() } else { abs_cmp }
}

impl core::fmt::Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unfinished => {
                write!(f, "Unimplemented.")
            }
            Self::NoTestInput => {
                write!(f, "No test input provided.")
            }
            Self::Unchecked(output) => {
                write!(f, "{output}, no answer to check against.")
            }
            Self::WrongFormat(ouput, correct) => {
                write!(
                    f,
                    "{ouput}. Test answer has wrong format to compare. Should be {}, is {}.",
                    ouput.variant_name(),
                    correct.variant_name()
                )
            }
            Self::Incorrect(output, correct, ord) => match ord {
                None => write!(f, "{output} is incorrect. Should be {correct}.",),
                Some(std::cmp::Ordering::Greater) => {
                    write!(f, "{output} is too high. Should be {correct}.",)
                }
                Some(std::cmp::Ordering::Less) => {
                    write!(f, "{output} is too low. Should be {correct}.",)
                }
                _ => unreachable!(),
            },
            Self::Corrupted(output) => {
                write!(f, "{output} is corrupted and not testable.")
            }
        }
    }
}
