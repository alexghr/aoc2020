use ndarray::Array2;

fn parse_lines(lines: &Vec<String>) -> Array2<u32> {
    let mut arr = Array2::<u32>::zeros((lines.len(), lines[0].len()));

    for (i, line) in lines.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            arr[[i, j]] = if ch == '#' { 1 } else { 0 };
        }
    }

    arr
}

pub fn count_trees_knight_move(lines: &Vec<String>) -> u32 {
    let grid = parse_lines(&lines);
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut count = 0;

    while i < grid.shape()[0] {
        count += if grid[[i, j]] == 1 { 1 } else { 0 };
        i += 1;
        j += 3;
        if j >= grid.shape()[1] {
            j = j % grid.shape()[1];
        }
    }

    count
}
