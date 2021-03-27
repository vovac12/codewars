fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    if matrix.len() == 0 || matrix[0].len() == 0 {
        return vec![];
    }
    let mut lb = 0usize;
    let mut rb = matrix.len();
    let mut tb = 0usize;
    let mut bb = matrix.len();
    let mut pos = (0, 0);
    let mut res = vec![];
    while rb >= lb && bb >= tb {
        for i in lb..rb {
            pos.1 = i;
            res.push(matrix[pos.0][pos.1]);
        }
        rb -= 1;
        for i in (tb..bb).skip(1) {
            pos.0 = i;
            res.push(matrix[pos.0][pos.1]);
        }
        bb -= 1;
        for i in (lb..rb).rev() {
            pos.1 = i;
            res.push(matrix[pos.0][pos.1]);
        }
        lb += 1;
        for i in (tb..bb).skip(1).rev() {
            pos.0 = i;
            res.push(matrix[pos.0][pos.1]);
        }
        tb += 1;
    }
    res
}

fn main() {
    println!(
        "{:?}",
        snail(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test1() {
        let square = &[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test2() {
        let square = &[vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test3() {
        let square: &[Vec<i32>; 1] = &[Vec::new()];
        let expected = Vec::new();
        assert_eq!(snail(square), expected, "Failed with empty input");
    }

    #[test]
    fn sample_test4() {
        let square = &[vec![1]];
        let expected = vec![1];
        assert_eq!(snail(square), expected);
    }
}
