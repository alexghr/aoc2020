use std::collections::HashMap;

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

pub fn count_valid_passports(data: &Vec<String>) -> u32 {
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
