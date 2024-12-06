use std::fs;
use regex::Regex;

fn str_to_int(input: &str) -> i32 {
  return input.parse::<i32>().unwrap();
}

fn find_mult(input: &str) -> i32{
  let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

  let mut total: i32 = 0;

  for (_, [x, y]) in re.captures_iter(&input).map(|c| c.extract()) {
    total += str_to_int(x) * str_to_int(y);
  }

  return total;
}

fn part_one(memory: &str) {
  let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

  let mut total: i32 = 0;
  total += find_mult(memory);

  println!("Part 1 total is: {}", total);
}

fn part_two(memory: &str) {
  let do_re = Regex::new(r"do\(\)").unwrap();
  let dont_re = Regex::new(r"don't\(\)").unwrap();

  let mut total: i32 = 0;
  let mut sub_str: &str = memory;

  while sub_str.len() > 0 {
    match dont_re.find(sub_str) {
      Some(index) => {
        total += find_mult(&sub_str[..index.start()]);
        sub_str = &sub_str[index.start()..]
      },
      None => {
        total += find_mult(&sub_str);
        sub_str = ""
      }
    };
    match do_re.find(sub_str) {
      Some(index) => sub_str = &sub_str[index.start()..],
      None => sub_str = ""
    };
  }

  println!("Part 2 total is: {}", total);
}

fn main() {
  let file_path = "input.txt";

  let input = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");

  part_one(&input);
  part_two(&input);
}