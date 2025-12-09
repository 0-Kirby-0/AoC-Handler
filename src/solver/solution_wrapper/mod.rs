mod check;
mod impls;

pub use check::CheckResult;

#[derive(Debug, Clone)]
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

    pub fn check(&self, correct: &Self) -> CheckResult {
        use std::cmp::Ordering;
        match (self, correct) {
            (Self::Unfinished, _) => CheckResult::Unfinished,
            (_, Self::Unfinished) => CheckResult::Unchecked(self.clone()),
            (a, b) if std::mem::discriminant(a) != std::mem::discriminant(b) => {
                CheckResult::WrongFormat(self.clone())
            }
            (Self::Integer(a), Self::Integer(b)) => {
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
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        };
                    }

                    // Same sign: compare absolute values
                    let abs_cmp = match a.len().cmp(&b.len()) {
                        Ordering::Less => Ordering::Less,
                        Ordering::Greater => Ordering::Greater,
                        Ordering::Equal => a.cmp(b),
                    };

                    // For negatives, larger absolute value means smaller number
                    if neg_a { abs_cmp.reverse() } else { abs_cmp }
                }

                match compare_strings_as_ints(a, b) {
                    Ordering::Equal => CheckResult::Correct,
                    Ordering::Less => CheckResult::TooLow(self.clone(), correct.clone()),
                    Ordering::Greater => CheckResult::TooHigh(self.clone(), correct.clone()),
                }
            }
            (Self::Real(a), Self::Real(b)) => a.partial_cmp(b).map_or_else(
                || {
                    if a.is_normal() || a.is_subnormal() {
                        CheckResult::Unchecked(self.clone())
                    } else {
                        CheckResult::WrongFormat(self.clone())
                    }
                },
                |cmp| match cmp {
                    Ordering::Equal => CheckResult::Correct,
                    Ordering::Less => CheckResult::TooLow(self.clone(), correct.clone()),
                    Ordering::Greater => CheckResult::TooHigh(self.clone(), correct.clone()),
                },
            ),
            (Self::String(a), Self::String(b)) => {
                if a == b {
                    CheckResult::Correct
                } else {
                    CheckResult::Incorrect(self.clone(), correct.clone())
                }
            }
            (_, _) => unreachable!(),
        }
    }
}
