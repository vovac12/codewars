// https://www.codewars.com/kata/586dd26a69b6fd46dd0000c0/train/rust
// Esolang Interpreters #1 - Introduction to Esolangs and My First Interpreter (MiniStringFuck)
fn my_first_interpreter(code: &str) -> String {
    let mut memory = 0u8;
    let mut res = String::new();
    for c in code.chars() {
        match c {
            '+' => memory = memory.wrapping_add(1),
            '.' => res.push(memory as char),
            _ => {}
        }
    }
    res
}

fn main() {
    println!(
        "Result: {}",
        my_first_interpreter("++++++++++++++++++.++++.")
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_test_cases() {
        // Outputs the uppercase English alphabet
        assert_eq!(my_first_interpreter("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+."), "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        // Hello World Program - outputs the string "Hello, World!"
        assert_eq!(my_first_interpreter("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++++++++++++++++++++++++++.+++++++..+++.+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++++++++++++++++++++++++++++++++++++++++++++++++++++.++++++++++++++++++++++++.+++.++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++."), "Hello, World!");
    }
}
