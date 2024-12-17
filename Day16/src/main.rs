use std::{collections::{BinaryHeap, HashMap}, fs, usize};

fn where_am_i(matrix: &Vec<Vec<char>>, me: &char) -> (usize, usize) {
  for y in 0..matrix.len() {
    for x in 0..matrix[y].len() {
      if matrix[y][x] == *me {
        return (y, x);
      }
    }
  }

  return (0,0);
}

fn a_star_algorithm(matrix: &Vec<Vec<char>>, prev_position: &(usize, usize), prev_direction: &(isize, isize)) -> (HashMap<((usize, usize), (isize, isize)), usize>, usize) {
  let mut open_set: BinaryHeap<((usize, usize), (isize, isize), usize)> = BinaryHeap::new();
  let mut f_score: HashMap<((usize, usize), (isize, isize)), usize> = HashMap::new();

  let mut lowest: usize = usize::MAX;

  f_score.insert((*prev_position, *prev_direction), 0);
  open_set.push((*prev_position, *prev_direction, 0));

  while open_set.len() != 0 {
    let (current_position, current_direction, cost) = open_set.pop().unwrap();
    
    if cost > lowest {
      continue;
    }

    if matrix[current_position.0][current_position.1] == 'E' {
      if cost < lowest {
        lowest = cost;
      }

      continue;
    }

    let next = [
      (((current_position.0 as isize + current_direction.0) as usize, (current_position.1 as isize + current_direction.1) as usize), current_direction, cost + 1),
      (current_position, (current_direction.1, current_direction.0), cost + 1000),
      (current_position, (-current_direction.1, -current_direction.0), cost + 1000),
    ];

    for (new_position, new_direction, cost) in next {
      if matrix[new_position.0][new_position.1] == '#' {
        continue;
      }

      if cost < *f_score.get(&(new_position, new_direction)).unwrap_or(&usize::MAX) {
        f_score.insert((new_position, new_direction), cost);
        open_set.push((new_position, new_direction, cost));
      }
    };
  }

  return (f_score, lowest);
}

fn part_one(total: &usize) {
  println!("Part 1 total is: {}", *total);
}

fn part_two(matrix: &mut Vec<Vec<char>>, visited: &mut HashMap<((usize, usize), (isize, isize)), usize>, lowest: &usize, end: &(usize, usize), start: &(usize, usize)) {
  let mut open_set: BinaryHeap<((usize, usize), (isize, isize), usize)> = BinaryHeap::new();

  let directions: Vec<(isize, isize)> = Vec::from([
    (0, -1),
    (0, 1),
    (-1, 0),
    (1, 0),
  ]);

  for dir in directions {
    if *visited.get(&(*end, dir)).unwrap_or(&usize::MAX) == *lowest {
      open_set.push((*end, dir, *lowest));
    }
  }

  while open_set.len() != 0 {
    let (current_position, current_direction, cost) = open_set.pop().unwrap();

    matrix[current_position.0][current_position.1] = 'O';

    if current_position == *start {
      continue;
    }

    let next = [
      (((current_position.0 as isize - current_direction.0) as usize, (current_position.1 as isize - current_direction.1) as usize), current_direction, cost - 1),
      (current_position, (current_direction.1, current_direction.0), cost - 1000),
      (current_position, (-current_direction.1, -current_direction.0), cost - 1000),
    ];

    for (new_position, new_direction, new_cost) in next {
      if new_cost == *visited.get(&(new_position, new_direction)).unwrap_or(&usize::MIN) {
        open_set.push((new_position, new_direction, new_cost));
        
        visited.insert((new_position, new_direction), usize::MAX);
      }
    };

  }

  let mut total: usize = 0;

  for line in matrix {
    for c in line {
      if *c == 'O' {
        total += 1;
      }
    }
  }

  println!("Part 2 total is: {}", total);
}

fn main() {
  let file_path = "input.txt";

  let input = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");

  let mut matrix: Vec<Vec<char>> = Vec::new();
  
  for line in input.split("\n") {
    matrix.push(line.chars().collect());
  }

  let start: (usize, usize) = where_am_i(&matrix, &'S');
  let end: (usize, usize) = where_am_i(&matrix, &'E');

  let (mut f_score, total) = a_star_algorithm(&matrix, &start, &(0, 1));

  part_one(&total);
  part_two(&mut matrix, &mut f_score, &total, &end, &start);
}
