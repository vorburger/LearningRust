// https://google.github.io/comprehensive-rust/exercises/day-2/luhn.html

// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    let mut valid_cc_number = String::new();
    for char in cc_number.chars() {
        if char.is_whitespace() {
            continue;
        }
        if !char.is_numeric() {
            return false;
        }
        valid_cc_number.push(char);
    }
    if valid_cc_number.len() < 2 {
        return false;
    }

    // https://stackoverflow.com/a/38083610
    let reverse = valid_cc_number.chars().rev().collect::<String>();

    let mut sum = 0;
    let mut double = false;
    for char in reverse.chars() {
        let mut n = char.to_digit(10).unwrap();
        if double {
            n *= 2;
        }
        if n > 9 {
            match n {
                10 => n = 1,
                11 => n = 2,
                12 => n = 3,
                13 => n = 4,
                14 => n = 5,
                15 => n = 6,
                16 => n = 7,
                17 => n = 8,
                18 => n = 9,
                _ => (),
            }
        }
        sum += n;
        double = !double;
    }
    sum.to_string().ends_with('0')
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}
