use std::str::FromStr;

use regex::Regex;

struct PasswordEntry {
    min: u32,
    max: u32,
    character: String,
    password: String
}

#[derive(Debug)]
struct PasswordEntryParseError {

}

impl FromStr for PasswordEntry {
    type Err = PasswordEntryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PASSWORD_ENTRY_RE: Regex = Regex::new(r"(\d+)-(\d+)\s(.):\s(.+)").unwrap();
        }

        let groups = PASSWORD_ENTRY_RE.captures(s).unwrap();
        Result::Ok(PasswordEntry {
            min: groups.get(1).unwrap().as_str().parse().unwrap(),
            max: groups.get(2).unwrap().as_str().parse().unwrap(),
            character: groups.get(3).unwrap().as_str().to_string(),
            password: groups.get(4).unwrap().as_str().to_string(),
        })
    }
}

pub fn count_passwords(lines: &Vec<String>) -> u32 {
    let pass_entries: Vec<PasswordEntry> = lines
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();
    
    pass_entries.iter().filter(|&x| {
        let count = x.password.split(&x.character).count() as u32;
        if count - 1 >= x.min && count - 1 <= x.max {
            true
        } else {
            false
        }
    }).count() as u32
}
