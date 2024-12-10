use std::fs;

fn char_to_int(input: &char) -> usize {
    return input.to_string().parse::<usize>().unwrap_or_else(|_| panic!("{}", input));
}

fn search(num: &usize, input: &Vec<Vec<usize>>, y: usize, x: usize, mut already_travelled: &mut Vec<(usize, usize)>, part_two: bool) -> usize {
    let diffs: Vec<(i32, i32)> = Vec::from([(0, 1), (0, -1), (1, 0), (-1, 0)]);
    let mut total: usize = 0;

    for (y_diff, x_diff) in &diffs {
        if y as i32 + *y_diff < 0 || input.len() <= (y as i32 + *y_diff) as usize {
            continue;
        }

        let new_y: usize = (y as i32 + *y_diff) as usize;

        if x  as i32 + *x_diff < 0 || input[new_y].len() <= (x as i32 + *x_diff) as usize {
            continue;
        }

        let new_x: usize = (x as i32 + *x_diff) as usize;

        if !part_two {
            if already_travelled.contains(&(new_y, new_x)) {
                continue;
            }
    
            if *num == 0 && input[new_y][new_x] == 0 {
                already_travelled.push((new_y, new_x));
                total += 1;
            }
        } else {
            if *num == 0 && input[new_y][new_x] == 0 {
                total += 1;
            }
        }

        if *num == 0 {
            continue;
        }  

        if input[new_y][new_x] == *num {
            total += search(&(num - 1), input, new_y, new_x, already_travelled, part_two);
        }
    }

    return total;
}

fn part_one(input: &Vec<Vec<usize>>) {
    let mut total: usize = 0;

    for (y, line) in input.iter().enumerate() {
        for (x, num) in line.iter().enumerate() {
            if *num == 9 {
                total += search(&(num - 1), input, y, x, &mut Vec::new(), false);
            }
        }
    }
   
    println!("Part 1 total is: {}", total);
}

fn part_two(input: &Vec<Vec<usize>>) {
    let mut total: usize = 0;

    for (y, line) in input.iter().enumerate() {
        for (x, num) in line.iter().enumerate() {
            if *num == 9 {
                total += search(&(num - 1), input, y, x, &mut Vec::new(), true);
            }
        }
    }
   
    println!("Part 2 total is: {}", total);
}

fn main() {
    let file_path = "input.txt";

    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut matrix: Vec<Vec<usize>> = Vec::new();
    
    for line in input.split("\n") {
        if line.chars().count() > 0 {
            matrix.push(line.chars().map(|x| char_to_int(&x)).collect());
        }
    }

    part_one(&matrix);
    part_two(&matrix);
}
