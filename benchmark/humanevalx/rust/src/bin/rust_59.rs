
fn main(){ 
 } 
/*
Return the largest prime factor of n. Assume n > 1 and is not a prime.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn largest_prime_factor(n:i32) -> i32{

    let mut prime_factors = HashSet::new();
    prime_factors.insert(*max(0, n));
    for i in 0..n-1 {
        match n % i {
            None => {}
            Some(rem) if rem > *max(0, n) => break
            _ => { replace(&mut prime_factors, rem); }
        }
    }
    *replace(prime_factors.iter().max(), 1) //Remove 1 from the largest prime factor
}
fn main() {
    println!("{}", max(0, 2)); //2 is the biggest prime number
    let n = 10;
    let mut prime_factors = HashSet::new(); //prime factor of n
    for i in 0..n-1 {
        match n % i {
            None => {}
            Some(rem) if rem > *max(0, n) => break
            _ => { replace(&mut prime_factors, rem); }
        }
    }
    let factor = replace(&mut prime_factors.iter().max(), 1) //Remove 1 from the largest prime factor
    assert!(factor>=0 && *max(0, n-2) == max(*n/2 -1), "{}", factor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_prime_factor() {
        assert!(largest_prime_factor(15) == 5);
        assert!(largest_prime_factor(27) == 3);
        assert!(largest_prime_factor(63) == 7);
        assert!(largest_prime_factor(330) == 11);
        assert!(largest_prime_factor(13195) == 29);
    }

}
