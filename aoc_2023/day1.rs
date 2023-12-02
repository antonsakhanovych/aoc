use std::{env, fs, path::Path, process};

const NUMBER_MAP: [(&'static str, usize); 20] = [
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn parse_args() -> String {
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("ERROR: Wrong number of arguments.");
        eprintln!("Usage: ./day1 input.txt");
        process::exit(1);
    }
    return args.remove(1);
}

fn process_line_1(line: &str) -> usize {
    let all_digits: String = line.chars().filter(|c| c.is_digit(10)).collect();
    if all_digits.is_empty() {
        return 0;
    }
    let first_digit = all_digits.chars().next().unwrap();
    let second_digit = all_digits.chars().last().unwrap();
    let digit_str = format!("{first_digit}{second_digit}");
    return digit_str.parse().unwrap_or(0);
}

fn process_line_2(line: &str) -> usize {
    let mut numbers = (0..line.len()).filter_map(|i| {
        NUMBER_MAP
            .iter()
            .find(|(num, _val)| line[i..].starts_with(num))
            .map(|(_num, val)| val)
    });
    let first = numbers.next().unwrap();
    if let Some(last) = numbers.last() {
        (first * 10) + last
    } else {
        first * 11
    }
}

fn calculate(content: &str, f: impl Fn(&str) -> usize) -> usize {
    let mut sum = 0;

    for line in content.lines() {
        sum += f(line);
    }
    sum
}

fn main() {
    let file_path_str = parse_args();
    let file_path = Path::new(&file_path_str);

    if !file_path.exists() {
        eprintln!("ERROR: Invalid path provided");
        process::exit(1);
    }
    let file_content: String = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("ERROR: couldn't read the file because of: {}", e);
            process::exit(1);
        }
    };
    println!("Part1: {}", calculate(&file_content, process_line_1));
    println!("Part2: {}", calculate(&file_content, process_line_2));
}
