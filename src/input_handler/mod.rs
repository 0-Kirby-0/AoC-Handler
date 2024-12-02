#![allow(dead_code, unused_variables)]

use std::{
    io::{self, Read},
    path::PathBuf,
};

use downloader::Downloader;

use crate::{Day, Year};

mod cache;
mod downloader;

pub struct InputHandler {
    year: Year,
    downloader: Downloader,
}

impl InputHandler {
    pub fn new(year: Year) -> Self {
        InputHandler {
            year,
            downloader: Downloader::new(Self::get_token()),
        }
    }
    pub fn get_day_input(&self, day: Day) -> String {
        let sub_path = format!("{}/day{}_input.txt", self.year, day).into();

        if !cache::is_cached(&sub_path) {
            let loaded = self.downloader.get(self.year, day);
            cache::cache(&sub_path, loaded.as_bytes()).expect("Unable to cache input.");
        }

        let mut out_string = String::default();
        cache::get_cached(&sub_path)
            .read_to_string(&mut out_string)
            .expect("Unable to read cached input to string.");
        out_string
    }

    fn get_token() -> String {
        let sub_path: PathBuf = "token.txt".into();
        let mut token: String = Default::default();

        if cache::is_cached(&sub_path) {
            cache::get_cached(&sub_path)
                .read_to_string(&mut token)
                .expect("Unable to read token.");
            return token;
        }

        println!("Please provide your AoC access token:");
        println!("- 128 character hexadecimal string");
        println!("- Found by inspecting AoC page requests in your browser");
        println!(
            "- Will be cached unencrypted in: {}",
            dirs::cache_dir()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "cache directory".to_string())
        );
        println!(
            "Should there be issues with the currently cached token, manually delete it there."
        );

        io::stdin()
            .read_to_string(&mut token)
            .expect("Unable to read given token.");

        token = token.trim().to_lowercase().to_owned(); //cleaning up, after all we're nice :)

        Self::validate_token(&token).unwrap();

        cache::cache(&sub_path, token.as_bytes()).expect("Unable to cache token.");
        token
    }

    fn validate_token(token: &str) -> Result<(), String> {
        if token.len() != 128 {
            Err("Token is incorrect length. Should be 128 characters.".to_string())
        } else if !(token.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f'))) {
            Err("Token contains invalid characters. Must be hexadecimal.".to_string())
        } else {
            Ok(())
        }
    }
}
