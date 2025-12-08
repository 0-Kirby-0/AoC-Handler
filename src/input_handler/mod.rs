use crate::{Day, Year};

mod cache;

pub struct InputHandler {
    client: reqwest::blocking::Client,
    token: Token,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            token: Token::new(),
        }
    }

    /// Fetches the input data for a given day and year of Advent of Code.
    /// Preferentially sources from internal cache.
    pub fn get_day_input(&self, year: Year, day: Day) -> String {
        let sub_path: std::path::PathBuf = format!("{year}/day{day}_input.txt").into();

        //If the file is already cached, we trust that it's fine, because we wouldn't cache a broken file.
        //(And if we did, the user can delete the cache themselves)
        if cache::is_cached(&sub_path) {
            return cache::text::get_cached_text(&sub_path)
                .expect("Couldn't get cached input file, even though it exists.");
        }

        let url = format!("https://adventofcode.com/{year}/day/{day}/input");

        let input = self
            .client
            .get(url)
            .header("Cookie", format!("session={}", self.token))
            .send()
            .expect("Failed to send request to AoC servers. Please check your internet connection.")
            .text()
            .expect("Failed to get response text from AoC servers. Please check if their servers are offline and try again later.")
            .trim()
            .to_owned();

        if input == "Puzzle inputs differ by user.  Please log in to get your puzzle input." {
            self.token.invalidate();
            panic!("Cached Advent of Code session token failed to validate. Please try again to be prompted for the updated token.");
            //? I cannot be bothered to build a loop that appropriately connects back to this same point.
            //? This harness is called repeatedly on solution re-compiles anyway, one more is fine.
        }

        cache::text::cache_text(&sub_path, &input).expect("Unable to cache input file.");
        input
    }
}

struct Token {
    token: String,
    path: std::path::PathBuf,
}

/*
A note on security:
Storing a token in plain text in file, no encryption, not even perm restrictions, feels horrible.
However we have to acknowledge that this token is for AoC, a toy web page. There are no consequences of a token leak.
Any security overhead we accept here would just be posturing, it's not worth it. */

impl Token {
    pub fn new() -> Self {
        let path: std::path::PathBuf = "token.txt".into();

        Self {
            token: if cache::is_cached(&path) {
                Self::get_token_from_cache(&path)
            } else {
                let token = Self::get_token_from_user();
                Self::cache_token(&path, &token);
                token
            },
            path,
        }
    }
    pub fn invalidate(&self) {
        cache::clear_cached(&self.path).expect("Unable to clear cached token file.");
    }

    fn get_token_from_cache(path: &std::path::Path) -> String {
        cache::text::get_cached_text(path).expect("Unable to get cached token.")
    }
    fn cache_token(path: &std::path::Path, token: &str) {
        cache::text::cache_text(path, token)
            .unwrap_or_else(|e| eprintln!("Unable to cache given token: {e}"));
    }

    fn get_token_from_user() -> String {
        println!("Please provide your AoC access token:");
        println!("- 128 character hexadecimal string");
        println!("- Found by inspecting AoC page requests in your browser");
        println!(
            "- Will be cached unencrypted in: {}",
            dirs::cache_dir()
                .expect("dirs did not provide a cache directory.")
                .display()
        );

        let mut maybe_token = String::default();
        std::io::stdin()
            .read_line(&mut maybe_token)
            .expect("Unable to read given token.");
        let maybe_token = maybe_token.trim().to_string();
        Self::validate_format(&maybe_token).expect("The provided token was invalid:");

        maybe_token
    }

    fn validate_format(maybe_token: &str) -> Result<(), String> {
        if maybe_token.len() != 128 {
            Err("Token is incorrect length. Should be 128 characters.".to_string())
        } else if !(maybe_token
            .chars()
            .all(|c| matches!(c, '0'..='9' | 'a'..='f')))
        {
            Err("Token contains invalid characters. Must be hexadecimal.".to_string())
        } else {
            Ok(())
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token)
    }
}
