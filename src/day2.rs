use std::str::FromStr;

use regex::Regex;

struct PasswordEntry {
    first_num: u32,
    second_num: u32,
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
            first_num: groups.get(1).unwrap().as_str().parse().unwrap(),
            second_num: groups.get(2).unwrap().as_str().parse().unwrap(),
            character: groups.get(3).unwrap().as_str().to_string(),
            password: groups.get(4).unwrap().as_str().to_string(),
        })
    }
}

fn parse_lines(lines: &Vec<String>) -> Vec<PasswordEntry> {
    lines
        .iter()
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn count_sled_passwords(lines: &Vec<String>) -> u32 {
    let pass_entries = parse_lines(&lines);
    
    pass_entries.iter().filter(|&x| {
        let count = x.password.split(&x.character).count() as u32;
        if count - 1 >= x.first_num && count - 1 <= x.second_num {
            true
        } else {
            false
        }
    }).count() as u32
}

pub fn count_toboggan_passwords(lines: &Vec<String>) -> u32 {
    let pass_entries = parse_lines(&lines);
    
    pass_entries.iter().filter(|&x| {
        let first = x.password[(x.first_num - 1) as usize .. x.first_num as usize] == x.character;
        let second = x.password[(x.second_num - 1) as usize .. x.second_num as usize] == x.character;

        first ^ second
    }).count() as u32
}
