use std::collections::{HashSet, HashMap};

const SUM_TARGET: i32 = 2020;

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

pub fn find_triplet(expense_report: &Vec<i32>) -> Option<(i32, i32, i32)> {
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

pub fn find_product(expense_report: &Vec<i32>) -> Option<i32> {
    match find_pair(&expense_report) {
        Some((a, b)) => Some(a * b),
        None => None
    }
}

pub fn find_product_triplet(expense_report: &Vec<i32>) -> Option<i32> {
    match find_triplet(&expense_report) {
        Some((x, y, z)) => Some(x * y * z),
        None => None
    }
}