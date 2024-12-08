use chrono::{Datelike, Local};
use clap::ValueEnum;
use reqwest::blocking::Client;
use rookie::{brave, chrome, firefox};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::process::exit;
#[cfg(target_os = "macos")]
#[derive(ValueEnum, Debug, Clone)]
pub enum Browsers {
    Brave,
    Chrome,
    Firefox,
    Safari,
}

#[cfg(not(target_os = "macos"))]
#[derive(ValueEnum, Debug, Clone)]
pub enum Browsers {
    Brave,
    Chrome,
    Firefox,
}

impl Display for Browsers {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Browsers::Brave => write!(f, "Brave"),
            Browsers::Chrome => write!(f, "Chrome"),
            Browsers::Firefox => write!(f, "Firefox"),
            #[cfg(target_os = "macos")]
            Browsers::Safari => write!(f, "Safari"),
        }
    }
}

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

    pub fn get_result(&self, day: Option<u32>, browser: Browsers) -> String {
        let domains = vec![String::from(ADVENT_OF_CODE_WEBSITE)];
        let cookies = match browser {
            #[cfg(target_os = "macos")]
            Browsers::Safari => safari(Some(domains)),
            Browsers::Brave => brave(Some(domains)),
            Browsers::Chrome => chrome(Some(domains)),
            Browsers::Firefox => firefox(Some(domains)),
        };

        let cookies = cookies.unwrap_or_else(|_err| {
            println!("AOC Cookies not found in {browser} browser.");
            exit(0); // Exit if cookies are not found
        });

        let cookie = cookies.get(0).unwrap_or_else(|| {
            println!("AOC Cookies not found in {browser} browser.");
            exit(1); // Exit with status 1 if no cookie is found
        });

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
