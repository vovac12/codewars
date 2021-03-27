// https://www.codewars.com/kata/51675d17e0c1bed195000001
// Largest 5 digit number in a series


use std::str;

fn largest_five_digit_number(num: &str) -> u32 {
    num.as_bytes()
       .windows(5)
       .map(|x| str::from_utf8(x)
            .unwrap()
            .parse()
            .unwrap())
       .max()
       .unwrap()
}

fn main() {
    let s = "1234567890";
    println!("largest_five_digit_number({}) = {}", s, largest_five_digit_number(s));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(largest_five_digit_number(&"1234567890"), 67890);
    }
}
