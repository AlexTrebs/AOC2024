use std::fs;

fn char_to_int(input: &char) -> u64 {
    return input.to_string().parse::<u64>().unwrap_or_else(|_| panic!("{}", input));
}

fn find_total(number: usize, amount: &u16, start: &u64) -> u64 {
    let mut total: u64 = 0;
    for index in *start..(start + *amount as u64) {
        total += index * number as u64;
    }

    return total;
}

fn part_one(input: &mut Vec<u16>) {
    let mut start_index: u64 = 0;
    let mut diff: i16 = 0;
    let mut total: u64 = 0;
    let mut length: usize = input.clone().len().try_into().unwrap();

    'outer: for index in 0..input.len() {
        if index == length {
            break;
        }

        if index % 2 == 0 {
            total += find_total(index/2, &input[index], &start_index);

            start_index += input[index] as u64;
            input[index] = 0;

            continue;
        }

        while index != length - 1 && input[index] > 0 {
            if length % 2 == 0 {
                input.pop();
                length -= 1;

                continue;
            }

            diff = input[index] as i16 - input[length - 1] as i16;

            if diff < 0 {
                total += find_total((length - 1)/2, &input[index], &start_index);

                start_index += input[index] as u64;

                input[length - 1] -= input[index];
                input[index] = 0;
            } else {
                total += find_total((length - 1)/2, &input[length - 1], &start_index);

                start_index += input[length - 1] as u64;
                input[index] = diff as u16;

                input.pop();
                length -= 1;
            }
        }
    }

   
    println!("Part 1 total is: {}", total);
}

fn part_two(amounts: &mut Vec<u16>) {
    let mut start_index: u64 = 0;
    let mut diff: i16 = 0;
    let mut total: u64 = 0;
    let mut length: usize = amounts.clone().len().try_into().unwrap();
    let mut check_index: usize = 1;
    let mut index_array: Vec<usize> = (0..amounts.len()).collect();

    for index in (1..amounts.len()).rev() {
        if index % 2 == 1 {
            continue;
        }

        let position: usize = index_array.iter().position(|x| *x as usize == index).unwrap();

        while position > check_index {
            if index_array[check_index] % 2 == 0 {

                check_index += 1;

                continue;
            }

            diff = amounts[check_index] as i16 - amounts[position] as i16;

            if diff > 0 {
                index_array[position] = index_array[check_index];
                index_array.insert(check_index, index.try_into().unwrap());

                let amount = amounts[position];

                amounts[check_index] = diff as u16;
                amounts.insert(check_index, amount);

                break;
            } else if diff == 0 {
                index_array[position] = index_array[check_index];
                index_array[check_index] = index;

                let amount = amounts[position];
                amounts[position] = amounts[check_index];
                amounts[check_index] = amounts[position];

                break;
            }

            check_index += 1;
        }

        diff = 0;
        check_index = 1;
    }

    for (index, num) in index_array.iter().enumerate() {
        if num % 2 == 1 {
            start_index += amounts[index] as u64;
            continue;
        }
        
        total += find_total(*num/2, &amounts[index], &start_index);

        start_index += amounts[index] as u64;
    }

    println!("Part 2 total is: {}", total);
}

fn main() {
    let file_path = "input.txt";

    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut nums: Vec<u16> = input.chars().map(|x| char_to_int(&x) as u16).collect();

    part_one(&mut nums.clone());
    part_two(&mut nums.clone());
}
