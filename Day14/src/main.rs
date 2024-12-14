use std::fs;
use regex::Regex;
use std::fs::OpenOptions;
use std::io::Write;

fn str_to_int(input: &str) -> i32 {
  return input.parse::<i32>().unwrap();
}

fn print_map(robots: &mut Vec<Vec<(i32, i32)>>, max_x: &i32, max_y: &i32, sec: i32) {
  let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open("output.txt")
    .expect("Unable to find file");

  writeln!(file, "---------------------------------{sec}----------------------------------").expect("Unable to write file");

  let mut grid = vec![vec!["-".to_string(); *max_x as usize]; *max_y as usize];

  for robot in robots {
    if grid[robot[0].1 as usize][robot[0].0 as usize] != "-" {
      grid[robot[0].1 as usize][robot[0].0 as usize] = (str_to_int(&grid[robot[0].1 as usize][robot[0].0 as usize]) + 1).to_string();
    } else {
      grid[robot[0].1 as usize][robot[0].0 as usize] = "1".to_string();
    }
  }

  for line in grid {
    let mut line_string: String = String::new();
    for c in line {
      line_string.push_str(&c.to_string());
    }

    writeln!(file, "{}", line_string).expect("Unable to write file");
  }
  writeln!(file, "-------------------------------------------------------------------").expect("Unable to write file");
}

fn update_robots(robots: &mut Vec<Vec<(i32, i32)>>, max_x: &i32, max_y: &i32) {
  for (index, robot) in robots.clone().into_iter().enumerate() {
    let mut new_pos = (robot[0].0 + robot[1].0, robot[0].1 + robot[1].1);

    if new_pos.0 < 0 {
      new_pos = (new_pos.0 + max_x, new_pos.1);
    } else if new_pos.0 >= *max_x {
      new_pos = (new_pos.0 - max_x, new_pos.1);
    }     
    
    if new_pos.1 < 0 {
      new_pos = (new_pos.0, new_pos.1 + max_y);
    } else if new_pos.1 >= *max_y {
      new_pos = (new_pos.0, new_pos.1 - max_y);
    }

    robots[index] = Vec::from([new_pos, robot[1]]);
  }
}

fn part_one(robots: &mut Vec<Vec<(i32, i32)>>) {
  let (max_x, max_y) = (101,103);
  let num_seconds: i32 = 100;

  for _sec in 0..num_seconds {
    update_robots(robots, &max_x, &max_y);
  }
  
  let x_halves = [0..(max_x/2), (max_x/2)+1..max_x];
  let y_halves = [0..(max_y/2), (max_y/2)+1..max_y];
  let mut total = 1;

  for x in &x_halves {
    for y in &y_halves {
      total = total * robots.iter().filter(|robot| x.contains(&robot[0].0) && y.contains(&robot[0].1)).count();
    }
  }

  println!("Part 1 total is: {}", total);
}

fn part_two(robots: &mut Vec<Vec<(i32, i32)>>) {
  let (max_x, max_y) = (101,103);
  let num_seconds: i32 = 7000;

  for sec in 0..num_seconds {
    update_robots(robots, &max_x, &max_y);

    if sec == 6667 {
      print_map(robots, &max_x, &max_y, sec);
    }
  }
}

fn main() {
  let re = Regex::new(r"([-+]?\d+),([-+]?\d+)").unwrap();

  let file_path = "input.txt";

  let input = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");

  let mut robots: Vec<Vec<(i32, i32)>> = Vec::new();

  for robot in input.split("\n") {
    let mut robot_vec: Vec<(i32, i32)> = Vec::new();

    for cap in re.captures_iter(robot) {
      robot_vec.push((str_to_int(&cap[1]), str_to_int(&cap[2])));
    }

    robots.push(robot_vec);
  }

  part_one(&mut robots.clone());
  part_two(&mut robots.clone());
}