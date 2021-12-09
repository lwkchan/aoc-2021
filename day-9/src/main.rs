use std::fs;

fn main() {
    let file_name = "src/input.txt";
    let file_data = fs::read_to_string(file_name).unwrap();
    let lines = file_data.lines().collect::<Vec<&str>>();

    let grid: Vec<Vec<usize>> = lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect();

    let mut low_points: Vec<usize> = vec![];

    // Your first goal is to find the low points - the locations that are lower than any of its adjacent locations.
    for (row_i, row) in grid.clone().iter().enumerate() {
        for (col_i, sq) in row.into_iter().enumerate() {
            let grid_slice = &grid;
            let surrounding: [(i32, i32); 8] = [
                (-1, -1),
                (-1, -0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];

            let surround_squares: [Option<&usize>; 8] = surrounding.map(|(x, y)| {
                let loc_row_index: i32 = row_i as i32 + x; // must ensure that everything is positive, as you cannot use negative usize
                let loc_col_index: i32 = col_i as i32 + y;
                if loc_col_index >= 0 && loc_row_index >= 0 {
                    if let Some(r) = grid_slice.get(loc_row_index as usize) {
                        if col_i as i32 + y >= 0 {
                            if let Some(e) = r.get(loc_col_index as usize) {
                                return Some(e);
                            };
                        }
                    };
                }

                None
            });

            let is_all_neighbor_squares_greater = surround_squares.iter().all(|neighbor_square| {
                if let Some(neighbor_square_num) = neighbor_square {
                    &neighbor_square_num > &&sq 
                } else {
                    true
                }
            });

            if is_all_neighbor_squares_greater {
                low_points.push(sq.clone())
            }
        }
    }

    println!("{:?}", low_points.iter().sum::<usize>() + low_points.len());
}
