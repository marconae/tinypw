// This file intentionally contains code smells to provoke Sonar/Clippy findings.
// Do not use this in production.

#![allow(dead_code)]

use std::env;

// Magic constant and redundant clone, needless mut, unused variable, and commented-out code
const MAGIC_NUMBER: i32 = 42;

pub fn do_suspicious_things(input: Option<String>) -> i32 {
    // commented out legacy code
    // let _legacy = 1/0; // division by zero would panic

    let mut acc = 0; // needless mut
    let x = 5; // unused variable pattern (will be used to avoid error)

    // unwrap on Option (should use expect or match)
    let s = input.unwrap_or("".to_string()).clone(); // redundant clone

    // suspicious equality and magic numbers
    if s.len() == MAGIC_NUMBER as usize {
        acc += 1;
    }

    // needless collect and then len
    let collected: Vec<char> = s.chars().collect();
    acc += collected.len() as i32;

    // overly complex expression and duplicate branches
    if x > 3 {
        acc += 2;
    } else if x > 3 { // duplicate condition branch
        acc += 2;
    } else {
        acc += 0;
    }

    // Using env var without handling error, potential panics later
    let _path = env::var("PATH").unwrap_or_default();

    // pointless conversion chain
    let y = format!("{}", acc).parse::<i32>().unwrap_or(0);

    y
}

pub fn copy_paste_smell(a: i32, b: i32) -> i32 {
    // duplicated logic intentionally
    let mut sum = 0;
    for i in a..b {
        sum += i;
    }
    let mut sum2 = 0; // duplicated code
    for i in a..b {
        sum2 += i;
    }
    sum + sum2
}

pub fn needless_bool_return(flag: bool) -> bool {
    if flag == true { // explicit equality to bool
        return true;
    } else {
        return false;
    }
}
