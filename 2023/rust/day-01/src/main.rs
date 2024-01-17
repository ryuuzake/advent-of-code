use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt");

    let sum: u32 = contents
        .unwrap()
        .lines()
        .map(|line| {
            let tens = line
                .chars()
                .find(|c| c.is_digit(10))
                .unwrap_or('0')
                .to_digit(10)
                .unwrap()
                * 10;
            let ones = line
                .chars()
                .rev()
                .find(|c| c.is_digit(10))
                .unwrap_or('0')
                .to_digit(10)
                .unwrap();

            tens + ones
        })
        .sum();

    println!("{sum}");
}
