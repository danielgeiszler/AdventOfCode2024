use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let records = load_file("../Inputs/Day2-1.txt")?;
    let n_safe_records = count_safe_records(records);

    println!("n_safe_records: {}", n_safe_records);

    Ok(())
}

fn load_file(file_path: &str) -> io::Result<Vec<Vec<i32>>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;

    let mut records = Vec::new();

    for line in io::BufReader::new(file).lines() {
        let mut levels = Vec::new();
        let line = line?; // Handle potential errors
        let values = line
            .split_whitespace()       // Split the line by whitespace
            .map(|s| s.parse().unwrap()); // Convert each item to i32

        for value in values {
            levels.push(value);
        }

        records.push(levels);
    }

    Ok(records)
}

fn count_safe_records(records: Vec<Vec<i32>>) -> i32 {
    let mut n_safe_records = 0;

    for i in 0..records.len() {
        let mut monotonic_increasing = false;
        let mut monotonic_decreasing = false;
        let mut safe = true;
        for j in 0..(records[i].len() - 1) {
            if j == 0 {
                if records[i][j] <= records[i][j + 1] {
                    monotonic_increasing = true;
                } else if records[i][j] >= records[i][j + 1] {
                    monotonic_decreasing = true;
                }
            }
            let diff = records[i][j + 1] - records[i][j];

            if diff >= 1 && diff <= 3 {
                if monotonic_decreasing {
                    safe = false;
                    break;
                }
            } else if diff >= -3 && diff <= -1 {
                if monotonic_increasing {
                    safe = false;
                    break;
                }
            } else {
                safe = false;
                break;
            }
        }
        if safe {n_safe_records += 1}
    }

    n_safe_records
}