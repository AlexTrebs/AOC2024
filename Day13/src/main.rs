use std::fs;
use regex::Regex;

fn str_to_int(input: &str) -> i32 {
  return input.parse::<i32>().unwrap();
}

fn solve_equation(games: &Vec<Vec<(usize, usize)>>, scalar: i64) -> i64 { 
  let mut total: i64 = 0;

  for game in games {
    let a_x = game[0].0 as i64;
    let a_y = game[0].1 as i64;
    let b_x = game[1].0 as i64;
    let b_y = game[1].1 as i64;

    let mut prize_x = game[2].0 as i64 + scalar;
    let mut prize_y = game[2].1 as i64 + scalar;

    let mut top = a_y * prize_x - a_x * prize_y;
    let mut bottom = a_y * b_x - a_x * b_y;
  
    if bottom == 0 || top % bottom != 0 { 
      continue;
    }

    let b = top / bottom;
    top = prize_x - b * b_x;
    bottom = a_x;

    if bottom == 0 || top % bottom != 0 {
      continue;
    }

    let a = top / bottom;

    total += a*3 + b;    
  }

  return total;
}

fn part_one(games: &Vec<Vec<(usize, usize)>>) {
  println!("Part 1 total is: {}", solve_equation(games, 0));
}

fn part_two(games: &Vec<Vec<(usize, usize)>>) {
  println!("Part 2 total is: {}", solve_equation(games, 10000000000000));
}

fn main() {
  let re = Regex::new(r"\w+:\s*X[\+=](\d+),\s*Y[\+=](\d+)").unwrap();

  let file_path = "input.txt";

  let input = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");

  let mut games: Vec<Vec<(usize, usize)>> = Vec::new();

  for group in input.split("\n\n") {
    let mut game_vec: Vec<(usize, usize)> = Vec::new();

    for cap in re.captures_iter(group) {
      game_vec.push((cap[1].parse().unwrap(), cap[2].parse().unwrap()));
    }

    games.push(game_vec);
  }

  part_one(&games);
  part_two(&games);
}