pub type Year = u16;
pub type Day = u8;
pub type Part = u8;

#[derive(thiserror::Error, Debug)]
pub enum IndexingError {
    #[error("Invalid Year: Advent of Code started in 2015")]
    YearTooEarly,
    #[error("Invalid Day: {0}")]
    InvalidDay(String),
    #[error("Invalid Part: {0}")]
    InvalidPart(String),
}

pub fn get_day_range(year: Year) -> std::ops::RangeInclusive<Day> {
    match year {
        2015..2025 => 1..=25,
        2025.. => 1..=12,
        _ => unreachable!(),
    }
}

pub fn validate_year(year: Year) -> Result<(), IndexingError> {
    (year >= 2015)
        .then_some(())
        .ok_or(IndexingError::YearTooEarly)
}

pub fn validate_day(year: Year, day: Day) -> Result<(), IndexingError> {
    if day == 0 {
        return Err(IndexingError::InvalidDay(
            "Days are one-indexed, the first of december is day number 1.".to_string(),
        ));
    }
    validate_year(year)?;
    get_day_range(year)
        .contains(&day)
        .then_some(())
        .ok_or_else(|| {
            IndexingError::InvalidDay(format!(
                "{year} only has {} puzzle days.",
                get_day_range(year).end()
            ))
        })
}
pub fn validate_part(part: Part) -> Result<(), IndexingError> {
    match part {
        0 => Err(IndexingError::InvalidPart(
            "Parts are one-indexed, the first part is part number 1.".to_string(),
        )),
        1..=2 => Ok(()),
        _ => Err(IndexingError::InvalidPart(
            "Advent of Code puzzles have only two parts.".to_string(),
        )),
    }
}
