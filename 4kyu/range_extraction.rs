// https://www.codewars.com/kata/51ba717bb08c1cd60f00002f
// Range Extraction


mod solution {
  pub fn range_extraction(a: &[i32]) -> String {
    if a.len() == 0 {
      return "".to_string();
    }
    let mut res = String::new();
    let (mut s, mut e) = (a[0], a[0]);
    for &val in a {
      if val > e + 1 {
        if s == e {
          res += &format!("{},", s);
        } else if s + 1 == e {
          res += &format!("{},{},", s, e);
        } else {
          res += &format!("{}-{},", s, e);
        }
        s = val;
      }
      e = val;
    }
    if s == e {
      res += &format!("{}", s);
    } else if s + 1 == e {
      res += &format!("{},{}", s, e);
    } else {
      res += &format!("{}-{}", s, e);
    }
    res
  }
}

fn main() {
  let a = [1, 2, 4, 6, 7, 8, 9, 12, 13, 14, 16];
  println!("{:?} as ranges: {}", a, solution::range_extraction(&a));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!("-6,-3-1,3-5,7-11,14,15,17-20", solution::range_extraction(&[-6,-3,-2,-1,0,1,3,4,5,7,8,9,10,11,14,15,17,18,19,20]));	
        assert_eq!("-3--1,2,10,15,16,18-20", solution::range_extraction(&[-3,-2,-1,2,10,15,16,18,19,20]));
    }
}
