use std::fs;

fn main() {
    solve_1();
    solve_2();
}

// How many measurements are larger than the previous measurement?
fn solve_1() {
    let file_name = "src/input.txt";
    let file_data = fs::read_to_string(file_name).unwrap();
    let lines = file_data.lines();

    let mut number_of_increases: u32 = 0;
    let mut last_number: Option<u32> = None;

    for line in lines {
        let current_number = line.parse::<u32>().unwrap();

        match last_number {
            Some(last_number) => {
                if current_number > last_number {
                    number_of_increases += 1;
                }
            }
            _ => {}
        }

        last_number = Some(current_number);
    }

    println!("{}", number_of_increases)
}

fn solve_2() {
    let file_name = "src/input.txt";
    let file_data = fs::read_to_string(file_name).unwrap();
    let lines: Vec<u32> = file_data
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect();

    let mut number_of_increases: u32 = 0;
    let mut last_total: u32 = 0;

    for index in 0..lines.len() {
        let indices: [u32; 3] = [0, 1, 2];
        let group: [Option<&u32>; 3] = indices.map(|i| lines.get(index + i as usize));

        let mut current_total: u32 = 0;
        match group {
            [Some(a), Some(b), Some(c)] => {
                current_total = a + b + c;
                if last_total != 0 && current_total > last_total {
                    number_of_increases += 1;
                }
            }
            _ => {}
        }
        last_total = current_total;
    }

    println!("{}", number_of_increases)
}
