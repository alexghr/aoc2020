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
    count_trees(&grid, &(1, 3))
}

pub fn check_slopes(lines: &Vec<String>) -> u64 {
    let grid = parse_lines(&lines);
    let slopes = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    slopes.iter().fold(1 as u64, |prod, &slope| prod * (count_trees(&grid, &slope) as u64))
}

fn count_trees(grid: &Array2<u32>, slope: &(usize, usize)) -> u32 {
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut count = 0;

    let (dx, dy) = slope;
    let grid_shape = grid.shape();

    while i < grid_shape[0] {
        count += if grid[[i, j]] == 1 { 1 } else { 0 };
        i += dx;
        j = (j + dy) % grid_shape[1];
    }

    count
}
