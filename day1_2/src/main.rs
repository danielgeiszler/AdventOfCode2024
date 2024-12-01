use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let (mut col1, mut col2) = load_file("../Inputs/Day1-1.txt")?;
    sort_columns(&mut col1, &mut col2);
    let similarity_score = compute_similarity_score(col1, col2);

    println!("{}", similarity_score);

    Ok(())
}

fn load_file(file_path: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;

    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    for line in io::BufReader::new(file).lines() {
        let line = line?; // Handle potential errors
        let mut values = line
            .split_whitespace()       // Split the line by whitespace
            .map(|s| s.parse().unwrap()); // Convert each item to i32

        if let (Some(val1), Some(val2)) = (values.next(), values.next()) {
            col1.push(val1);
            col2.push(val2);
        }
    }


    Ok((col1, col2))
}

fn sort_columns(col1: &mut Vec<i32>, col2: &mut Vec<i32>) {
    col1.sort();
    col2.sort();
}

fn compute_similarity_score(col1: Vec<i32>, col2: Vec<i32>) -> i32 {
    let mut similarity_score = 0;
    let mut col2_count = 0;

    for i in 0..col1.len() {
        // Count how many times the number appears in col2
        for j in 0..col2.len() {
            if col1[i] == col2[j] {
                col2_count += 1;
            } else if col2[j] > col1[i] {
                break;
            }
        }
        similarity_score += col1[i] * col2_count;
        col2_count = 0;
    }

    similarity_score
}