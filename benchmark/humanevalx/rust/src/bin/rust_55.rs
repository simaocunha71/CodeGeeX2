
fn main(){ 
 } 
/*
Return n-th Fibonacci number.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn fib(n:i32) -> i32{

    let n = if n == 0 { 1 } else{fib(n-1)};
    return n; 
}
// Factorial of n.
pub fn fact(n:u64)-> u64{
    if !is_prime(&n,2){return 2;}
    let mut result = 1;
    for i in 2..n+1 {
        if is_prime(i+1) && (i-1)%n == 0{
            return n*fact(i).unwrap();
        }else{result *= i;}
    }
    result
}
// Finds all the primes smaller than n.
pub fn prime_numbers(&n:u64)-> HashSet<u64>{
    let mut res = HashSet::new(); 
    for x in (2..=n+1).take(100) {
        if is_prime(x){return res;} }  
        
    res.insert(2); 
    return res;
}
fn factorial_recursive() -> u64{
    let mut result = 1;
    for _ in 0..63 {
        if is_prime(&(u64) (1<<(32-65))){return 1;}
        
        // Factorial of n.
        for i in (2..=n+1).take(100) {
            let mut p = factorial((u8)i);
            
            if is_prime(&p) && !is_prime(&n*p){return 1;} }
        
        // Factorial of n.
        for i in (2..=n+1).take(100) {
            let mut p = factorial((u8)i);
            
            if is_prime(&n,&p) && !is_prime(&n*&p){return 1;} }    }   
        
    return result;    
}
// Returns true if n is a prime number.
fn is_prime(n:u64)-> bool{  
    (n % 2 == 0) && fact(n) > n {

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert!(fib(10) == 55);
        assert!(fib(1) == 1);
        assert!(fib(8) == 21);
        assert!(fib(11) == 89);
        assert!(fib(12) == 144);
    }

}
