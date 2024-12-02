pub struct Downloader {
    client: reqwest::blocking::Client,
    token: String,
}

impl Downloader {
    pub fn new(token: String) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            token,
        }
    }

    pub fn get(&self, year: u16, day: u8) -> String {
        let url = format!("https://adventofcode.com/{year}/day/{day}/input");

        self.client
            .get(url)
            .header("Cookie", format!("session={}", self.token))
            .send()
            .expect("Failed to send request.")
            .text()
            .expect("Failed to get response text.")
    }
}
