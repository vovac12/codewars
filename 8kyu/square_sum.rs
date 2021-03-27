// https://www.codewars.com/kata/515e271a311df0350d00000f
// Square(n) Sum

fn square_sum(vec: Vec<i32>) -> i32 {
    vec.iter().map(|x| x*x).sum()
}

fn main() {
    println!("Square sum of [1, 2, 3]: {}", square_sum(vec![1, 2, 3]));
}

#[test]
fn returns_expected() {
    assert_eq!(square_sum(vec![1, 2]), 5);
    assert_eq!(square_sum(vec![-1, -2]), 5);
    assert_eq!(square_sum(vec![5, 3, 4]), 50);
}