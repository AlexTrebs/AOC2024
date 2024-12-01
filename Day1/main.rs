use std::env;
use std::fs;

fn str_to_int(input: &str) -> i32 {
    return input.parse::<i32>().unwrap();
}

fn part_one(left: &Vec<i32>, right: &Vec<i32>) {
    let mut total: i32 = 0;

    for i in 0..left.iter().count() {
        let diff = left[i] - right[i];

        if diff > 0 {
            total += diff;
        } else {
            total -= diff;
        }
    }   

    println!("Part 1 total is: {}", total);
}

fn part_two(left: &Vec<i32>, right: &Vec<i32>) {
    let mut total: i32 = 0;

    for item in left {
        let weight: i32 = right.iter().filter(|&n| *n == *item).count() as i32;
        
        total += weight * *item;
    }   
    println!("Part 2 total is: {}", total);
}

fn main() {
    let file_path = "input.txt";

    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in input.split("\n") {
        if line.chars().count() > 0 {
            let items: Vec<&str> = line.split("   ").collect();

            left_list.push(str_to_int(items[0]));
            right_list.push(str_to_int(items[1]));
        }
    }

    left_list.sort();
    right_list.sort();

    part_one(&left_list, &right_list);
    part_two(&left_list, &right_list);
}