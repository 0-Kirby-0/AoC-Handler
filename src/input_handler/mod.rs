use crate::{
    Day, Year,
    time_key::{TimeDetailDay, TimeKey},
};

mod cache;
mod token;
use token::Token;

pub struct Client {
    client: reqwest::blocking::Client,
    token: Token,
}

impl Client {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static(
                "AoC-Handler (https://github.com/0-Kirby-0/AoC-Handler) <3",
            ),
        );
        Self {
            client: reqwest::blocking::Client::builder()
                .default_headers(headers)
                .build()
                .expect("Couldn't build reqwest client."),
            token: Token::new(),
        }
    }

    /// Fetches the input data for a given day and year of Advent of Code.
    /// Preferentially sources from internal cache.
    pub fn get_day_input(&self, key: TimeKey<TimeDetailDay>) -> Result<String, InputError> {
        let (year, day) = key.to_primitive();
        let sub_path: std::path::PathBuf = format!("{year}/day{day}_input.txt").into();

        //If the file is already cached, we trust that it's fine, because we wouldn't cache a broken file.
        //(And if we did, the user can delete the cache themselves)
        if cache::is_cached(&sub_path) {
            //This can only fail if the file is *there*, but couldn't be read to string. That's weird.
            return cache::text::get_cached(&sub_path).map_err(|_| InputError::CacheRead(sub_path));
        }

        let url = format!("https://adventofcode.com/{year}/day/{day}/input");

        let input = self
            .client
            .get(url)
            .header("Cookie", format!("session={}", self.token))
            .send()?
            .text()?
            .trim()
            .to_owned();

        match input.as_str() {
            "Puzzle inputs differ by user.  Please log in to get your puzzle input." => {
                self.token.invalidate();
                Err(InputError::InvalidToken)
            }
            //? The AoC website appears to treat any day 0<x<100 as valid for checking, in which case it returns the "please don't repeatedly request"
            //? Since TimeKey can only be valid dates, we are safe from erroneously hammering the connection
            "404 Not Found"
            | "Please don't repeatedly request this endpoint before it unlocks! The calendar countdown is synchronized with the server time; the link will be enabled on the calendar the instant this puzzle becomes available." => {
                Err(InputError::NotFound { year, day })
            }

            _ => Ok(()),
        }?;

        cache::text::cache(&sub_path, &input)?;
        Ok(input)
    }
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum InputError {
    #[error("Cached input file  at {0} corrupted or unusable")]
    CacheRead(std::path::PathBuf),
    #[error("Unable to cache input file. {0}")]
    CacheWrite(#[source] std::rc::Rc<std::io::Error>),
    #[error("Connection to AoC servers failed. {0}")]
    Request(#[source] std::rc::Rc<reqwest::Error>),
    #[error("Session token invalid. Please try again to be prompted for a new token")]
    InvalidToken,
    #[error("No input data found for {year}-{day}")]
    NotFound { year: Year, day: Day },
}

impl From<std::io::Error> for InputError {
    fn from(value: std::io::Error) -> Self {
        Self::CacheWrite(std::rc::Rc::new(value))
    }
}
impl From<reqwest::Error> for InputError {
    fn from(value: reqwest::Error) -> Self {
        Self::Request(std::rc::Rc::new(value))
    }
}
