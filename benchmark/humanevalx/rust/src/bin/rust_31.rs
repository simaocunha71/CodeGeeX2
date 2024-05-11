
fn main(){ 
 } 
/*
Return true if a given number is prime, and false otherwise.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn is_prime(n:i32) -> bool{

    let n = n.to_string();
//    n.chars().all(|c| c == b'0') || any_primes(n[1..].iter().map(|s| s.parse::<u32>().unwrap())).any() // not used for anything specific yet
        if let Some(prime) = any_primes(&n[1..]){ return prime as u8 == 0 || (prime % n ).0 == 0 } else { false }
    }
fn any_primes(s:Vec<u32>) -> Option<Vec<i32>>{
//    let s = s.iter().map(|n| n.parse::<u8>().unwrap()).collect();
    let s = s[1..].iter().map(|&x| x).collect::<Vec<_>>(); // remove index 0 which is the string
    let mut p = HashSet::new(&s); // use the same set of primes for different strings
        
    if !p.contains(&0){ // prime or not, check every char in the string to count how many times it appears (counting the first occurrence)
        let count:Vec<u8> = p.iter().map(|&prime| prime as u8).collect();
        for v in s[1..].iter_mut(){ 
            if !v.is_ascii() { // non-ascii characters are not included in the set of primes, but we count them anyway!
                let c = v.chars().collect::<Vec<char>>();
                for (i,c) in c.iter_mut().enumerate(){
                    if i < count[0] || (i == count[0] && c[0] != b':'){ // ':' is not counted as prime when it appears first, because of the preceding \n which causes it to be counted again for each subsequent line
                        *count.get_mut(i).unwrap() += 1; // incrementing the count while checking for primality!
                    }
                }
            }
        }
        let mut p = &mut p[1..]; // remove index 0 which is always '0' anyway ;)
        
        return

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(is_prime(6) == false);
        assert!(is_prime(101) == true);
        assert!(is_prime(11) == true);
        assert!(is_prime(13441) == true);
        assert!(is_prime(61) == true);
        assert!(is_prime(4) == false);
        assert!(is_prime(1) == false);
        assert!(is_prime(5) == true);
        assert!(is_prime(11) == true);
        assert!(is_prime(17) == true);
        assert!(is_prime(5 * 17) == false);
        assert!(is_prime(11 * 7) == false);
        assert!(is_prime(13441 * 19) == false);
    }

}
