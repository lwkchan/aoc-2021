use std::fs;

#[derive(Debug, Clone, PartialEq)]
enum IsMarked {
    Marked,
    Unmarked(String),
}

fn main() {
    let file_name = "src/input.txt";
    let file_data = fs::read_to_string(file_name).unwrap();
    let lines = file_data.lines().collect::<Vec<&str>>();

    let drawn_numbers: Vec<&str> = lines.get(0).unwrap().split(',').collect();

    let board_lines = &lines[2..].to_vec();

    let mut board_width: Option<i32> = None;
    let mut current_board: Vec<IsMarked> = Vec::new();
    let mut boards: Vec<Vec<IsMarked>> = Vec::new();
    let mut winning_board_index: Option<i32> = None;

    for index in 0..board_lines.len() {
        let current_line = board_lines[index];
        if current_line == "" {
            boards.push(current_board);
            current_board = Vec::new();
            continue;
        }

        let entries: Vec<&str> = current_line.trim().split_whitespace().collect();

        if board_width == None {
            board_width = Some(entries.len() as i32)
        }

        for e in entries {
            current_board.push(IsMarked::Unmarked(e.to_string()));
        }

        if index == board_lines.len() - 1 {
            boards.push(current_board.clone());
        }
    }

    let mut winning_number: Option<&str> = None;

    for drawn_number in drawn_numbers {
        // go through marking the boards
        for board_index in 0..boards.len() {
            let mut marked_board = boards[board_index].clone();
            for entry_index in 0..marked_board.len() {
                let entry = &marked_board[entry_index];
                match entry {
                    IsMarked::Unmarked(n) => {
                        if n == drawn_number {
                            marked_board[entry_index] = IsMarked::Marked
                        }
                    }
                    _ => {}
                };
            }
            boards[board_index] = marked_board;
            if is_winning_board(&boards[board_index], board_width.unwrap() as usize) {
                winning_board_index = Some(board_index as i32);
                break;
            }
        }

        match winning_board_index {
            Some(_) => {
                winning_number = Some(drawn_number);
                break;
            }
            None => {
                continue;
            }
        }
    }

    let mut board_sum = 0;
    for index in 0..boards[winning_board_index.unwrap() as usize].len() {
        match &boards[winning_board_index.unwrap() as usize][index] {
            IsMarked::Unmarked(num) => {
                board_sum = board_sum + num.parse::<i32>().expect("Oh no number parse failed")
            }
            IsMarked::Marked => {}
        }
    }

    println!(
        "Solution: {:?}",
        board_sum * winning_number.unwrap().parse::<i32>().expect("Oh no")
    );
}

fn is_winning_board(board: &Vec<IsMarked>, board_width: usize) -> bool {
    let mut current_row_end_index: usize = board_width;
    let mut i: usize = 0;

    while i < board.len() {
        let row = board[i..current_row_end_index].to_vec();
        if row.iter().all(|sq| &IsMarked::Marked == sq) {
            println!("win with row");
            return true;
        };
        i += board_width;
        current_row_end_index += board_width;
    }
    for col_index in 0..board_width {
        let indices: [u32; 5] = [0, 5, 10, 15, 20].map(|i| i + (col_index as u32));
        let col = indices.map(|i| board[i as usize].clone());

        if col.iter().all(|sq| &IsMarked::Marked == sq) {
            return true;
        };
    }

    false
}
