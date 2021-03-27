// https://www.codewars.com/kata/55c45be3b2079eccff00010f
// Your order, please

fn order(sentence: &str) -> String {
    let mut s = sentence.split_whitespace().collect::<Vec<&str>>();

    s.sort_by_cached_key(|x| {
        *x.as_bytes()
            .get(x.find(|c: char| c.is_ascii_digit()).unwrap())
            .unwrap()
    });
    s.join(" ")
}

fn main() {
    let s = "is2 Thi1s T4est 3a";
    println!("order({}) = {}", s, order(s));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(order(""), "");
    }
}
