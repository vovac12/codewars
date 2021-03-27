// https://www.codewars.com/kata/545a4c5a61aa4c6916000755
// Find the middle element

fn gimme(input_array: [i32; 3]) -> usize {
    let mut arr: Vec<_> = input_array.iter().enumerate().collect();
    arr.sort_by_key(|x| x.1);
    arr[1].0
}

fn main() {
    println!("Index of the median [4, 3, 6]: {}", gimme([4, 3, 6]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gimme() {
        assert_eq!(gimme([2, 3, 1]), 0);
        assert_eq!(gimme([-2, -3, -1]), 0);
        assert_eq!(gimme([5, 10, 14]), 1);
    }
}
