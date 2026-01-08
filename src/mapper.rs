use crate::{Day, DayMapper, Solver};

pub struct Mapper {
    pub map: Box<dyn Fn(Day) -> Solver>,
}

impl<T: DayMapper> From<T> for Mapper {
    fn from(value: T) -> Self {
        Self {
            map: Box::new(|day| T::map(day)),
        }
    }
}
