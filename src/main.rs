use std::io;

fn stanton_measure(arr: &[u32]) -> u32 {
    let n = arr.iter().filter(|&&x| x == 1).count() as u32;

    let j = arr.iter().filter(|&&x| x == n).count() as u32;
    println!("{}", j);
    return j as u32;

}

fn main() {
    
    fn dotest(arr: &[u32], expected: u32) {
        let actual = stanton_measure(arr);
        assert!(actual == expected, "With arr = {arr:?}\nExpected {expected} but got {actual}")
    }

    fn fixed_tests() {
        dotest(&[1, 4, 1, 2, 11, 2, 3, 1], 1);
    }
    fixed_tests();
}