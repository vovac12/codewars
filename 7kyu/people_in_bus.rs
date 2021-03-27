// https://www.codewars.com/kata/5648b12ce68d9daa6b000099
// Number of People in the Bus

fn number(bus_stops:&[(i32,i32)]) -> i32 {
    bus_stops.iter().fold(0, |a, b| a + b.0 - b.1)
}

fn main() {
  let arg = [(10,0),(3,5),(5,8)];
  println!("number({:?}) = {}", arg, number(&arg));
}

#[test]
fn returns_expected() {
  assert_eq!(number(&[(10,0),(3,5),(5,8)]), 5);
  assert_eq!(number(&[(3,0),(9,1),(4,10),(12,2),(6,1),(7,10)]), 17);
  assert_eq!(number(&[(3,0),(9,1),(4,8),(12,2),(6,1),(7,8)]), 21);
}