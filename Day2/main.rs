use std::env;
use std::fs;

fn str_to_int(input: &str) -> i32 {
  return input.parse::<i32>().unwrap();
}

fn check_dist (x: &i32, y: &i32, min: &i32, max: &i32, is_increasing: &bool) -> bool {
  let mut diff: i32 = *x - *y;
    
  if (diff > 0 && *is_increasing) || (diff < 0 && !*is_increasing) {
    return false;
  }

  if diff < 0 {
    diff = -diff;
  }

  if *min <= diff && diff <= *max {
    return true;
  } else {
    return false;
  }
}

fn is_safe(report: &Vec<i32>, tolerance: &i32) -> bool {
  let is_increasing: bool = &report[0] < &report[1];
  let mut number_of_fails: i32 = 0;

  for i in 1..report.iter().count() {

    if !check_dist(&report[i-1], &report[i], &1, &3, &is_increasing) {
      if number_of_fails == *tolerance {
        return false;
      }

      number_of_fails += 1;
    }
  }

  return true;
}

fn part_one(reports: &Vec<Vec<i32>>) {
  let total: Vec<bool> = reports.iter().map(|rep| is_safe(rep, &0)).filter(|res| *res).collect();

  println!("Part 1 total is: {}", total.len());
}

fn part_two(reports: &Vec<Vec<i32>>) {
  let total: Vec<bool> = reports.iter().map(|rep| is_safe(rep, &1)).filter(|res| *res).collect();

  println!("Part 1 total is: {}", total.len());
}

fn main() {
  let file_path = "input.txt";

  let input = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");

  let mut reports: Vec<Vec<i32>> = Vec::new();

  for line in input.split("\n") {
      if line.chars().count() > 0 {
          reports.push(line.split(" ").map(|x| str_to_int(x)).collect());
      }
  }
  part_one(&reports);
  part_two(&reports);
}