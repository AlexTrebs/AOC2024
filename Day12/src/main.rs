use std::fs;
use fxhash::FxHashSet;

fn str_to_int(input: &str) -> usize {
  return input.parse::<usize>().unwrap();
}

fn find_char(field: &Vec<Vec<char>>, y: i32, x: i32) -> char {
  if y < 0 || y >= field.len() as i32
  || x < 0 || x >= field[y as usize].len() as i32 {
    return '#';
  } else {
    return field[y as usize][x as usize];
  }
}

fn find_perimeter(
    prev_char: &char, y: usize, x: usize, 
    visited_indexes: &mut FxHashSet<(usize, usize)>, 
    field: &Vec<Vec<char>>
) -> (usize, usize) {
  if visited_indexes.contains(&(y, x)) {
    return (0, 0);
  }

  visited_indexes.insert((y,x));

  let mut perimiter: usize = 0;
  let mut adjacent: usize = 0;
  let mut amount: usize = 0;
  let directions: Vec<(i32, i32)> = Vec::from([(0, 1), (-1,0), (0,-1), (1,0)]);
  
  for (y_dir, x_dir) in directions {
    let y_new: i32 = y as i32 + y_dir;
    let x_new: i32 = x as i32 + x_dir;

    if y_new < 0 || y_new >= field.len() as i32
        || x_new < 0 || x_new >= field[y_new as usize].len() as i32 {
      continue;
    }

    if *prev_char == field[y_new as usize][x_new as usize] {
      adjacent += 1;
      let (a, p) = find_perimeter(&field[y_new as usize][x_new as usize], y_new as usize, x_new as usize, visited_indexes, field);
      amount += a;
      perimiter += p;
    }
  }

  return (amount + 1, perimiter + 4 - adjacent);
}

fn find_sides(
  prev_char: &char,
  y: usize,
  x: usize,
  visited_indexes: &mut FxHashSet<(usize, usize)>,
  field: &Vec<Vec<char>>
) -> (usize, usize) {
  if visited_indexes.contains(&(y, x)) {
    return (0, 0);
  }

  visited_indexes.insert((y,x));

  let directions: Vec<(i32, i32)> = Vec::from([(0, 1), (-1,0), (0,-1), (1,0)]);

  let corner_directions: Vec<Vec<(i32, i32)>> = Vec::from([
    Vec::from([(-1, -1), (-1,0), (0,-1)]), 
    Vec::from([(1, -1), (1,0), (0,-1)]), 
    Vec::from([(-1, 1), (-1,0), (0,1)]), 
    Vec::from([(1, 1), (1,0), (0,1)]), 
  ]);

  let mut corners: usize = 0;
  let mut amount: usize = 0;

  for vec in corner_directions {
    let c_new_row: char = find_char(field, y as i32 + vec[1].0, x as i32 + vec[1].1);
    let c_new_col: char = find_char(field, y as i32 + vec[2].0, x as i32 + vec[2].1);
    let c_new_diag: char = find_char(field, y as i32 + vec[0].0, x as i32 + vec[0].1);

    if c_new_col == *prev_char && c_new_row == *prev_char && c_new_diag != *prev_char {
      corners += 1;
    } else if c_new_col != *prev_char && c_new_row != *prev_char {
      corners += 1;
    }
  }
  
  for (y_dir, x_dir) in directions {
    let y_new: i32 = y as i32 + y_dir;
    let x_new: i32 = x as i32 + x_dir;

    if y_new < 0 || y_new >= field.len() as i32
        || x_new < 0 || x_new >= field[y_new as usize].len() as i32 {
      continue;
    }

    if *prev_char == field[y_new as usize][x_new as usize] {
      let (a, c) = find_sides(&field[y_new as usize][x_new as usize], y_new as usize, x_new as usize, visited_indexes, field);
      amount += a;
      corners += c;
    }
  }

  return (amount + 1, corners);
}

fn part_one(field: &Vec<Vec<char>>) {
  let mut visited_indexes: FxHashSet<(usize, usize)> = FxHashSet::default();
  let mut perimiter: usize = 0;
  let mut amount: usize = 0;
  let mut total: usize = 0;

  for (y, line) in field.into_iter().enumerate() {
    for (x, c) in line.into_iter().enumerate() {
      (amount, perimiter) = find_perimeter(&c, y, x, &mut visited_indexes, field);
      total += amount * perimiter;
    }
  }

  println!("Part 1 total is: {}", total);
}

fn part_two(field: &Vec<Vec<char>>) {
  let mut visited_indexes: FxHashSet<(usize, usize)> = FxHashSet::default();
  let mut amount: usize = 0;
  let mut total: usize = 0;
  let mut sides: usize = 0;

  for (y, line) in field.into_iter().enumerate() {
    for (x, c) in line.into_iter().enumerate() {
      (amount, sides) = find_sides(&c, y, x, &mut visited_indexes, field);
      total += amount * sides;
    }
  }

  println!("Part 2 total is: {}", total);
}

fn main() {
  let file_path = "input.txt";

  let input = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");

  let mut field: Vec<Vec<char>> = Vec::new();

  for line in input.split("\n") {
    field.push(line.chars().collect());
  }

  part_one(&mut field);
  part_two(&mut field);
}