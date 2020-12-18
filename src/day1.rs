use std::collections::{HashSet, HashMap};

const SUM_TARGET: i32 = 2020;

fn parse_lines(lines: &Vec<String>) -> Vec<i32> {
    lines
        .iter()
        .map(|val| val.parse().unwrap())
        .collect()
}

fn find_pair(expense_report: &Vec<i32>) -> Option<(i32, i32)> {
    let hash: HashSet<i32> = expense_report.iter().cloned().collect();
    expense_report.iter().find_map(|x| {
        if hash.contains(&(SUM_TARGET - x)) {
            Some((x.clone(), SUM_TARGET - x))
        } else {
            None
        }
    })
}

fn find_triplet(expense_report: &Vec<i32>) -> Option<(i32, i32, i32)> {
    let mut pairs: HashMap<i32, (i32, i32)> = HashMap::new();
    expense_report.iter().enumerate().for_each(|(i, x)| {
        expense_report.iter().enumerate().for_each(|(j, y)| {
            if j == i {
                return;
            }

            pairs.insert(x + y, (x.clone(), y.clone()));
        })
    });

    expense_report.iter().find_map(|x| { 
        match pairs.get(&(SUM_TARGET - x)) {
            Some((y, z)) => Some((x.clone(), y.clone(), z.clone())),
            None => None
        }
    })
}

pub fn find_product(data: &Vec<String>) -> i32 {
    let expense_report = parse_lines(&data);
    let (a, b) = find_pair(&expense_report).unwrap();

    a * b
}

pub fn find_product_triplet(data: &Vec<String>) -> i32 {
    let expense_report = parse_lines(&data);
    let (x, y, z) = find_triplet(&expense_report).unwrap();

    x * y * z
}
