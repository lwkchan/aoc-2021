use std::fs;

#[derive(Debug)]
struct VentLine {
    x: (u32, u32),
    y: (u32, u32),
}

impl VentLine {
    fn new(line: &str) -> VentLine {
        let line_parts = line.split_whitespace().collect::<Vec<&str>>();
        let mut x: Vec<u32> = Vec::new();
        let mut y: Vec<u32> = Vec::new();
        for i in [
            line_parts.get(0).unwrap(), // first part
            line_parts.get(2).unwrap(), // second part
        ] {
            let nums = i
                .split(',')
                .map(|n| n.parse().expect("Oh no"))
                .collect::<Vec<u32>>();
            x.push(nums[0]);
            y.push(nums[1]);
        }

        // the vector direction shouldn't matter, so sort it from small to big here for easier compoarison later
        x.sort();
        y.sort();
        VentLine {
            x: (x.get(0).unwrap().clone(), x.get(1).unwrap().clone()),
            y: (y.get(0).unwrap().clone(), y.get(1).unwrap().clone()),
        }
    }
}

fn main() {
    solve_1();
    solve_2();
}

fn solve_1() {
    let file_name = "src/input.txt";
    let file_data = fs::read_to_string(file_name).unwrap();
    let lines = file_data.lines().collect::<Vec<&str>>();
    let vent_lines = generate_vent_lines(lines, Some(is_equal_x_or_y));
    let grid = generate_grid(&vent_lines);
    let mut processed_grid = grid.clone();
    for v in vent_lines {
        let (x1, x2) = v.x; // col
        let (y1, y2) = v.y; // row

        for (row_index, row) in grid.iter().enumerate() {
            if !(row_index >= y1 as usize && row_index <= y2 as usize) {
                continue;
            }
            for (col_index, _cell) in row.iter().enumerate() {
                // if numbers are between, then += 1;
                if col_index >= x1 as usize && col_index <= x2 as usize {
                    processed_grid[row_index][col_index] += 1;
                }
            }
        }
    }

    println!("{:?}", count_at_least_two(&processed_grid))
}

fn solve_2() {
    let file_name = "src/input.txt";
    let file_data = fs::read_to_string(file_name).unwrap();
    let lines = file_data.lines().collect::<Vec<&str>>();
    let vent_lines = generate_vent_lines(lines, None);
    let grid = generate_grid(&vent_lines);
    let mut processed_grid = grid.clone();

    for v in vent_lines {
        let (x1, x2) = v.x; // col
        let (y1, y2) = v.y; // row

        for (row_index, row) in grid.iter().enumerate() {
            if !(row_index >= y1 as usize && row_index <= y2 as usize) {
                continue;
            }
            for (col_index, _cell) in row.iter().enumerate() {
                // if numbers are between, then += 1;
                if col_index >= x1 as usize && col_index <= x2 as usize {
                    processed_grid[row_index][col_index] += 1;
                }
            }
        }
    }
}

fn is_equal_x_or_y(vent_line: &VentLine) -> bool {
    let (x1, x2) = vent_line.x;
    let (y1, y2) = vent_line.y;

    x1 == x2 || y1 == y2
}

fn generate_vent_lines(
    lines: Vec<&str>,
    filter: Option<fn(vent_line: &VentLine) -> bool>,
) -> Vec<VentLine> {
    let mut vent_lines: Vec<VentLine> = Vec::new();
    for line in lines {
        let vent_line = VentLine::new(line);
        match filter {
            Some(f) => {
                if f(&vent_line) {
                    vent_lines.push(vent_line);
                }
            }

            None => {
                vent_lines.push(vent_line);
            }
        }
    }

    vent_lines
}

fn generate_grid(vent_lines: &Vec<VentLine>) -> Vec<Vec<u32>> {
    let mut largest_x: u32 = 0;
    let mut largest_y: u32 = 0;

    for line in vent_lines {
        let (x1, x2) = line.x;
        let (y1, y2) = line.y;

        if x1 > largest_x {
            largest_x = x1;
        }
        if x2 > largest_x {
            largest_x = x2;
        }
        if y1 > largest_y {
            largest_y = y1;
        }
        if y2 > largest_y {
            largest_y = y2;
        }
    }

    vec![vec![0; (largest_x + 1) as usize]; (largest_y + 1) as usize]
}

fn count_at_least_two(grid: &Vec<Vec<u32>>) -> u32 {
    let mut count_at_least_two = 0;

    for row in grid {
        for square in row {
            if square >= &2 {
                count_at_least_two += 1;
            }
        }
    }

    count_at_least_two
}
