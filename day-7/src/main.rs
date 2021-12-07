use std::cmp::Ordering;
use std::fs;

fn main() {
    let file_name = "src/input.txt";
    let file_data = fs::read_to_string(file_name).unwrap();

    let nums: Vec<f32> = file_data
        .trim()
        .split(',')
        .map(|n| n.parse().expect("Oh no"))
        .collect();

    solve_1(&nums);
    solve_2(&nums);
}

fn solve_1(nums: &Vec<f32>) {
    let median = median(&nums).unwrap().round();
    let mut total_fuel: f32 = 0.0;
    for n in nums {
        total_fuel += (n - median).abs();
    }

    println!("{}", total_fuel)
}

fn solve_2(nums: &Vec<f32>) {
    let mut mean: f32 = { nums.iter().sum::<f32>() / nums.len() as f32 }.round();

    // check both sides of mean
    let mut result: Option<i32> = None;

    while result == None {
        let test_targets = [mean - 1 as f32, mean, mean + 1 as f32];
        let possible_total_fuel = test_targets.map(|t| calc_total_fuel_for_target(nums, t));
        let least_fuel_used = possible_total_fuel.iter().min().unwrap();
        let target_with_lowest_fuel_index = possible_total_fuel
            .iter()
            .position(|x| &x == &least_fuel_used)
            .unwrap();

        if mean == test_targets[target_with_lowest_fuel_index] {
            result = Some(*least_fuel_used);
        }
        mean = test_targets[target_with_lowest_fuel_index];
    }

    println!("{:?}", result.unwrap())
}

fn calc_total_fuel_for_target(nums: &Vec<f32>, target: f32) -> i32 {
    let mut total_fuel: i32 = 0;
    for num in nums {
        let distance = (num - target).abs();
        let fuel = (distance * (distance + 1 as f32)) / 2 as f32;
        total_fuel += fuel as i32;
    }
    total_fuel
}

// From Rust cookbook https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/statistics.html
fn median(data: &Vec<f32>) -> Option<f32> {
    let size = data.len();

    match size {
        even if even % 2 == 0 => {
            let fst_med = select(data, (even / 2) - 1);
            let snd_med = select(data, even / 2);

            match (fst_med, snd_med) {
                (Some(fst), Some(snd)) => Some((fst + snd) as f32 / 2.0),
                _ => None,
            }
        }
        odd => select(data, odd / 2).map(|x| x as f32),
    }
}

fn select(data: &[f32], k: usize) -> Option<f32> {
    let part = partition(data);

    match part {
        None => None,
        Some((left, pivot, right)) => {
            let pivot_idx = left.len();

            match pivot_idx.cmp(&k) {
                Ordering::Equal => Some(pivot),
                Ordering::Greater => select(&left, k),
                Ordering::Less => select(&right, k - (pivot_idx + 1)),
            }
        }
    }
}

fn partition(data: &[f32]) -> Option<(Vec<f32>, f32, Vec<f32>)> {
    match data.len() {
        0 => None,
        _ => {
            let (pivot_slice, tail) = data.split_at(1);
            let pivot = pivot_slice[0];
            let (left, right) = tail.iter().fold((vec![], vec![]), |mut splits, next| {
                {
                    let (ref mut left, ref mut right) = &mut splits;
                    if next < &pivot {
                        left.push(*next);
                    } else {
                        right.push(*next);
                    }
                }
                splits
            });

            Some((left, pivot, right))
        }
    }
}
