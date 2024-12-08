use std::fs;
use std::collections::HashSet;

fn find_indices(matrix: &Vec<Vec<char>>, target: char) -> Vec<(i32, i32)> { 
    let mut indices = Vec::new(); 
    
    for (i, line) in matrix.iter().enumerate() { 
        for (j, &val) in line.iter().enumerate() { 
            if val == target { 
                indices.push((i as i32, j as i32)); 
            } 
        } 
    } 
    
    return indices;
}

fn is_avaliable(matrix: &Vec<Vec<char>>, x_index: i32, y_index: i32) -> bool {
    return y_index >= 0 && y_index < matrix.len().try_into().unwrap()
            && x_index >= 0 && x_index < matrix[y_index as usize].len().try_into().unwrap();
}

fn part_one(mut matrix: Vec<Vec<char>>, unique_chars: &Vec<char>) {
    let mut new_matrix = matrix.clone();

    for c in unique_chars {
        let indices = find_indices(&matrix, *c);

        for (i, (y, x)) in indices[0..indices.len()-1].iter().enumerate() {
            for (i, (y_match, x_match)) in indices[i+1..].iter().enumerate() {
                let (y_diff, x_diff) = (y_match - y, x_match - x);

                if is_avaliable(&matrix, y-y_diff, x-x_diff) {
                    new_matrix[(y-y_diff) as usize][(x-x_diff) as usize] = '#';
                }

                if is_avaliable(&matrix, y_match + y_diff, x_match + x_diff) {
                    new_matrix[(y_match + y_diff) as usize][(x_match + x_diff) as usize] = '#';
                }
            }
        }
    }

    // println!("-------------------------------------------------------------------");
    // for line in &new_matrix {
    //     let mut line_string: String = String::new();
    //     for c in line {
    //         line_string.push_str(&c.to_string());
    //     }
    //     println!("{:?}", line_string);
    // }
    // println!("-------------------------------------------------------------------");
    
    println!("Part 1 total is: {}", find_indices(&new_matrix, '#').len());
}

fn part_two(mut matrix: Vec<Vec<char>>, unique_chars: &Vec<char>) {
    let mut new_matrix = matrix.clone();

    for c in unique_chars {
        let indices = find_indices(&matrix, *c);

        for (i, (y, x)) in indices[0..indices.len()-1].iter().enumerate() {
            for (i, (y_match, x_match)) in indices[i+1..].iter().enumerate() {
                let (y_diff, x_diff) = (y_match - y, x_match - x);
                new_matrix[*(y) as usize][*(x) as usize] = '#';
                new_matrix[*(y_match) as usize][*(x_match) as usize] = '#';

                let (mut y_1, mut x_1) = (y-y_diff, x-x_diff);
                let (mut y_2, mut x_2) = (y_match + y_diff, x_match + x_diff);

                while is_avaliable(&matrix, y_1, x_1) {
                    new_matrix[(y_1) as usize][(x_1) as usize] = '#';

                    (y_1, x_1) = (y_1 - y_diff, x_1 - x_diff);
                }

                while is_avaliable(&matrix, y_2, x_2) {
                    new_matrix[(y_2) as usize][(x_2) as usize] = '#';

                    (y_2, x_2) = (y_2 + y_diff, x_2 + x_diff);
                }
            }
        }
    }

    // println!("-------------------------------------------------------------------");
    // for line in &new_matrix {
    //     let mut line_string: String = String::new();
    //     for c in line {
    //         line_string.push_str(&c.to_string());
    //     }
    //     println!("{:?}", line_string);
    // }
    // println!("-------------------------------------------------------------------");
    
    println!("Part 2 total is: {}", find_indices(&new_matrix, '#').len());
}

fn main() {
    let file_path = "input.txt";

    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut matrix: &mut Vec<Vec<char>> = &mut Vec::new();

    let mut unique_chars: Vec<char> = Vec::new();
    let mut seen = HashSet::new(); 
    
    for c in input.chars().filter(|c| *c != '.' && *c != '\n') { 
        if seen.insert(c) { 
            unique_chars.push(c);
        }
    }

    for line in input.split("\n") {
        if line.chars().count() > 0 {
            matrix.push(line.chars().collect());
        }
    }

    part_one(matrix.clone(), &unique_chars);
    part_two(matrix.clone(), &unique_chars);
}
