use std::fs;

struct Cave {
    grid: Vec<Vec<usize>>,
    low_points: Vec<(usize, usize)>,
}

impl Cave {
    fn new(grid: Vec<Vec<usize>>) -> Cave {
        let mut low_points: Vec<(usize, usize)> = vec![];

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

                let is_all_neighbor_squares_greater =
                    surround_squares.iter().all(|neighbor_square| {
                        if let Some(neighbor_square_num) = neighbor_square {
                            &neighbor_square_num > &&sq
                        } else {
                            true
                        }
                    });
                if is_all_neighbor_squares_greater {
                    low_points.push((row_i, col_i))
                }
            }
        }

        Cave { grid, low_points }
    }
}

fn main() {
    let file_name = "src/input.txt";
    let file_data = fs::read_to_string(file_name).unwrap();
    let lines = file_data.lines().collect::<Vec<&str>>();

    let grid: Vec<Vec<usize>> = lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect();

    let cave = Cave::new(grid);

    solve_1(&cave);
    solve_2(&cave);
}

fn solve_1(cave: &Cave) {
    let low_points_sum = cave
        .low_points
        .iter()
        .map(|(row_i, col_i)| cave.grid[*row_i][*col_i])
        .sum::<usize>();
    println!("Answer 1: {:?}", low_points_sum + cave.low_points.len());
}

fn solve_2(cave: &Cave) {
    let mut pool_sizes: Vec<usize> = cave
        .low_points
        .iter()
        .map(|low_point| find_pool_size(low_point, &cave.grid))
        .collect::<Vec<usize>>();

    pool_sizes.sort();

    let largest_three_pool_sizes: Vec<usize> = pool_sizes[0..3].to_vec();
    println!("{:?}", largest_three_pool_sizes);
}

fn find_pool_size(low_point: &(usize, usize), cave_grid: &Vec<Vec<usize>>) -> usize {
    let (low_point_row, low_point_col) = low_point;
    // todo!();
    let mut is_done = false;
    let mut pool_points: Vec<(usize, usize)> = vec![];
    let mut current_row_i: usize = *low_point_row;
    let mut current_column_i: usize = *low_point_col;
    while !is_done {
        let lower_starting_point = cave_grid[current_row_i][current_column_i];
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
        let surround_squares: [Option<(usize, usize)>; 8] = surrounding.map(|(x, y)| {
            let loc_row_index: i32 = current_row_i as i32 + x; // must ensure that everything is positive, as you cannot use negative usize
            let loc_col_index: i32 = current_column_i as i32 + y;
            if loc_col_index >= 0 && loc_row_index >= 0 {
                if let Some(r) = cave_grid.get(loc_row_index as usize) {
                    if current_column_i as i32 + y >= 0 {
                        if let Some(e) = r.get(loc_col_index as usize) {
                            if *e != 9 as usize && e - lower_starting_point == 1 {
                                return Some((loc_row_index as usize, loc_col_index as usize));
                            } else {
                                return None;
                            }
                        }
                    }
                };
            }
            None
        });

        for sq in surround_squares {
            if let Some(s) = sq {
                let (s_x, s_y) = s;
                if let None = pool_points.iter().position(|(x, y)| *x == s_x && *y == s_y) {
                    pool_points.push(s)
                }
            }
        }

        is_done = true;
    }

    4
}
