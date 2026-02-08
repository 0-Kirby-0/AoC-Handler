use crate::{
    api::display::display_impls::{display_day_chunk, display_part_return},
    execution::result::{DayReturn, PartReturn},
    time_key::{TimeDetailDay, TimeDetailDayAndPart, TimeDetailNone, TimeKey},
};

mod display_impls;
mod test;

pub fn render_multiple_year_results(
    year_returns: impl Iterator<Item = (TimeKey<TimeDetailNone>, impl Iterator<Item = DayReturn>)>,
) -> String {
    //* Headers are taken care of by the render_year_returns.
    year_returns
        .map(|(time_key, day_returns)| render_year_returns(time_key, day_returns))
        .fold(String::default(), |acc, year| acc + &year + "\n")
}

pub fn render_year_returns(
    time_key: TimeKey<TimeDetailNone>,
    day_returns: impl Iterator<Item = DayReturn>,
) -> String {
    time_key.header()
        + "\n"
        + &day_returns
            .enumerate()
            .map(|(i, e)| (i + 1, e)) //Attach day labels, one indexed
            .collect::<Box<[_]>>()
            .chunk_by(|(_, a), (_, b)| a == b)
            .map(display_day_chunk)
            .fold(String::default(), |acc, chunk| acc + &chunk + "\n")
}

//? Deliberaly differs from the display of multiple days in sequence.
pub fn render_day_return(time_key: TimeKey<TimeDetailDay>, day_return: &DayReturn) -> String {
    let [p1, p2] = day_return;

    time_key.header()
        + "\n"
        + &if p1 == p2 {
            display_part_return(p1)
        } else {
            format!(
                "[2mPart One[22m - {}\n[2mPart Two[22m - {}",
                display_part_return(p1),
                display_part_return(p2)
            )
        }
}

//? Deliberately differs from the display of multiple parts in sequence.
pub fn render_part_return(
    time_key: TimeKey<TimeDetailDayAndPart>,
    part_return: &PartReturn,
) -> String {
    time_key.header() + "\n" + &display_part_return(part_return)
}

impl TimeKey<TimeDetailNone> {
    fn header(self) -> String {
        format!("[1m{}[22m", self.year)
    }
}
impl TimeKey<TimeDetailDay> {
    fn header(self) -> String {
        format!("{} — [1mDay {}[22m", self.deref().header(), self.detail.0)
    }
}
impl TimeKey<TimeDetailDayAndPart> {
    fn header(self) -> String {
        format!(
            "{} — [1mPart {}[22m",
            self.deref().header(),
            self.detail.part
        )
    }
}
