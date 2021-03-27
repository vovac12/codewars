/// f(1) = 1 [1]
/// f(2) = 2 [2] [1, 1] u(2, 2) = 1, u(2, 1) = 1
/// f(3) = 3 [3] [2, 1] [1, 1, 1] u(3, 3) = 1, u(3, 2) = 0, u(3, 1) = 2
/// f(4) = u(4, 4) + u(4, 3) + u(4, 2) + u(4, 1)
/// f(4) = 5 [4] [3 1] [2 2] [2 1 1] [1 1 1 1]
/// u(4, 4) = 1, u(4, 3) = 0, u(4, 2) = 1, u(4, 1) = 2
/// f(5) =   [5] [4 1] [3 2] [2 2 1] [3 1 1] [2 1 1 1] [1 1 1 1 1]
/// f(6) =   [6] [5 1] [4 2] [4 1 1] [3 3] [3 2 1] [3 1 1 1] [2 2 2] [2 2 1 1] [2 1 1 1 1] [1x6]
/// | 1 - - - - - |
/// | 1 1 - - - - |
/// | 2 0 1 - - - |
/// | 3 1 0 1 - - |
/// | 5 1 0 0 1 - |
/// | 7 2 1 0 0 1 |
/// f(n) = 1 + f(n-1) +
/// | - - - - - - |
/// | - - - - - - |
/// | - - - - - - |
/// | - - - - - - |
/// | - - - - - - |
/// | - - - - - - |
fn partitions(n: isize) -> isize {
    let mut m = vec![vec![0; n as usize]; n as usize];
    for i in 0..n as usize {
        m[i][i] = 1;
        for j in 0..i {
            let k = i - j - 1;
            for l in j..=k {
                m[i][j] += m[k][l];
            }
        }
    }
    m[n as usize - 1].iter().sum()
}

fn main() {
    println!("f({}) = {}", 5, partitions(6));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_01() {
        assert_eq!(partitions(1), 1);
    }

    #[test]
    fn basic_test_05() {
        assert_eq!(partitions(5), 7);
    }

    #[test]
    fn basic_test_10() {
        assert_eq!(partitions(10), 42);
    }

    #[test]
    fn basic_test_25() {
        assert_eq!(partitions(25), 1958);
    }
}
