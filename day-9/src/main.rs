use std::fs;

struct Cave {
    grid: Vec<Vec<i32>>,
    low_points: Vec<(i32, i32)>,
}

impl Cave {
    fn new(grid: Vec<Vec<i32>>) -> Cave {
        let mut low_points: Vec<(i32, i32)> = vec![];

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
                let surround_squares: [Option<&i32>; 8] = surrounding.map(|(x, y)| {
                    let loc_row_index: i32 = row_i as i32 + x; // must ensure that everything is positive, as you cannot use negative i32
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
                    low_points.push((row_i as i32, col_i as i32))
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

    let grid: Vec<Vec<i32>> = lines
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
        .map(|(row_i, col_i)| cave.grid[*row_i as usize][*col_i as usize])
        .sum::<i32>();
    println!("Answer 1: {:?}", low_points_sum + cave.low_points.len() as i32);
}

fn solve_2(cave: &Cave) {
    let mut pool_sizes: Vec<usize> = cave
        .low_points
        .iter()
        .map(|low_point| find_pool_size(low_point, &cave.grid))
        .collect::<Vec<usize>>();

    pool_sizes.sort();
    pool_sizes.reverse();

    println!("{:?}", pool_sizes); // 110, 99, 97, 96, 94, 92, 91, but largest pool size currently off by one :(
    let largest_three_pool_sizes: Vec<usize> = pool_sizes[0..3].to_vec();
    println!("{:?}", largest_three_pool_sizes.iter().fold(1, |a, b| a * b));
}

fn find_pool_size(low_point: &(i32, i32), cave_grid: &Vec<Vec<i32>>) -> usize {
    let squares_in_pool =
        get_surrounding_consecutive_squares(low_point, &cave_grid);

        squares_in_pool.len()
}

fn get_surrounding_consecutive_squares(
    (low_point_row, low_point_col): &(i32, i32),
    cave_grid: &Vec<Vec<i32>>,
) -> Vec<(i32, i32)> {
    let mut pool_points: Vec<(i32, i32)> = vec![(*low_point_row, *low_point_col)];
    let mut next_to_check: Vec<(i32, i32)> = vec![(*low_point_row, *low_point_col)];

    while next_to_check.len() > 0 {
        let squares_to_check: Vec<(i32, i32)> = next_to_check.clone();
        for current_square in squares_to_check {
            let (current_square_x, current_square_y) = current_square;
            let low_point_num: i32 = cave_grid[current_square_x as usize][current_square_y as usize];
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

            for (x, y) in surrounding {
                let loc_row_index: i32 = current_square_x as i32 + x; // must ensure that everything is positive, as you cannot use negative i32
                let loc_col_index: i32 = current_square_y as i32 + y;
                if loc_col_index >= 0 && loc_row_index >= 0 {
                    if let Some(row) = cave_grid.get(loc_row_index as usize) {
                        if let Some(col) = row.get(loc_col_index as usize) {
                            if *col != 9 as i32 && (col - low_point_num == 1 || col - low_point_num == i32::max_value()) {
                                if !pool_points.iter().any(|(existing_x, existing_y)| {
                                    loc_col_index as i32 == *existing_y
                                        && loc_row_index as i32 == *existing_x
                                }) 
                                {
                                    pool_points
                                        .push((loc_row_index , loc_col_index));
                                    next_to_check
                                        .push((loc_row_index , loc_col_index));
                                }
                            }
                        }
                    };
                }
            }


            next_to_check.retain(|(unchecked_x, unchecked_y)| {
                !(unchecked_x == &current_square_x && unchecked_y == &current_square_y)
            });
        }
    }

    pool_points
}


