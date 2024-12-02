use std::fs::File;
use std::io::{self, BufRead};

fn read_file_to_vectors(filename: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let mut first_numbers = Vec::new();
    let mut second_numbers = Vec::new();

    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            if let (Ok(num1), Ok(num2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                first_numbers.push(num1);
                second_numbers.push(num2);
            }
        }
    }
    Ok((first_numbers, second_numbers))
}

fn find_difference(first_list: &mut Vec<i32>, second_list: &mut Vec<i32>) -> i32 {
    let mut rolling_sum = 0;
    first_list.sort();
    second_list.sort();
    for (a, b) in first_list.iter().zip(second_list.iter()) {
        match a > b {
            true => rolling_sum += a - b,
            false => rolling_sum += b - a,
        };
    }
    rolling_sum
}

fn main() {
    let filename = "/Users/braedenaudlin/fleet/Java-Stuff/advent_of_code/day_1/src/values.txt";

    let (mut vec1, mut vec2) = match read_file_to_vectors(filename) {
        Ok((first, second)) => (first, second),
        Err(e) => { 
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let result = find_difference(&mut vec1, &mut vec2);
    println!("{}", result);
}

