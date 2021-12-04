use std::fs;

fn main() {
    solve_1();
    solve_2();
}

fn solve_1() {
    let file_name = "src/input.txt";
    let file_data = fs::read_to_string(file_name).unwrap();
    let lines = file_data.lines().collect::<Vec<&str>>();

    let length = lines.get(0).unwrap().len();

    // gamma rate is most common bit
    let mut gamma = String::new();
    // epsilon bit is least common bit
    let mut epsilon = String::new();

    for position_index in 0..length {
        let mut zero_counter = 0;
        let mut one_counter = 0;
        for line_index in 0..lines.len() {
            let line = lines[line_index];
            let char = line.chars().nth(position_index).unwrap();

            match char {
                '1' => {
                    one_counter += 1;
                }
                '0' => {
                    zero_counter += 1;
                }
                _ => {}
            }
        }
        if zero_counter > one_counter {
            gamma += "0";
            epsilon += "1";
        } else {
            gamma += "1";
            epsilon += "0";
        }
    }

    let gamma_num = i64::from_str_radix(&gamma, 2).expect("Oh no!");
    let epsilon_num = i64::from_str_radix(&epsilon, 2).expect("Oh no!");

    println!("{:?}", gamma_num * epsilon_num);
}

fn solve_2() {
    let file_name = "src/input.txt";
    let file_data = fs::read_to_string(file_name).unwrap();
    let lines = file_data.lines().collect::<Vec<&str>>();
    let length = lines.get(0).unwrap().len();

    let mut oxygen_only_pick_starting_with = String::new();
    let mut oxygen_final_bin = String::new();

    let mut co2_only_pick_starting_with = String::new();
    let mut co2_final_bin = String::new();

    for position_index in 0..length {
        if oxygen_final_bin.len() > 0 && co2_final_bin.len() > 0 {
            break;
        }

        if oxygen_final_bin.len() == 0 {
            // could reduce duplication by making this block a function for each co2 and oxygen
            let lines_to_consider_oxygen: Vec<&str> = lines
                .clone()
                .into_iter()
                .filter(|&l| l.starts_with(&oxygen_only_pick_starting_with))
                .collect();
            if lines_to_consider_oxygen.len() == 1 {
                oxygen_final_bin = lines_to_consider_oxygen.get(0).unwrap().to_string();
                continue;
            };
            let (oxygen_zero_counter, oxygen_one_counter) = count_in_position(
                &lines_to_consider_oxygen,
                position_index,
                &oxygen_only_pick_starting_with,
            );

            if oxygen_zero_counter < oxygen_one_counter || oxygen_zero_counter == oxygen_one_counter
            {
                oxygen_only_pick_starting_with += "1";
            } else if oxygen_zero_counter > oxygen_one_counter {
                oxygen_only_pick_starting_with += "0";
            }

            if position_index == length - 1 {
                oxygen_final_bin = oxygen_only_pick_starting_with.clone();
            }
        }

        // co2
        // before checking through again, make sure there are other matching lines in the list
        if co2_final_bin.len() == 0 {
            let lines_to_consider_co2: Vec<&str> = lines
                .clone()
                .into_iter()
                .filter(|&l| l.starts_with(&co2_only_pick_starting_with))
                .collect();
            if lines_to_consider_co2.len() == 1 {
                co2_only_pick_starting_with = lines_to_consider_co2.get(0).unwrap().to_string();
                continue;
            };
            let (co2_zero_counter, co2_one_counter) = count_in_position(
                &lines_to_consider_co2,
                position_index,
                &co2_only_pick_starting_with,
            );
            if co2_zero_counter < co2_one_counter || co2_zero_counter == co2_one_counter {
                co2_only_pick_starting_with += "0";
            } else if co2_zero_counter > co2_one_counter {
                co2_only_pick_starting_with += "1";
            }
        }

        if position_index == length - 1 {
            co2_final_bin = co2_only_pick_starting_with.clone();
        }
    }

    println!("ox {:?}", oxygen_only_pick_starting_with);
    println!("co2 {:?}", co2_only_pick_starting_with);

    let oxygen_number_dec =
        i64::from_str_radix(&oxygen_only_pick_starting_with, 2).expect("Oh no!");
    let co2_number_dec = i64::from_str_radix(&co2_only_pick_starting_with, 2).expect("Oh no!");

    println!("{:?}", co2_number_dec * oxygen_number_dec);
}

fn count_in_position(lines: &Vec<&str>, position_index: usize, filter: &String) -> (i32, i32) {
    let mut zero_counter = 0;
    let mut one_counter = 0;

    for line_index in 0..lines.len() {
        let line = lines[line_index];
        if !line.starts_with(filter) {
            continue;
        }
        let char = line.chars().nth(position_index).unwrap(); // current char in position

        match char {
            '1' => {
                one_counter += 1;
            }
            '0' => {
                zero_counter += 1;
            }
            _ => {}
        }
    }

    (zero_counter, one_counter)
}
