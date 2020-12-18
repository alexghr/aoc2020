use std::collections::HashMap;
use regex::Regex;

fn parse_data(lines: &Vec<String>) -> Vec<HashMap<String, String>> {
    lines.iter().fold(vec![HashMap::new()], |mut acc, x| {
        let last = acc.last_mut().unwrap();
        if x.is_empty() {
            acc.push(HashMap::new());
        } else {
            x.split(" ").for_each(|pair| {
                let mut split = pair.splitn(2, ':');
                let key = split.next().unwrap();
                let value = split.next().unwrap();
                last.insert(key.to_string(), value.to_string());
            });
        }

        acc
    })
}

pub fn count_valid_passports_p1(data: &Vec<String>) -> u32 {
    lazy_static! {
        static ref MANDATORY_FIELDS: Vec<String> = vec![
            "byr".to_string(),
            "iyr".to_string(),
            "eyr".to_string(),
            "hgt".to_string(),
            "hcl".to_string(),
            "ecl".to_string(),
            "pid".to_string(),
        ];

        static ref OPTIONAL_FIELDS: Vec<String> = vec![
            "cid".to_string(),
        ];
    }

    let documents = parse_data(&data);

    documents.iter().filter(|x| {
        MANDATORY_FIELDS.iter().all(|key| x.contains_key(key))
    }).count() as u32
}

type ValidationFn = Box<dyn Fn(&str) -> bool>;

fn make_num_validator(min: u32, max: u32) -> ValidationFn {
    Box::new(move |v| {
        if let Ok(val) = v.parse::<u32>() {
            val >= min && val <= max
        } else {
            false
        }
    })
}

fn make_re_validator(re: Regex) -> ValidationFn {
    Box::new(move |v| {
        re.is_match(v)
    })
}

pub fn count_valid_passports_p2(data: &Vec<String>) -> u32 {
    let documents = parse_data(&data);
    let mut map: HashMap<String, ValidationFn> = HashMap::new();

    map.insert("byr".to_string(), make_num_validator(1920, 2002));
    map.insert("iyr".to_string(), make_num_validator(2010, 2020));
    map.insert("eyr".to_string(), make_num_validator(2020, 2030));

    let in_validator = make_num_validator(59, 76);
    let cm_validator = make_num_validator(150, 193);
    map.insert("hgt".to_string(), Box::new(move |v| {
        lazy_static! {
            static ref HEIGHT_RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
        };

        if let Some(groups) = HEIGHT_RE.captures(v) {
            let height = groups.get(1).unwrap().as_str();
            let unit = groups.get(2).unwrap().as_str();

            if unit == "in" {
                in_validator(height)
            } else if unit == "cm" {
                cm_validator(height)
            } else {
                false
            }
        } else {
            false
        }
    }));

    map.insert("hcl".to_string(), make_re_validator(Regex::new(r"^#[0-9a-f]{6}$").unwrap()));
    map.insert("ecl".to_string(), make_re_validator(Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap()));
    map.insert("pid".to_string(), make_re_validator(Regex::new(r"^\d{9}$").unwrap()));

    documents.iter().filter(|x| {
        map.iter().all(|(key, validator)| {
            if let Some(val) = x.get(key) {
                validator(val)
            } else {
                false
            }
        })
    })
    .count() as u32
    
}
