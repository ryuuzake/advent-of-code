use std::{fs, usize};

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();

    let matrix = contents
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;

    for (i, v) in matrix.iter().enumerate() {
        for (j, c) in v.iter().enumerate() {
            if c.is_ascii_punctuation() {
                sum += add_adjacent_punctuation((i, j), &matrix);
            }
        }
    }
}

fn add_adjacent_punctuation(pos: (usize, usize), matrix: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    let adjacent_positions = generate_adjacent_pos(pos);

    for adjacent_position in adjacent_positions {
        if let Some(v) = matrix.get(adjacent_position.0) {
            if let Some(c) = v.get(adjacent_position.1) {
                if c.is_numeric() {
                    sum += get_adjacent_num(pos, &matrix);
                }
            }
        }
    }

    sum
}

fn get_adjacent_num(pos: (usize, usize), matrix: &Vec<Vec<char>>) -> i32 {
    if let Some(v) = matrix.get(0) {

    for i in (pos.1)..=0 {
        for j in pos.1..=
    }
    }

    0
}

fn generate_adjacent_pos(pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut positions: Vec<(usize, usize)> = vec![];

    for i in (pos.0 - 1)..=1 {
        for j in (pos.1 - 1)..=1 {
            positions.push((i, j));
        }
    }

    positions
}
