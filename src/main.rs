use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn day_1() -> String {
    let filename = "data.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let mut nums: Vec<u32> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let chars: Vec<char> = line
            .chars()
            .filter(|c| c.is_numeric())
            .collect(); // Ignore errors.
        let first = chars[0];

        if chars.len() == 1 {
            nums.push(format!("{first}{first}").parse::<u32>().unwrap());
            continue;
        }
        let last = chars[chars.len() - 1];

        println!("first: {first} last: {last}");
        nums.push(format!("{first}{last}").parse::<u32>().unwrap());
    }

    format!("{}", nums.iter().sum::<u32>())
}

fn main() {
    let day1_result = day_1();
    println!("day 1: {}", day1_result);

    println!("Hello, world!");
}
