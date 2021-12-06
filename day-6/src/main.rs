use std::fs;

#[derive(Debug)]
struct Buckets {
    buckets: [usize; 9],
}

// used https://github.com/yoshuawuyts/aoc/blob/main/aoc-2021/src/day6.rs as I couldn't figure out how to avoid the growing vector
//
impl Buckets {
    fn new() -> Self {
        Self { buckets: [0; 9] }
    }

    fn insert_fish(&mut self, index: usize) {
        self.buckets[index] += 1;
    }

    fn step_day(&mut self) {
        self.buckets.rotate_left(1);
        self.buckets[6] += self.buckets[8];
    }

    fn count_fish(&self) -> usize {
        self.buckets.iter().sum()
    }
}

fn main() {
    let file_name = "src/input.txt";
    let file_data = fs::read_to_string(file_name).unwrap();

    let mut bucket_1 = Buckets::new();
    for nums in file_data.split(',') {
        bucket_1.insert_fish(nums.trim().parse().unwrap());
    }

    let mut current_day = 1;

    while current_day <= 80 {
        bucket_1.step_day();
        current_day += 1;
    }

    println!("{}", bucket_1.count_fish());

    // second part

    let mut bucket_2 = Buckets::new();
    for nums in file_data.split(',') {
        bucket_2.insert_fish(nums.trim().parse().unwrap());
    }

    let mut current_day = 1;

    while current_day <= 256 {
        bucket_2.step_day();
        current_day += 1;
    }

    println!("{}", bucket_2.count_fish());
}
