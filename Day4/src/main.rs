use std::fs;

fn check_matches(matches: &Vec<char>, line: &Vec<char>) -> i32 {
  for index in 1..matches.len() {
    if matches[index] != line[index] {
      return 0;
    }
  }

  return 1;
}

fn find_number_of_matches(matches: &Vec<char>, word_search: &Vec<Vec<char>>, x_index: usize, y_index: usize) -> i32 {
  let mut total: i32 = 0;

  // L -> R
  if x_index <= word_search[y_index].len()-4 {
    let mut line = Vec::new();
    for index in 0..matches.len() {
      line.push(word_search[y_index][x_index + index]);
    }

    total += check_matches(matches, &line);
  }

  // T -> B
  if y_index <= word_search.len()-4 {
    let mut line = Vec::new();
    for index in 0..matches.len() {
      line.push(word_search[y_index + index][x_index]);
    }

    total += check_matches(matches, &line);
  }

  //TL -> RB
  if y_index <= word_search.len()-4 && x_index <= word_search[y_index].len()-4 {
    let mut line = Vec::new();
    for index in 0..matches.len() {
      line.push(word_search[y_index + index][x_index + index]);
    }

    total += check_matches(matches, &line);
  }

  //TR -> LB
  if y_index <= word_search.len()-4 && x_index >= 3 {
    let mut line = Vec::new();
    for index in 0..matches.len() {
      line.push(word_search[y_index + index][x_index - index]);
    }

    total += check_matches(matches, &line);
  }

  return total;
}

fn is_x(matches_l: &Vec<char>, matches_r: &Vec<char>, word_search: &Vec<Vec<char>>, x_index: usize, y_index: usize) -> i32 {
  //TL -> RB
  let mut left_line = Vec::new();
  for index in 0..matches_l.len() {
    left_line.push(word_search[y_index + index][x_index + index]);
  }

  if check_matches(matches_l, &left_line) == 0 {
    return 0;
  }

  //TR -> LB
  let mut right_line = Vec::new();
  for index in 0..matches_r.len() {
    right_line.push(word_search[y_index + index][x_index + 2 - index]);
  }

  if check_matches(matches_r, &right_line) == 0 {
    return 0;
  }

  return 1;
}

fn part_two(word_search: &Vec<Vec<char>>) {
  let matches = Vec::from(['M', 'A', 'S']);
  let mut total: i32 = 0;

  for y_index in 0..word_search.len()-2 {
    for x_index in 0..word_search[y_index].len()-2 {
      let left_character = word_search[y_index][x_index];
      let mut left_match: Vec<char> = Vec::new();
      
      if left_character == 'S' {
        left_match = matches.clone().into_iter().rev().collect();
      } else if left_character == 'M' {
        left_match = matches.clone();
      }

      if left_match.len() > 0 {
        let right_character = word_search[y_index][x_index + 2];

        if right_character == 'S' {
          total += is_x(&left_match, &matches.clone().into_iter().rev().collect(), word_search, x_index, y_index);
        } else if right_character == 'M' {
          total += is_x(&left_match, &matches, word_search, x_index, y_index);
        }
      }
    }
  }
  
  println!("Part 2 total is: {}", total);
}

fn part_one(word_search: &Vec<Vec<char>>) {
  let matches = Vec::from(['X', 'M', 'A', 'S']);
  let mut total: i32 = 0;

  for y_index in 0..word_search.len() {
    for x_index in 0..word_search[y_index].len() {
      let character = word_search[y_index][x_index];

      if character == 'S' {
        total += find_number_of_matches(&matches.clone().into_iter().rev().collect(), &word_search, x_index, y_index);
      } else if character == 'X' {
        total += find_number_of_matches(&matches, &word_search, x_index, y_index);
      }
    }
  }
  
  println!("Part 1 total is: {}", total);
}

fn main() {
  let file_path = "input.txt";

  let input = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");

  let mut word_search: Vec<Vec<char>> = Vec::new();

  for line in input.split("\n") {
    word_search.push(line.chars().collect());
  }

  part_one(&word_search);
  part_two(&word_search);
}