use std::{collections::{hash_map, HashMap}, fs::File, io::{self, BufRead}};

fn read_int_to_vec(filename: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let (mut first_nums, mut second_nums) = (Vec::new(), Vec::new());
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            if let (Ok(num1), Ok(num2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                first_nums.push(num1);
                second_nums.push(num2);
            }
        }
    }
    Ok((first_nums, second_nums))
}

fn similarity_score(mut list1: Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut count_map = HashMap::new();
    let mut rolling_sum = 0;
    for &num in list2.iter() {
        *count_map.entry(num).or_insert(0) += 1;
    }
    for &num in &list1 {
        match count_map.get(&num) {
            Some(frequency) => rolling_sum += num * frequency,
            None => {}
        }
    }
    rolling_sum
}

fn main() {
    let filename = "/Users/braedenaudlin/fleet/Java-Stuff/advent_of_code/day_1/src/values.txt";
    let (vec1, vec2) = match read_int_to_vec(filename) {
        Ok((first, second)) => (first, second),
        Err(e) => { 
            eprintln!("Error reading file: {}", e);
            return;
        }
    };
    let result = similarity_score(vec1, &vec2);
    println!("{}", result);
}
