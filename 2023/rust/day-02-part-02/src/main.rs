use std::{env, fs};

use regex::Regex;

fn main() {
    let mut args = env::args();

    let contents = fs::read_to_string("./input.txt").unwrap();

    args.next();

    regex_runner(contents);
}

struct RGB {
    red: i32,
    green: i32,
    blue: i32,
}

fn regex_runner(lines: String) {
    let sum: i32 = lines.lines().map(|line| regex_game(line)).sum();

    println!("{sum}");
}

fn regex_game(line: &str) -> i32 {
    if let Some((_, rest)) = line.split_once(':') {
        let game_lines: Vec<&str> = rest.split(';').collect();
        let mut rgb = RGB {
            red: 0,
            green: 0,
            blue: 0,
        };

        for line in game_lines {
            regex_each_game(line, &mut rgb);
        }

        return rgb.red * rgb.green * rgb.blue;
    }

    0
}

fn regex_each_game(line: &str, min_rgb: &mut RGB) {
    let red_re = Regex::new(r"(\d+) red").expect("Expect valid Regex");
    let green_re = Regex::new(r"(\d+) green").expect("Expect valid Regex");
    let blue_re = Regex::new(r"(\d+) blue").expect("Expect valid Regex");

    if let Some(caps) = red_re.captures(line) {
        let num = &caps[1];
        let num: i32 = num.parse().expect("Expect Number");

        if num > min_rgb.red {
            min_rgb.red = num;
        }
    }

    if let Some(caps) = green_re.captures(line) {
        let num = &caps[1];
        let num: i32 = num.parse().expect("Expect Number");

        if num > min_rgb.green {
            min_rgb.green = num;
        }
    }

    if let Some(caps) = blue_re.captures(line) {
        let num = &caps[1];
        let num: i32 = num.parse().expect("Expect Number");

        if num > min_rgb.blue {
            min_rgb.blue = num;
        }
    }
}
