use std::{fs::File, io::{self, BufRead}, path::Path};

fn read_levels(filename: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let levels: Vec<i32> = line.split_whitespace().map(|word| word.parse::<i32>()).filter_map(Result::ok).collect();
        reports.push(levels);
    }
    Ok(reports)
} 
struct Part1;
impl Part1 {
    fn find_safe_reports(reports: io::Result<Vec<Vec<i32>>>) -> i32 {
        let mut safe_counter = 0;
        let reports = match reports {
            Ok(data) => data,
            Err(e) => {
                println!("Error reading the data: {}", e);
                return -1;
            }
        };

        for array in &reports {
            if array.len() < 2 {
                safe_counter += 1;
                continue;
            }

            let mut working: bool = true;
            let increase: bool = array[0] < array[1];

            for i in 0..array.len() - 1 {
                let diff = (array[i + 1] - array[i]).abs();
                if diff < 1 || diff > 3 {
                    working = false;
                    break;
                }
                if increase && array[i] > array[i + 1] {
                    working = false;
                    break;
                }
                else if !increase && array[i] < array[i + 1] {
                    working = false;
                    break;
                }
            }
            if working {
                safe_counter += 1;
            }
        }
        safe_counter
    }
}
struct Part2;
impl Part2 {
    fn is_safe(report: &[i32]) -> bool {
        let mut is_increasing = true;
        let mut is_decreasing = true;

        for i in 1..report.len() {
            let diff = (report[i] - report[i - 1]).abs();
            if diff < 1 || diff > 3 {
                return false;
            }
            if report[i] < report[i - 1] {
                is_increasing = false;
            }
            if report[i - 1] < report[i] {
                is_decreasing = false;
            }
        }
        is_increasing || is_decreasing
    }
    fn is_safe_one_removal(report: &[i32]) -> bool {
        for i in 0..report.len() {
            let mut modified_report = report.to_vec();
            modified_report.remove(i);
            if Self::is_safe(&modified_report) {
                return true;
            }
        }
        false
    }
}

fn main() -> io::Result<()> {
    let path = Path::new("src/levels.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut safe_count = 0;

    for line in reader.lines() {
        let line = line?;
        let report: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        if Part2::is_safe(&report) || Part2::is_safe_one_removal(&report) {
            safe_count += 1;
        }
    }

    println!("Number of safe reports: {}", safe_count);

    Ok(())
}
