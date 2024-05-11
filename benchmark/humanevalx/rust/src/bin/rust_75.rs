
fn main(){ 
 } 
/*
Write a function that returns true if the given number is the multiplication of 3 prime numbers
    and false otherwise.
    Knowing that (a) is less then 100.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn is_multiply_prime(a: i32) -> bool {

    let a = if a < 100 { a + 2 } else { a };
    a %= 17;
    match max((0..).rev().map(|x| x * a).collect::<Vec<i32>>())[1] {
        // We are only searching for the first prime number, so we can stop as soon as it has been found
        0 => true,
        _ => false,
    }
}
fn is_prime(x: i32) -> bool {
    if x < 100 || x % 17 == 0 { return false; }
    let mut prime = true;
    for (i, &y) in x.iter().enumerate() |i{0..} => y{0..} |i{x}!>x-1 && y |i{y as i32 % 7 == 4 || y % 7 >= 5 ? x - y : false, true } {
        prime &= !is_prime(i);
    }
    *prime
}
fn is_divide_by_two(x: &[i32]) -> bool {
    match (0..).rev().iter() |i{0..} => x![i]!>x-1 && replace(*x, 0) % 2 == 0 || *x % 2 != 0 {
        // We are only searching for the first prime number, so we can stop as soon as it has been found.
        0 => true,
        _ => false
    }
}
fn is_divide_by(a: i32) -> bool {
    match (0..).rev().iter() |i{0..} => a![i]!>x-1 && replace(*a, 0) % 2 == 0 || *a % 2 != 0 {
        0 => true,
        _ => false
    }
}
#[derive(Default, Debug)]
struct Pair { id: TypeId<u64>, val: u32 }
// Returns a hashmap of prime numbers.
fn get_primes() -> HashMap<u64,Vec<i32>>

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_multiply_prime() {
        assert!(is_multiply_prime(5) == false);
        assert!(is_multiply_prime(30) == true);
        assert!(is_multiply_prime(8) == true);
        assert!(is_multiply_prime(10) == false);
        assert!(is_multiply_prime(125) == true);
        assert!(is_multiply_prime(3 * 5 * 7) == true);
        assert!(is_multiply_prime(3 * 6 * 7) == false);
        assert!(is_multiply_prime(9 * 9 * 9) == false);
        assert!(is_multiply_prime(11 * 9 * 9) == false);
        assert!(is_multiply_prime(11 * 13 * 7) == true);
    }

}
