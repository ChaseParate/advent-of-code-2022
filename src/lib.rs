use reqwest::blocking::Client;
use reqwest::header::COOKIE;

fn get_session_token() -> String {
    String::from(env!("SESSION_TOKEN"))
}

pub struct Puzzle {
    day: u8,
    year: u16,
}
impl Puzzle {
    const YEAR: u16 = 2022;

    pub const fn new(day: u8) -> Self {
        if day < 1 || day > 25 {
            panic!("Invalid day given")
        }

        Puzzle {
            day,
            year: Puzzle::YEAR,
        }
    }

    pub fn get_input(&self) -> Result<String, reqwest::Error> {
        let url = format!(
            "https://adventofcode.com/{}/day/{}/input",
            self.year, self.day
        );
        let session_cookie = format!("session={}", get_session_token());

        let client = Client::new();
        let request = client.get(url).header(COOKIE, session_cookie).build()?;
        let response = client.execute(request)?;
        response.error_for_status_ref()?;

        response.text()
    }
}
