use std::collections::HashSet;

const SUM_TARGET: i32 = 2020;

fn find_entries(expense_report: &Vec<i32>) -> Option<(i32, i32)> {
    let hash: HashSet<i32> = expense_report.iter().cloned().collect();
    expense_report.iter().find_map(|x| {
        if hash.contains(&(SUM_TARGET - x)) {
            Some((x.clone(), SUM_TARGET - x))
        } else {
            None
        }
    })
}

pub fn find_product(expense_report: &Vec<i32>) -> Option<i32> {
    match find_entries(&expense_report) {
        Some((a, b)) => Some(a * b),
        None => None
    }
}