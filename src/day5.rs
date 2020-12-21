
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

pub fn find_max_seat_id(data: &Vec<String>) -> u32 {
    data.iter().fold(0, |max, bsp| {
        let row_id = bsp_to_id(bsp.get(0..7).unwrap(), &ROWS);
        let col_id = bsp_to_id(bsp.get(7..10).unwrap(), &COLUMNS);
        let seat_id = row_id * COLUMNS + col_id;

        if seat_id > max {
            seat_id
        } else {
            max
        }
    })
}
