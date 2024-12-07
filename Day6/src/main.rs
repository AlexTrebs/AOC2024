use std::fs;
use std::collections::HashMap;

fn find_elem(matrix: &Vec<Vec<char>>, c: char) -> (i32, i32) {
    let mut x_index: i32 = 0;
    let mut y_index: i32 = 0;

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len(){
            if matrix[y][x] == c {
                x_index = x as i32;
                y_index = y as i32;
                break;
            }
        }

        if x_index != 0 {
            break;
        }
    }

    return (x_index, y_index)
}

fn count_elems(matrix: &Vec<Vec<char>>, c: char) -> i32 {
    let mut total: i32 = 0;
    
    for line in matrix {
        total += line.iter().filter(|&n| *n == c).count() as i32;
    }

    return total;
}

fn check_loop(matrix: &Vec<Vec<char>>, mr_man_x: &i32, mr_man_y: &i32, direction: (i32, i32), direction_map: &HashMap<(i32, i32), char>) -> bool {

    if mr_man_x + 2 * direction.1 < (matrix[0].len()).try_into().unwrap()
        && mr_man_y + 2 * direction.0 < (matrix.len()).try_into().unwrap()
        && mr_man_x + 2 * direction.1 >= 0 && mr_man_y + 2 * direction.0 >= 0 {

        if matrix[(mr_man_y + direction.0) as usize][(mr_man_x + direction.1) as usize] == *direction_map.get(&direction).unwrap()
            || (matrix[(mr_man_y + 2 * direction.0) as usize][(mr_man_x + 2 * direction.1) as usize] == '#' 
                && matrix[(mr_man_y + direction.0 +  direction.1) as usize][(mr_man_x + direction.1 - direction.0) as usize] == *direction_map.get(&(-direction.0, -direction.1)).unwrap() 
                && matrix[(mr_man_y + direction.0 + 2 * direction.1) as usize][(mr_man_x + direction.1 - 2 * direction.0) as usize] == '#')
            || (matrix[(mr_man_y + 2 * direction.0) as usize][(mr_man_x + 2 * direction.1) as usize] == '#' 
                && matrix[(mr_man_y + direction.0 + direction.1) as usize][(mr_man_x + direction.1 - direction.0) as usize] == '#'
                && matrix[(mr_man_y + direction.0) as usize][(mr_man_x + direction.1) as usize] == *direction_map.get(&(-direction.0, -direction.1)).unwrap()) {
            // println!("-------------------------------------------------------------------");
            // for line in matrix {
            //     let mut line_string: String = String::new();
            //     for c in line {
            //         line_string.push_str(&c.to_string());
            //     }
            //     println!("{:?}", line_string);
            // }
            // println!("-------------------------------------------------------------------");
            return true;
        }
    }

    return false;
}

fn run_simulation(matrix: &mut Vec<Vec<char>>) -> bool {
    let (mut mr_man_x, mut mr_man_y) = find_elem(matrix, '^');
    let mut direction: (i32, i32) = (-1,0);
    let direction_map = HashMap::from([
        ((-1,0), 'N'),
        ((0,1), 'E'),
        ((1,0), 'S'),
        ((0,-1), 'W'),
    ]);
    while mr_man_x + direction.1 < (matrix[0].len()).try_into().unwrap()
            && mr_man_y + direction.0 < (matrix.len()).try_into().unwrap()
            && mr_man_x + direction.1 >= 0 && mr_man_y + direction.0 >= 0 {

        if matrix[(mr_man_y + direction.0) as usize][(mr_man_x + direction.1) as usize] != '#' 
                && matrix[(mr_man_y + direction.0) as usize][(mr_man_x + direction.1) as usize] != 'O' {
            if check_loop(matrix, &mr_man_x, &mr_man_y, direction, &direction_map) {
                return false;
            }

            matrix[(mr_man_y) as usize][(mr_man_x) as usize] = *direction_map.get(&direction).unwrap();
            matrix[(mr_man_y + direction.0) as usize][(mr_man_x + direction.1) as usize] = '^';

            mr_man_x += direction.1;
            mr_man_y += direction.0;
        } else {
            direction = (direction.1, -direction.0);
        }
    }
    matrix[(mr_man_y) as usize][(mr_man_x) as usize] = *direction_map.get(&direction).unwrap();

    return true;
}

fn part_one(matrix: &mut Vec<Vec<char>>) {
    
    if run_simulation(matrix) {
        let mut total: i32 = 0;
        
        total += count_elems(&matrix, 'N');
        total += count_elems(&matrix, 'E');
        total += count_elems(&matrix, 'S');
        total += count_elems(&matrix, 'W');
    
        println!("Part 1 total is: {}", total);
    } else {
        println!("Failed!");
    }
}

fn part_two(matrix: &mut Vec<Vec<char>>) {
    let temp_matrix: Vec<Vec<char>> = matrix.clone();
    let mut inprogress_matrix: Vec<Vec<char>> = matrix.clone();

    let mut total: i32 = 0;

    run_simulation(matrix);

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            let c = matrix[y][x];
            if c == 'N' || c == 'E' || c == 'S' || c == 'W' {
                inprogress_matrix[y][x] = 'O';

                if !run_simulation(&mut inprogress_matrix) {
                    total += 1;
                }

                inprogress_matrix = temp_matrix.clone();
            }
        }
    }

    println!("Part 2 total is: {}", total);
}

fn main() {
    let file_path = "input.txt";

    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let matrix: &mut Vec<Vec<char>> = &mut Vec::new();

    for line in input.split("\n") {
        if line.chars().count() > 0 {
            matrix.push(line.chars().collect());
        }
    }

    part_one(&mut matrix.clone());
    part_two(&mut matrix.clone());
}
