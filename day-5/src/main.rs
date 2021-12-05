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
    let file_name = "src/input.txt";
    let file_data = fs::read_to_string(file_name).unwrap();
    let lines = file_data.lines().collect::<Vec<&str>>();

    let mut vent_lines: Vec<VentLine> = Vec::new();
    let mut largest_x: u32 = 0;
    let mut largest_y: u32 = 0;

    for line in lines {
        let vent_line = VentLine::new(line);

        let (x1, x2) = vent_line.x;
        let (y1, y2) = vent_line.y;

        // For now, only consider horizontal and vertical lines: lines where either x1 = x2 or y1 = y2.
        if x1 == x2 || y1 == y2 {
            // get largest x2 and largest y2 to lay out the grid for later
            vent_lines.push(vent_line);
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
    }

    // make a largest poss grid grid;
    let grid: Vec<Vec<u32>> = vec![vec![0; (largest_x + 1) as usize]; (largest_y + 1) as usize];
    // println!("{:?}", grid);
    // process lines
    let mut processed_grid = grid.clone();
    for v in vent_lines {
        let (x1, x2) = v.x; // col
        let (y1, y2) = v.y; // row

        for (row_index, row) in grid.iter().enumerate() {
            if !(row_index >= y1 as usize && row_index <= y2 as usize) {
                continue;
            }
            for (col_index, cell) in row.iter().enumerate() {
                // if numbers are between, then += 1;
                if col_index >= x1 as usize && col_index <= x2 as usize {
                    processed_grid[row_index][col_index] += 1;
                }
            }
        }
    }

    let mut count_at_least_two = 0;

    for row in processed_grid {
        for square in row {
            if square >= 2 {
                count_at_least_two += 1;
            }
        }
    }

    println!("{:?}", count_at_least_two)
}
