use std::fs;
use std::collections::HashMap;

fn str_to_int(input: &str) -> u64 {
    return input.parse::<u64>().unwrap_or_else(|_| panic!("{}", input));
}

fn replace_first(mut equation: Vec<u64>, num: u64) -> Vec<u64> {
    equation[0] = num;

    return equation;
}

fn append_first(equation: Vec<u64>, first: u64) -> Vec<u64> {
    let new_first: u64 = str_to_int(&(first.to_string() + &equation[0].to_string()));

    return replace_first(equation, new_first);
}

fn is_valid(equation: Vec<u64>, result: u64, is_second: &bool) -> bool {
    if equation.len() == 1 {
        return result == equation[0];
    } 
    if equation[0] > result {
        return false;
    }

    let length = equation.len();
    let reduced_vec = (&equation[1..length]).to_vec();

    if is_valid(replace_first(reduced_vec.clone(), equation[0] + equation[1]), result, is_second) {
        return true;
    }
    if is_valid(replace_first(reduced_vec.clone(), equation[0] * equation[1]), result, is_second) {
        return true;
    }    
    if *is_second && is_valid(append_first(reduced_vec, equation[0]), result, is_second) {
        return true;
    }

    return false;
}

fn part_one(equations: &HashMap<u64, Vec<u64>>) {
    let mut total: u64 = 0;

    for (result, eq) in equations {
        if is_valid(eq.clone(), result.clone(), &false) {
            total += result;
        }
    }
    
    println!("Part 1 total is: {}", total);
}

fn part_two(equations: &HashMap<u64, Vec<u64>>) {
    let mut total: u64 = 0;

    for (result, eq) in equations {
        if is_valid(eq.clone(), result.clone(), &true) {
            total += result;
        }
    }
    
    println!("Part 2 total is: {}", total);
}

fn main() {
    let file_path = "input.txt";

    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let equations: &mut HashMap<u64, Vec<u64>> = &mut HashMap::new();

    for line in input.split("\n") {
        if line.chars().count() > 0 {
            let split_line: Vec<String>  = line.split(": ").map(|x| x.to_string()).collect();

            equations.insert(str_to_int(&split_line[0]), split_line[1].split(" ").map(|x| str_to_int(x)).collect());
        }
    }

    part_one(&equations);
    part_two(&equations);
}
