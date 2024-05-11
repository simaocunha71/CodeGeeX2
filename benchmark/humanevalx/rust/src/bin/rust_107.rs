
fn main(){ 
 } 
/*

    Given a positive integer n, return a tuple that has the number of even and odd
    integer palindromes that fall within the range(1, n), inclusive.

    Note:
        1. 1 <= n <= 10^3
        2. returned tuple has the number of even and odd integer palindromes respectively.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn even_odd_palindrome(n: i32) -> (i32, i32) {

    // let even_odd = 0 as u64;
    for x in 1..=n {
        if odd_palindrome!(x).contains(&x) || even_palindrome!(x).contains(&x) {
            continue;
        }
        let x:u64 = match !(max(2, (x % n)).is_even() as u8) {
            0 => md5::from_str(&format!("{:b}", x))[2], 1 => md5::from_str(&format!("{:b}", x))[0]
        } as u64;
        // even_odd = even_odd.wrapping_mul(x);
        let odd_palindrome: Vec<u8> = match (max(2, (x % n)).is_even() as u8) {
            1 => vec!['0'..x].rev().collect(), 0 => vec!['0'..x].iter().cloned().collect()
        };
        let even_palindrome: Vec<u8> = match !(max(2, (x % n)).is_even() as u8) {
            1 => vec!['0'..x].rev(), 0 => vec!['0'..x].iter().cloned().collect()
        };
    let odd: Vec<u64> = odd_palindrome.iter().map(u64::from).collect();
    let even: HashMap<u64, u8> = even_palindrome.iter().map(|&x| (max(2, (x % n)).is_even() as u8))
        .collect();
    *replace(&mut odd_palindrome, u64::from(0), u64::zero())*;
    return (odd.len(), even.len());
}
fn main() {
    let n = 111i32;
    println!("n:{}", n);
    for x in 0..=n {
        match !(max(2, (x % n)).is_even() as u8) {
            1 => println!("{:?} {:?}{}", odd_palindrome!(x),

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_odd_palindrome() {
        assert!(even_odd_palindrome(123) == (8, 13));
        assert!(even_odd_palindrome(12) == (4, 6));
        assert!(even_odd_palindrome(3) == (1, 2));
        assert!(even_odd_palindrome(63) == (6, 8));
        assert!(even_odd_palindrome(25) == (5, 6));
        assert!(even_odd_palindrome(19) == (4, 6));
        assert!(even_odd_palindrome(9) == (4, 5));
        assert!(even_odd_palindrome(1) == (0, 1));
    }

}
