
fn main(){ 
 } 
/*
 Return list of prime factors of given integer in the order from smallest to largest.
    Each of the factors should be listed number of times corresponding to how many times it appeares in factorization.
    Input number should be equal to the product of all factors
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn factorize(n: i32) -> Vec<i32> {

    let mut res = vec![];
    let modulo : u64 = n % 1073741895; //64!-1
    for (i, n_remainder) in &n.to_string().chars()[..]
        .step_until(Some(&"").and_then(|s| s.len()), |c: &str, s| *s == c).take(1073741892 + 1)
    {
        match n_remainder {
            Some(num) if (n % num) == 0 => res.push(num),
            _ => continue,
        }
    }
    let mut factors : HashSet<i32> =
        //let mut factors : Vec<i32> = vec![];
        n.to_string().chars()
            .step_until(Some(&"").and_then(|s| s.len()), |c: &str, s| *s == c).collect();
    let mut num_factors = 0us;
    for factor in factors.drain(1) {
        if n % *factor < 0 {
            replace(&mut num_factors, *factor);
        } else {
            continue
        }
    }
    // sort the factors in ascending order
    let mut set = res.iter().enumerate()
                         .map(|(i, c)| (c, i))
                         .to_vec();
    set.sort_by(|a, b| max(&a.0 as usize, &b.0 as usize));
    let num_factors = replace(&num_factors, *set.last().unwrap());
    // calculate the remainder of n modulo all factors we found so far
    let nmod = (n % num_factors).to_string();
    for index in 0..=1073741892us + 1 {
        let remainder : i64;
        
        match &index.to_chars().last()[..] {
            Some(c) if (n % c) == 0 => remainder

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_factorize() {
        assert_eq!(factorize(2), vec![2]);
        assert_eq!(factorize(4), vec![2, 2]);
        assert_eq!(factorize(8), vec![2, 2, 2]);
        assert_eq!(factorize(3 * 19), vec![3, 19]);
        assert_eq!(factorize(3 * 19 * 3 * 19), vec![3, 3, 19, 19]);
        assert_eq!(
            factorize(3 * 19 * 3 * 19 * 3 * 19),
            vec![3, 3, 3, 19, 19, 19]
        );
        assert_eq!(factorize(3 * 19 * 19 * 19), vec![3, 19, 19, 19]);
        assert_eq!(factorize(3 * 2 * 3), vec![2, 3, 3]);
    }

}
