use std::num::NonZeroU8;

pub type Year = u16;
pub type Day = u8;
pub type Part = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeKey<TimeDetail> {
    pub year: Year,
    pub detail: TimeDetail,
}

pub trait TimeDetail {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeDetailNone;
impl TimeDetail for TimeDetailNone {}

#[derive(Debug, Clone, Copy)]
pub struct TimeDetailDay(pub NonZeroU8);
impl TimeDetail for TimeDetailDay {}

#[derive(Debug, Clone, Copy)]
pub struct TimeDetailDayAndPart {
    pub day: NonZeroU8,
    pub part: PartInternal,
}
impl TimeDetail for TimeDetailDayAndPart {}

#[derive(Debug, Clone, Copy)]
pub enum PartInternal {
    One,
    Two,
}

impl<TimeDetail> TimeKey<TimeDetail> {
    fn max_days(year: Year) -> Day {
        match year {
            2015..2025 => 25,
            2025.. => 12,
            _ => unreachable!(),
        }
    }
}

impl TimeKey<TimeDetailNone> {
    pub fn new(year: Year) -> Result<Self, TimeKeyError> {
        (year >= 2015).ok_or(TimeKeyError::YearTooLow(year))?;

        Ok(Self {
            year,
            detail: TimeDetailNone,
        })
    }

    pub fn to_primitive(self) -> Year {
        self.year
    }

    pub fn iterate(self) -> impl DoubleEndedIterator<Item = TimeKey<TimeDetailDay>> {
        let Self { year, detail: _ } = self;
        (1..=Self::max_days(year))
            .map(|day| unsafe { NonZeroU8::new_unchecked(day) }) //Safety: Range starts at 1
            .map(move |day| TimeKey {
                year,
                detail: TimeDetailDay(day),
            })
    }
}
impl TimeKey<TimeDetailDay> {
    pub fn new(year: Year, day: Day) -> Result<Self, TimeKeyError> {
        let TimeKey::<TimeDetailNone> {
            year,
            detail: TimeDetailNone,
        } = TimeKey::<TimeDetailNone>::new(year)?; //Hooking into validation

        // Check Day
        let max_days = Self::max_days(year);
        (day <= max_days).ok_or(TimeKeyError::DayTooHigh {
            year,
            day,
            max_days,
        })?;
        let day = NonZeroU8::new(day).ok_or(TimeKeyError::DayZero)?;

        Ok(Self {
            year,
            detail: TimeDetailDay(day),
        })
    }

    pub fn to_primitive(self) -> (Year, Day) {
        (self.year, self.detail.0.get())
    }

    pub fn deref(self) -> TimeKey<TimeDetailNone> {
        TimeKey {
            year: self.year,
            detail: TimeDetailNone,
        }
    }

    pub fn both(self) -> [TimeKey<TimeDetailDayAndPart>; 2] {
        {
            let Self {
                year,
                detail: TimeDetailDay(day),
            } = self;
            [
                TimeKey {
                    year,
                    detail: TimeDetailDayAndPart {
                        day,
                        part: PartInternal::One,
                    },
                },
                TimeKey {
                    year,
                    detail: TimeDetailDayAndPart {
                        day,
                        part: PartInternal::Two,
                    },
                },
            ]
        }
    }
}

impl TimeKey<TimeDetailDayAndPart> {
    pub fn new(year: Year, day: Day, part: Part) -> Result<Self, TimeKeyError> {
        let TimeKey::<TimeDetailDay> {
            year,
            detail: TimeDetailDay(day),
        } = TimeKey::<TimeDetailDay>::new(year, day)?; //Hooking into validation

        let part = match part {
            0 => Err(TimeKeyError::PartZero),
            1 => Ok(PartInternal::One),
            2 => Ok(PartInternal::Two),
            invalid => Err(TimeKeyError::PartTooHigh(invalid)),
        }?;

        Ok(Self {
            year,
            detail: TimeDetailDayAndPart { day, part },
        })
    }

    pub fn to_primitive(self) -> (Year, Day, Part) {
        {
            let Self { year, detail } = self;
            let TimeDetailDayAndPart { day, part } = detail;
            let part = match part {
                PartInternal::One => 1,
                PartInternal::Two => 2,
            };
            (year, day.get(), part)
        }
    }

    pub fn deref(self) -> TimeKey<TimeDetailDay> {
        let Self { year, detail } = self;
        let TimeDetailDayAndPart { day, .. } = detail;
        TimeKey {
            year,
            detail: TimeDetailDay(day),
        }
    }
}

#[derive(thiserror::Error, Debug, PartialEq, Eq, Hash)]
pub enum TimeKeyError {
    #[error("Invalid Year: Advent of Code started in 2015")]
    YearTooLow(Year),
    #[error("Invalid Day: Days are one-indexed, the first of december is day number 1.")]
    DayZero,
    #[error("Invalid Day: {day} is too high. {year} only has {max_days} days. ")]
    DayTooHigh { year: Year, day: Day, max_days: Day },
    #[error("Invalid Part: Parts are one-indexed, the first part is part number 1.")]
    PartZero,
    #[error("Invalid Part: Advent of Code puzzles have two parts, tried to get part {0}.")]
    PartTooHigh(Part),
}
