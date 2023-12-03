use env_logger::{Builder, Target};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[allow(unused_mut)]
#[allow(unused_variables)]
#[allow(unused_assignments)]

fn process(values: &mut str) {
    let mut sum = 0;
    for line in values.lines() {
        let chars: Vec<char> = line.chars().collect();
        let mut first: i32 = -1;
        let mut last: i32 = -1;

        for (i, c) in chars.iter().enumerate() {
            // if c is a digit, then set first to i
            if c.is_ascii_digit() {
                first = i as i32;
                break;
            }
        }

        for (i, c) in chars.iter().enumerate().rev() {
            // if c is a digit, then set last to i
            if c.is_ascii_digit() {
                last = i as i32;
                break;
            }
        }

        if first == -1 || last == -1 {
            log::error!("NO DIGIT FOUND!");
        }

        log::debug!("line: {}", line);
        log::debug!("first: {}, last: {}", first, last);
        let first_char = chars[first as usize];
        let last_char = chars[last as usize];
        let joined = format!("{}{}", first_char, last_char);
        // let first_value: i32 = chars[first as usize].to_digit(10).unwrap() as i32;
        // let last_value: i32 = chars[last as usize].to_digit(10).unwrap() as i32;
        // let mut first_value: i32 = chars[first].to_digit(10).unwrap();
        // let mut last_value: i32 = chars[last].to_digit(10).unwrap();
        log::debug!("{} + {}: {}", first_char, last_char, joined);

        sum += joined.parse::<i32>().unwrap();
    }

    println!("sum: {}", sum);
}

fn main() {
    // Builder::new().target(Target::Stdout).init();
    env_logger::init();

    // Create a path to the desired file
    let path = Path::new("../data/values.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => process(&mut s),
    }
}
