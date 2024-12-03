use std::fs;
use std::io;
use std::path::Path;
use regex::Regex;

fn main() -> io::Result<()> {
    let file_string = load_file("../Inputs/Day3-1.txt")?;
    let mult_strings = parse_file_string(file_string, Vec::new());
    let cleaned_mult_nums = parse_mult_string_ints(mult_strings);
    let cum_sum = sum_mults(cleaned_mult_nums);
    println!("{}", cum_sum);
    Ok(())
}

fn load_file(file_path: &str) -> io::Result<String> {
    let path = Path::new(file_path);
    let file_string = fs::read_to_string(path)?;
    Ok(file_string)
}

fn parse_file_string(file_string: String, mut mult_strings: Vec<String>) -> Vec<String> {
    let pattern = format!(r"{}(.*?){}", regex::escape("mul("), regex::escape(")"));
    let re = Regex::new(&pattern).unwrap();

    let mut file_substring = file_string.clone();
    while let Some(mult_match) = re.captures(&file_substring) {
        if let Some(cmatch) = mult_match.get(1) {
            let full_match = mult_match.get(0).unwrap().as_str().to_string();
            // Add the current match to the list
            mult_strings.push(cmatch.as_str().to_string());

            // Recursively process the full substring
            // removing the first character to prevent double matching
            mult_strings = parse_file_string(full_match.as_str()[1..].to_string(), mult_strings);

            // Update the remaining text by removing the current match
            let end_index = mult_match.get(0).unwrap().end();
            file_substring.replace_range(0..end_index, "");
        }
    }

    mult_strings
}

fn parse_mult_string_ints(mult_strings: Vec<String>) -> Vec<(i32, i32)> {
    let mut mult_string_ints: Vec<(i32, i32)> = Vec::new();

    for i in 0..mult_strings.len() {
        let cstring = mult_strings[i].clone();
        let cmatch: Vec<&str> = cstring.split(",").collect();
        if cmatch.len() == 2 {
            if let Ok(num1) = cmatch[0].parse::<i32>() {
                if let Ok(num2) = cmatch[1].parse::<i32>() {
                    mult_string_ints.push((num1, num2));
                }
            }
        }

    }

    mult_string_ints
}

fn sum_mults(mults: Vec<(i32, i32)>) -> i32 {
    let mut cum_sum = 0;

    for i in 0..mults.len() {
        cum_sum += mults[i].0 * mults[i].1;
    }

    cum_sum
}