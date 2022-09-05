fn get_count(string: &str) -> usize {
  let vowels = [a,e,i,o,u];
  let mut vowels_count: usize = 0;
  
    for c in string.chars() {
      if vowels.contains(&c) {
        vowels_count += 1;
      }
    }
  // Do your magic here
  
  vowels_count
}

fn main() {
  println!("{}", get_count("abracadabra"));
}