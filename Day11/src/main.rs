use std::fs;
use fxhash::FxHashMap;

fn str_to_int(input: &str) -> usize {
  return input.parse::<usize>().unwrap();
}

fn find_stone_total(stone: usize, explored_stones: &mut FxHashMap<(usize, usize), usize>, blinks_left: usize) -> usize {
  if blinks_left == 0 {
    return 1;
  }
  if let Some(num) = explored_stones.get(&(blinks_left, stone)) {
    return *num;
  } 
  
  let number_of_stones: usize = if stone == 0 {
    find_stone_total(1, explored_stones, blinks_left - 1)

  } else if stone.to_string().len() % 2 == 0 {
    let middle: usize = stone.to_string().len()/2;

    let start: usize = str_to_int(&stone.to_string()[..middle]);
    let end: usize = str_to_int(&stone.to_string()[middle..]);
    
    find_stone_total(start, explored_stones, blinks_left - 1) + find_stone_total(end, explored_stones, blinks_left - 1)
  } else {
    find_stone_total(stone*2024, explored_stones, blinks_left - 1)
  };

  explored_stones.insert((blinks_left, stone), number_of_stones);
  return number_of_stones;
}

fn part_one(stones: &mut Vec<usize>) {
  let mut explored_stones: FxHashMap<(usize, usize), usize> = FxHashMap::default();
  const NUM_OF_BLINKS: usize = 25;
  let mut total: usize = 0;

  for stone in stones {
    total += find_stone_total(*stone, &mut explored_stones, NUM_OF_BLINKS);
  }

  println!("Part 1 total is: {}", total);
}

fn part_two(stones: &mut Vec<usize>) {
  let mut explored_stones: FxHashMap<(usize, usize), usize> = FxHashMap::default();
  const NUM_OF_BLINKS: usize = 75;
  let mut total: usize = 0;

  for stone in stones {
    total += find_stone_total(*stone, &mut explored_stones, NUM_OF_BLINKS);
  }
  println!("Part 2 total is: {}", total);
}

fn main() {
  let file_path = "input.txt";

  let input = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");

  let stones: Vec<usize> = input.split(" ").map(|x| str_to_int(x)).collect();

  part_one(&mut stones.clone());
  part_two(&mut stones.clone());
}