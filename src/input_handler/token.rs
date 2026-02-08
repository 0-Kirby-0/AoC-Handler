use super::cache;

pub struct Token {
    token: String,
    path: std::path::PathBuf,
}

/*
A note on security:
Storing a token in plain text in file, no encryption, not even perm restrictions, feels horrible.
However we have to acknowledge that this token is for AoC, a toy web page. There are no consequences to a token leak.
Any security overhead we accept here would just be posturing, it's not worth it.
*/

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
        cache::text::get_cached(path).expect("Unable to get cached token.")
    }
    fn cache_token(path: &std::path::Path, token: &str) {
        cache::text::cache(path, token)
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
