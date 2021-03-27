// https://www.codewars.com/kata/554b4ac871d6813a03000035
// Highest and Lowest

use std::i32;

fn high_and_low(numbers: &str) -> String {
    let (max, min) = numbers
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .fold((i32::MIN, i32::MAX), |x, y| (x.0.max(y), x.1.min(y)));
    format!("{} {}", max, min)
}

fn main() {
    let s = "5, 3, 2, 5, 6, 7";
    println!("high_and_low({}) = {}", s, high_and_low(s));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_test() {
        assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
    }
}
