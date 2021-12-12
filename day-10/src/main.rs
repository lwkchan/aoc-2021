use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct SubsystemLine {
    line: String,
}

impl SubsystemLine {
    fn new(line: &str) -> SubsystemLine {
        return SubsystemLine {
            line: line.to_string(),
        };
    }

    fn corrupted_char(&self) -> Option<char> {
        let mut pairs: HashMap<char, char> = HashMap::new();
        for (k, v) in [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')] {
            pairs.insert(k, v);
        }

        let mut current_stack: Vec<char> = vec![];
        for char in self.line.chars() {
            // if it's opening, push it to the stack
            if pairs.get(&char) != None {
                let closing = pairs.get(&char).unwrap();
                current_stack.push(*closing)
            } else {
                let expected = current_stack.pop();
                if expected.unwrap() != char {
                    return Some(char);
                }
            }
        }
        None
    }
}

fn main() {
    let file_name = "src/input.txt";
    let file_data = fs::read_to_string(file_name).unwrap();
    let lines = file_data.lines().collect::<Vec<&str>>();

    let mut total = 0;
    for l in lines {
        let subsystem_line = SubsystemLine::new(l);

        if let Some(corrupting_char) = subsystem_line.corrupted_char() {
            match corrupting_char {
                ')' => total += 3,
                ']' => total += 57,
                '}' => total += 1197,
                '>' => total += 25137,
                _ => {}
            }
        }
    }

    println!("{:?}", total);
}
