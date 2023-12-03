#![allow(unused)]
#![allow(dead_code)]

use log::{debug, error, info, trace, warn};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn find_substring(haystack: &str, needle: &str) -> Option<usize> {
    haystack.find(needle)
    // if let Some(index) = haystack.find(needle) {
    //     Some(index)
    // } else {
    //     None
    // }
}

fn find_all_literals(line: &str) -> Vec<(usize, &str, usize)> {
    debug!("searching for literals line: {}", line);
    let literals = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut result: Vec<(usize, &str, usize)> = Vec::new();
    for (i, l) in literals.iter().enumerate() {
        if let Some(index) = line.find(l) {
            debug!("Substring [{}] found at index: {}", l, index);
            result.push((index, l, i))
        }
    }
    debug!("result: {:?}", result);
    result
}

fn find_all_digits(line: &str) -> Vec<(usize, usize)> {
    debug!("searching for digits in line: {}", line);
    let mut result: Vec<(usize, usize)> = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        if c.is_ascii_digit() {
            debug!("Digit [{}] found at index: {}", c, i);
            result.push((i, c.to_string().parse::<usize>().unwrap()));
        }
    }
    debug!("result: {:?}", result);
    result
}

fn process(values: &mut str) -> u32 {
    let literals = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut sum: u32 = 0;

    for line in values.lines() {
        let chars: Vec<char> = line.chars().collect();

        let digits = find_all_digits(line);
        let literals = find_all_literals(line);

        let first = if !literals.is_empty() {
            if digits[0].0 < literals[0].0 {
                digits[0].1
            } else {
                literals[0].2
            }
        } else {
            digits[0].1
        };

        let last = if !literals.is_empty() {
            if digits.last().unwrap().0 > literals.last().unwrap().0 {
                digits.last().unwrap().1
            } else {
                literals.last().unwrap().2
            }
        } else {
            digits.last().unwrap().1
        };

        let joined = format!("{}{}", first, last);

        debug!("line: {}", line);
        debug!("first: {}, last: {}", first, last);
        debug!("{} + {}: {}", first, last, joined);
        debug!("-----------------------------------------------------------------------------------------------");

        sum += match joined.parse::<u32>() {
            Err(why) => panic!("couldn't parse {}: {}", joined, why),
            Ok(parsed) => parsed,
        };
    }

    sum
}

fn main() {
    env_logger::init();

    // debug!("Result: {:?}", find_all_literals("sixrrmlkptmc18zhvninek"));
    // debug!("Result: {:?}", find_all_digits("sixrrmlkptmc18zhvninek"));
    // return;

    let path = Path::new("../data/values.txt");
    let display = path.display();

    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    let sum = match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => process(&mut s),
    };

    print!("sum = {}", sum);
}
