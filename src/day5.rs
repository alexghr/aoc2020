use std::collections::HashSet;

const ROWS: u32 = 128;
const COLUMNS: u32 = 8;

fn bsp_to_id(bsp: &str, size: &u32) -> u32 {

    let mut min = 0;
    let mut max = size - 1;

    for ch in bsp.chars() {
        if ch == 'F' || ch == 'L' {
            max = max - (max - min) / 2 - 1;
        } else {
            min = min + (max - min) / 2 + 1;
        }
    }
    
    let ch = bsp.chars().last();
    if ch == Some('F') || ch == Some('L') {
        min
    } else {
        max
    }
}

fn map_bsp(data: &Vec<String>) -> Vec<u32> {
    data
        .iter()
        .map(|bsp| {
            let row_id = bsp_to_id(bsp.get(0..7).unwrap(), &ROWS);
            let col_id = bsp_to_id(bsp.get(7..10).unwrap(), &COLUMNS);
            row_id * COLUMNS + col_id
        })
        .collect()
}

pub fn find_max_seat_id(data: &Vec<String>) -> u32 {
    map_bsp(&data).iter().fold(0, |max, &seat_id| {
        if seat_id > max {
            seat_id
        } else {
            max
        }
    })
}

pub fn find_free_seat(data: &Vec<String>) -> u32 {
    let taken_seats: HashSet<u32> = map_bsp(&data).iter().cloned().collect();

    // technically seat 0 exists but the problem statement says the first few seats will be missing
    // so... start searching from seat 1 in order to avoid a `subtract with overflow` panic
    for seat_id in 1..(ROWS * COLUMNS) {
        if !taken_seats.contains(&seat_id) {
            if taken_seats.contains(&(seat_id - 1)) && taken_seats.contains(&(seat_id + 1)) {
                return seat_id
            }
        }
    }

    panic!("Where's my seat?");
}
