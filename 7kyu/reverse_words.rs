// https://www.codewars.com/kata/5259b20d6021e9e14c0010d4
// Reverse words

fn reverse_words(str: &str) -> String {
    let mut res = String::new();
    str.split(" ")
        .map(|x| {
            res.extend(x.chars().rev().chain(Some(' ')));
        })
        .count();
    res.pop();
    res
}

fn main() {
    let s = "big apple";
    println!("Reversed '{}' = '{}'", s, reverse_words(s));
}

// Rust tests
#[test]
fn sample_test() {
    assert_eq!(
        reverse_words("The quick brown fox jumps over the lazy dog."),
        "ehT kciuq nworb xof spmuj revo eht yzal .god"
    );
    assert_eq!(reverse_words("apple"), "elppa");
    assert_eq!(reverse_words("a b c d"), "a b c d");
    assert_eq!(
        reverse_words("double  spaced  words"),
        "elbuod  decaps  sdrow"
    );
}
