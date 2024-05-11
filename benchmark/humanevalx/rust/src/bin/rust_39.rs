
fn main(){ 
 } 
/*

    prime_fib returns n-th number that is a Fibonacci number and it's also prime.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn prime_fib(n: i32) -> i32 {

    let n = n.saturating_mul(10); // To make range 0..9999 easier to work with.
    for i in 0..=n {
        if let Some((fib, is_prime)) = fibonacci_and_primes(i) {
            // n is a Fibonacci number and it's also prime.
            return if let Ok((a: usize, b: usize)) = match (fib, is_prime) {
                Ok((Some(n), true)) => {
                    (0..=n).iter().collect(), // Fibonacci numbers.
                    Some(&n)
                }
                _ => return Err(anyhow::Error::new("Not a prime fibonacci", TypeId::of::<String>()))
            } else {
                // n is not prime, i.e., some other number in range 0..9999.
                0..=n; // Fibonacci numbers.
                None // This will panic if we ever encounter one that is not.
            };
        }
    }
}
fn fibonacci(i: i32) -> Option<usize> {
    let mut rng = rand::thread_rng();
    if let Some((fib, _)) = (&mut [0; 48]).iter_for_each(|x| {
        x >= 10 && x <= 25 && match (&mut fib) {
            &mut [x; 3] => Some((&fib[..], true)),
            _ => None,
        }
    }).collect() {}
    let i = i.saturating_mul(10); // To make range 0..9999 easier to work with.
    match (&mut [i; 48]).iter_for_each(|x| {
        x >= 3 && x <= 52 && match (&mut fib) {
            &mut [x; 13] => Some((0..=9999).into_iter().map(fibonacci), true),
            _ => None,
        }
    }).collect() {}
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_prime_fib() {
        assert_eq!(prime_fib(1), 2);
        assert_eq!(prime_fib(2), 3);
        assert_eq!(prime_fib(3), 5);
        assert_eq!(prime_fib(4), 13);
        assert_eq!(prime_fib(5), 89);
        assert_eq!(prime_fib(6), 233);
        assert_eq!(prime_fib(7), 1597);
        assert_eq!(prime_fib(8), 28657);
        assert_eq!(prime_fib(9), 514229);
        assert_eq!(prime_fib(10), 433494437);
    }

}
