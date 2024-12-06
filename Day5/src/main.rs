use std::fs;
use std::collections::HashMap;

fn str_to_int(input: &str) -> i32 {
    return input.parse::<i32>().unwrap();
}

fn move_elem(arr: &mut Vec<i32>, old_index: usize, new_index: usize) {
    if old_index < new_index {
        arr[old_index..=new_index].rotate_left(1);
    } else {
        arr[new_index..=old_index].rotate_right(1);
    }
}

fn abides_by_rules(rules: &HashMap<i32, Vec<i32>>, line: &Vec<i32>, number: &i32, num_index: &usize) -> bool {
    match rules.get(number) {
        Some(rule) => for i in rule {
            let mut index: usize = usize::MAX;
            match line.iter().position(|&r| r == *i) {
                Some(i) => index = i,
                None => {}
            };

            if index <= *num_index && index != usize::MAX {
                return false;
            }
        },
        None => println!("FAIL")
    }

    return true;
}

fn fix_update(rules: &HashMap<i32, Vec<i32>>, line: &Vec<i32>) -> Vec<i32> {
    let mut new_line: Vec<i32> = Vec::new();

    for number in line {
        let mut index: usize = usize::MAX;

        match rules.get(number) {
            Some(rule) => for i in rule {
                match new_line.iter().position(|&r| r == *i) {
                    Some(i) => if index > i {
                        index = i;
                    },
                    None => {}
                };
            },
            None => {}
        }

        if index != usize::MAX {
            new_line.insert(index, *number)
        } else {
            new_line.push(*number)
        }
    }

    return new_line;
}

fn part_one(rules: &HashMap<i32, Vec<i32>>, updates: &Vec<Vec<i32>>) {
    let mut has_failed: bool = false;
    let mut total: i32 = 0;

    for line in updates {
        for index in 0..line.len() {
            if !abides_by_rules(rules, &line, &line[index], &index) {
                has_failed = true;
                break;
            }
        }

        if !has_failed {
            total += line[(line.len()-(line.len() % 2))/2]
        }
        
        has_failed = false;
    }

    println!("Part 1 total is: {}", total);
}

fn part_two(rules: &HashMap<i32, Vec<i32>>, updates: &Vec<Vec<i32>>) {
    let mut has_failed: bool = false;
    let mut total: i32 = 0;

    for line in updates {
        for index in 0..line.len() {
            if !abides_by_rules(rules, &line, &line[index], &index) {
                has_failed = true;
                break;
            }
        }

        if has_failed {
            let updated = fix_update(rules, line);
            total += updated[(updated.len()-(updated.len() % 2))/2]
        }
        
        has_failed = false;
    }

    println!("Part 2 total is: {}", total);
}

fn main() {
    let file_path = "input.txt";

    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let split_input: Vec<&str> = input.split("\n\n").collect();

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut update: Vec<Vec<i32>> = Vec::new();

    for line in split_input[0].split("\n") {
        if line.chars().count() > 0 {
            let items: Vec<i32> = line.split("|").map(|x| str_to_int(x)).collect();

            match rules.get(&items[0]) {
                Some(rule) => {
                    let mut new_vec: Vec<i32> = (*rule).to_vec(); 
                    new_vec.push(items[1]);
                    rules.insert(items[0], new_vec);
                },
                None => {
                    rules.insert(items[0], Vec::from([items[1]]));
                }
            }
        }
    }

    for line in split_input[1].split("\n") {
        if line.chars().count() > 0 {
            update.push(line.split(",").map(|x| str_to_int(x)).collect());
        }
    }

    part_one(&rules, &update);
    part_two(&rules, &update);
}