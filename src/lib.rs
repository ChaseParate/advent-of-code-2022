use std::io::{self, Write};

use reqwest::blocking::Client;
use reqwest::header::COOKIE;

mod cache {
    use std::fs;
    use std::path::PathBuf;

    use directories::ProjectDirs;

    #[derive(Debug)]
    pub enum File {
        SessionToken,
        PuzzleInput { day: u8 },
    }

    fn get_file_path(file: &File) -> PathBuf {
        let project_dirs = ProjectDirs::from("", "", env!("CARGO_PKG_NAME")).unwrap();

        let mut cache_dir = project_dirs.cache_dir().to_owned();
        cache_dir.push(match file {
            File::SessionToken => String::from("session_token.txt"),
            File::PuzzleInput { day } => format!("puzzle_input_day{day:02}.txt"),
        });
        fs::create_dir_all(cache_dir.parent().unwrap()).unwrap();

        cache_dir
    }

    pub fn read_from_cache(file: &File) -> Option<String> {
        let file_path = get_file_path(file);
        fs::read_to_string(file_path).ok()
    }

    pub fn write_to_cache(file: &File, data: &String) {
        let path = get_file_path(file);
        fs::write(path, data).expect("Unable to write to file");
    }
}

fn get_session_token() -> String {
    let file = cache::File::SessionToken;
    if let Some(session_token) = cache::read_from_cache(&file) {
        return session_token;
    }

    print!("Enter your Advent of Code session token: ");
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let session_token = String::from(buf.trim());
    cache::write_to_cache(&file, &session_token);

    session_token
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
        let file = cache::File::PuzzleInput { day: self.day };
        if let Some(puzzle_input) = cache::read_from_cache(&file) {
            return Ok(puzzle_input);
        }

        let url = format!(
            "https://adventofcode.com/{}/day/{}/input",
            self.year, self.day
        );
        let session_cookie = format!("session={}", get_session_token());

        let client = Client::new();
        let request = client.get(url).header(COOKIE, session_cookie).build()?;
        let response = client.execute(request)?;
        response.error_for_status_ref()?;

        let puzzle_input = String::from(response.text()?.trim());
        cache::write_to_cache(&file, &puzzle_input);

        Ok(puzzle_input)
    }
}
