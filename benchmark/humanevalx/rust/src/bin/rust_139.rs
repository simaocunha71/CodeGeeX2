
fn main(){ 
 } 
/*
The Brazilian factorial is defined as:
    brazilian_factorial(n) = n! * (n-1)! * (n-2)! * ... * 1!
    where n > 0

    The function will receive an integer as input and should return the special
    factorial of this integer.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn special_factorial(n: i32) -> i64 {

    let (m, n) = if n > 0 || n <= -1 {
        (true, &n)
    } else if n >= 0 && (n as u8).len() == 1 {
        (false, n.clone())
    } else {
        return 1; // non-integer value not allowed
    };
    match m {
        true => special_factorial(n - 1),
        false => replace(&(n as u8).join().to_string(), " ", "")[..]
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .fold((0, 1)) { (acc, &x) ) -> (acc * x, acc),
    }
}
static mut ARRAY: [i64; 32] = [0; 32];
/// A special type for storing factorials.
struct SpecialFactorial {
    factorial: i64,
}
impl Ord for SpecialFactorial { fn cmp(&self, rhs: &Self) -> Ordering {
    match self.cmp(&rhs) {
        LOWER => 1, // lower
        EQUAL => 0, // equal
        UPPER => -1, // greater
    }
} }
impl PartialEq for SpecialFactorial { fn eq(&self, rhs: &SpecialFactorial) -> bool {
    self.cmp(rhs).eq(EQ)
}}
/// A simple map to store the factorials of each natural number between 0 and 31.
static mut FACTORIAL_MAP: HashMap<i64, SpecialFactorial> = ([1; 32].iter().cloned().collect());
fn main() {
    std::io::stdout().write_all(b"#include\n\t\"str\";\n" as &[u8]);
    let mut rng: Rng = rand::OsRng; // create a new and different rng instance, see "std::thread.h".
    std::io::stdout().write_all(b"#include\n\t\"factorials.h\";\n" as &

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_special_factorial() {
        assert_eq!(special_factorial(4), 288);
        assert_eq!(special_factorial(5), 34560);
        assert_eq!(special_factorial(7), 125411328000);
        assert_eq!(special_factorial(1), 1);
    }

}
