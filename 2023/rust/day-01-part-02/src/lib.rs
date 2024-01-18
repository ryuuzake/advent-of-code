use std::{collections::HashMap, fs};

use regex::Regex;

pub fn run(mut args: impl Iterator<Item = String>) {
    let contents = fs::read_to_string("./input.txt").unwrap();

    args.next();

    let method = args.next().unwrap_or("first".to_string());

    if method == "replace" {
        replace_runner(contents);
    } else {
        first_runner(contents);
    }
}

fn first_runner(lines: String) {
    let keys = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let mut sum = 0;

    for line in lines.lines() {
        sum += first(&keys, line);
    }

    println!("{sum}");
}

fn replace_runner(lines: String) {
    let keys = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let sum: i32 = lines.lines().map(|line| replace(&keys, line)).sum();

    println!("{sum}");
}

fn first(keys: &Vec<&str>, line: &str) -> i32 {
    let mut digits_index = keys
        .iter()
        .map(|&k| (k, None::<usize>))
        .collect::<HashMap<_, _>>();

    for key in keys.iter() {
        digits_index.insert(key, line.find(key));
    }

    let tens = digits_index
        .iter()
        .filter(|(_k, v)| v.is_some())
        .min_by(|a, b| a.1.cmp(&b.1))
        .map(|(&k, _v)| k)
        .unwrap_or("");
    let tens = parse_to_int(tens) * 10;

    let ones = digits_index
        .iter()
        .filter(|(_k, v)| v.is_some())
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(&k, _v)| k)
        .unwrap_or("");
    let ones = parse_to_int(ones);

    tens + ones
}

fn regex(line: &str) -> i32 {
    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine).*(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let Some(caps) = re.captures(line) else {
        return 0;
    };
    println!("{:?}", &caps);

    let tens = parse_to_int(&caps[0]) * 10;
    let ones = parse_to_int(&caps[1]);

    tens + ones
}

fn replace(keys: &Vec<&str>, line: &str) -> i32 {
    let mut new_line = String::from(line);

    for key in keys.iter() {
        let new_replace = format!("{}{}{}", &key[..1], parse_to_int(key), &key[1..]);

        new_line = new_line.replace(key, &new_replace);
    }

    let tens = (new_line
        .chars()
        .find(|c| c.is_digit(10))
        .unwrap_or('0')
        .to_digit(10)
        .unwrap()
        * 10) as i32;
    let ones = new_line
        .chars()
        .rev()
        .find(|c| c.is_digit(10))
        .unwrap_or('0')
        .to_digit(10)
        .unwrap() as i32;

    tens + ones
}

fn parse_to_int(value: &str) -> i32 {
    match value {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_twone() {
        let lines = "twone";
        let keys = vec![
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
            "six", "seven", "eight", "nine",
        ];

        assert_eq!(first(&keys, lines), 21);
    }

    #[test]
    fn case_oneight() {
        let lines = "oneight";
        let keys = vec![
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
            "six", "seven", "eight", "nine",
        ];

        assert_eq!(first(&keys, lines), 18);
    }

    #[test]
    fn regex_twone() {
        let line = "twone";

        assert_eq!(regex(line), 21);
    }

    #[test]
    fn replace_twone() {
        let line = "twone";
        let keys = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        assert_eq!(replace(&keys, line), 21);
    }
}
