use itertools::Itertools;
use std::fs;
use std::str;

struct Digit {
    string: String,
}

impl Digit {
    pub fn new(string: String) -> Digit {
        Digit { string }
    }
    fn as_number(self) -> Option<usize> {
        let length = self.string.len();
        match length {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        }
    }
}

struct DigitDisplay {
    map: Vec<String>,
}

impl DigitDisplay {
    pub fn new(string: String) -> DigitDisplay {
        fn generate_map(string: &String) -> Vec<String> {
            let mut map = vec![String::new(); 10];
            let num_strings: Vec<&str> = string.split_whitespace().collect::<Vec<&str>>();

            // determine 1, 4, 7, 8
            for num_string in &num_strings {
                let digit = Digit::new(num_string.to_string());
                if let Some(number) = digit.as_number() {
                    map[number] = num_string.to_string();
                }
            }

            map[9] = {
                let four_chars: Vec<char> = map[4].chars().collect();
                find_in_num_strings(&num_strings, Some(&four_chars), 6, vec![])
            };

            map[0] = {
                let one_chars: Vec<char> = map[1].chars().collect();
                find_in_num_strings(&num_strings, Some(&one_chars), 6, vec![&map[9]])
            };

            map[6] = find_in_num_strings(&num_strings, None, 6, vec![&map[0], &map[9]]);

            map[3] = {
                let seven_chars: Vec<char> = map[7].chars().collect();
                find_in_num_strings(&num_strings, Some(&seven_chars), 5, vec![])
            };

            map[5] = {
                let nine_chars: Vec<char> = map[9].chars().collect();
                num_strings
                    .clone() // have to do this before 2, otherwise num_strings is dropped
                    .into_iter()
                    .find(|ns| {
                        ns.len() == 5
                            && ns
                                .chars()
                                .all(|char| nine_chars.contains(&char) && ns != &map[3])
                    })
                    .unwrap()
                    .to_string()
            };

            map[2] = find_in_num_strings(&num_strings, None, 5, vec![&map[5], &map[3]]);

            map
        }

        DigitDisplay {
            map: generate_map(&string),
        }
    }

    fn get_as_number(&mut self, string: &str) -> Option<usize> {
        self.map.iter().position(|s| {
            string.chars().sorted().rev().collect::<String>()
                == s.chars().sorted().rev().collect::<String>()
        })
    }
}

fn main() {
    let file_name = "src/input.txt";
    let file_data = fs::read_to_string(file_name).unwrap();
    let lines = file_data.lines().collect::<Vec<&str>>();

    solve_1(&lines);
    solve_2(&lines);
}

fn solve_1(lines: &Vec<&str>) {
    let all_output_digits: Vec<&str> = lines
        .into_iter()
        .map(|l| l.split('|').collect::<Vec<&str>>()[1].trim()) // only consider output side after |
        .map(|parts| parts.split_whitespace())
        .flatten()
        .collect();

    let mut counter = 0;
    for output_digit in all_output_digits {
        let digit = Digit::new(output_digit.to_string());
        if let Some(_) = digit.as_number() {
            counter += 1;
        }
    }

    println!("Counter: {:?}", counter);
}

fn solve_2(lines: &Vec<&str>) {
    let inputs_and_outputs: Vec<(&str, &str)> = lines
        .into_iter()
        .map(|l| {
            let parts = l.split('|').collect::<Vec<&str>>();
            (parts[0].trim(), parts[1].trim())
        })
        .collect();

    let mut total: usize = 0;
    for (input, output) in inputs_and_outputs {
        let mut digit_display = DigitDisplay::new(input.to_string());

        let output_num_string = output
            .split_whitespace()
            .map(|s| digit_display.get_as_number(s).unwrap().to_string())
            .collect::<Vec<String>>()
            .join("");
        let number: usize = output_num_string.parse().unwrap();

        total += number;
    }

    println!("Total is {}", total)
}

fn find_in_num_strings(
    num_strings: &Vec<&str>,
    all_chars: Option<&Vec<char>>,
    length: usize,
    not_these_strings: Vec<&str>,
) -> String {
    num_strings
        .into_iter()
        .find(|ns| {
            not_these_strings.iter().all(|s| &s != ns)
                && ns.len() == length
                && if let Some(chars) = all_chars {
                    chars.iter().all(|&char| ns.contains(char))
                } else {
                    true
                }
        })
        .unwrap()
        .to_string()
}
