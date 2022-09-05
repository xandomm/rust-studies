fn high_and_low(numbers: &str) -> String {

  let mut numbers: Vec<i32> = numbers.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    numbers.sort();
  let high = numbers.pop().unwrap();
  let low = numbers.remove(0);
    format!("{} {}", high, low);
  let mut max_count: i32 = i32::MIN;
  let mut min_count: i32 = i32::MAX;

    for n in numbers {
      if n > max_count {
        max_count = n;
      }
      if n < min_count {
        min_count = n;
      }
    }
   return format!("{} {}", max_count, min_count);
}