use std::fs;

fn main() {
    solve_1();
    solve_2();
}

fn solve_1() {
    let file_name = "src/input.txt";
    let file_data = fs::read_to_string(file_name).unwrap();
    let lines = file_data.lines().collect::<Vec<&str>>();

    let mut current_depth = 0;
    let mut current_horizontal_position = 0;
    for i in 0..lines.len() {
        let current_line_parts: Vec<&str> = lines.get(i).unwrap().split_whitespace().collect();
        let direction = current_line_parts.get(0).unwrap();
        let number: u32 = current_line_parts
            .get(1)
            .unwrap()
            .parse()
            .expect("Failed to parse number");

        match (direction, number) {
            (&"forward", _) => {
                current_horizontal_position += number;
            }
            (&"down", _) => {
                current_depth += number;
            }
            (&"up", _) => {
                current_depth -= number;
            }
            (_, _) => {}
        }
    }

    println!("{:?}", current_horizontal_position * current_depth)
}

// down X increases your aim by X units.
// up X decreases your aim by X units.
// forward X does two things:
//     It increases your horizontal position by X units.
//     It increases your depth by your aim multiplied by X.

fn solve_2() {
    let file_name = "src/input.txt";
    let file_data = fs::read_to_string(file_name).unwrap();
    let lines = file_data.lines().collect::<Vec<&str>>();

    let mut current_depth = 0;
    let mut current_horizontal_position = 0;
    let mut aim = 0;

    for i in 0..lines.len() {
        let current_line_parts: Vec<&str> = lines.get(i).unwrap().split_whitespace().collect();
        let direction = current_line_parts.get(0).unwrap();
        let number: u32 = current_line_parts
            .get(1)
            .unwrap()
            .parse()
            .expect("Failed to parse number");

        match (direction, number) {
            (&"forward", _) => {
                current_horizontal_position += number;
                current_depth += aim * number
            }
            (&"down", _) => {
                aim += number;
            }
            (&"up", _) => {
                aim -= number;
            }
            (_, _) => {}
        }
    }

    println!("{:?}", current_horizontal_position * current_depth)
}
