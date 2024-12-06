use chrono::{Datelike, Local};
use reqwest::blocking::Client;
use rookie::brave;

const ADVENT_OF_CODE_WEBSITE: &str = "adventofcode.com";

pub struct AutoInputScraper {
    day: u32,
    year: i32,
}

impl AutoInputScraper {
    pub fn new() -> Self {
        let now = Local::now();

        Self {
            day: now.day(),
            year: now.year(),
        }
    }

    pub fn get_result(&self, day: Option<u32>) -> String {
        let domains = vec![String::from(ADVENT_OF_CODE_WEBSITE)];
        let cookies = brave(Some(domains)).expect("Cookies not found in Brave.");
        let cookie = cookies.get(0).expect("No cookie found.");

        let day_to_get = if let Some(day_changed) = day {
            day_changed
        } else {
            self.day
        };

        let input_address = format!(
            "https://{}/{}/day/{}/input",
            ADVENT_OF_CODE_WEBSITE, self.year, day_to_get
        );

        let client = Client::new();
        let response = client
            .get(input_address)
            .header("Cookie", format!("{}={}", cookie.name, cookie.value))
            .send()
            .expect("HTTP Request failed.");

        response.text().expect("Failed to get the response text.")
    }
}
