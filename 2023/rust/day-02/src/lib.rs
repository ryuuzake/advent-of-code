use regex::Regex;
use std::fs;

pub fn run(mut args: impl Iterator<Item = String>) {
    let contents = fs::read_to_string("./input.txt").unwrap();

    args.next();

    regex_runner(contents);
}

fn regex_runner(lines: String) {
    let sum: i32 = lines.lines().map(|line| regex_game(line)).sum();

    println!("{sum}");
}

fn regex_game(line: &str) -> i32 {
    let game_num = get_game_num(line);

    if let Some((_, rest)) = line.split_once(':') {
        if rest.split(";").all(|line| regex_each_game(line)) {
            return game_num;
        }
    }

    0
}

fn get_game_num(line: &str) -> i32 {
    let game_num_re = Regex::new(r"Game (\d+):").expect("Expect Valid Regex");

    let game_num_cap = game_num_re
        .captures(&line)
        .expect("Expect Game Number in Regex");
    let game_num = &game_num_cap[1];
    let game_num: i32 = game_num.parse().expect("Expect Game Number");

    game_num
}

const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;

fn regex_each_game(line: &str) -> bool {
    let red_re = Regex::new(r"(\d+) red").expect("Expect valid Regex");
    let green_re = Regex::new(r"(\d+) green").expect("Expect valid Regex");
    let blue_re = Regex::new(r"(\d+) blue").expect("Expect valid Regex");

    if let Some(caps) = red_re.captures(line) {
        let num = &caps[1];
        let num: i32 = num.parse().expect("Expect Number");

        if num > RED {
            return false;
        }
    }

    if let Some(caps) = green_re.captures(line) {
        let num = &caps[1];
        let num: i32 = num.parse().expect("Expect Number");

        if num > GREEN {
            return false;
        }
    }

    if let Some(caps) = blue_re.captures(line) {
        let num = &caps[1];
        let num: i32 = num.parse().expect("Expect Number");

        if num > BLUE {
            return false;
        }
    }

    return true;
}
