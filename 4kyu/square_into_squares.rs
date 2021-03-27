fn decompose(n: i64) -> Option<Vec<i64>> {
    fn decompose_inner(acc: i64, n: i64) -> Option<Vec<i64>> {
        if acc < 0 {
            return None;
        }
        if acc == 0 {
            return Some(vec![]);
        }
        for i in (1..n).rev() {
            match decompose_inner(acc - i.pow(2), i) {
                Some(mut v) if v.iter().filter(|&&x| x == 1).count() <= 1 => {
                    v.push(i);
                    return Some(v);
                }
                _ => {}
            }
        }
        None
    }
    decompose_inner(n.pow(2), n)
}

fn main() {
    println!("f({}) = {:?}", 625, decompose(625));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(n: i64, exp: Option<Vec<i64>>) -> () {
        assert_eq!(decompose(n), exp, "{}", n)
    }

    #[test]
    fn tests_decompose() {
        testing(50, Some(vec![1, 3, 5, 8, 49]));
        testing(44, Some(vec![2, 3, 5, 7, 43]));
        testing(625, Some(vec![2, 5, 8, 34, 624]));
        testing(5, Some(vec![3, 4]));
    }
}
