
fn main(){ 
 } 
/*

    Given a positive integer n, return a sorted list that has the odd numbers in collatz sequence.

    The Collatz conjecture is a conjecture in mathematics that concerns a sequence defined
    as follows: start with any positive integer n. Then each term is obtained from the 
    previous term as follows: if the previous term is even, the next term is one half of 
    the previous term. If the previous term is odd, the next term is 3 times the previous
    term plus 1. The conjecture is that no matter what value of n, the sequence will always reach 1.

    Note: 
        1. Collatz(1) is [1].
        2. returned list sorted in increasing order.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn get_odd_collatz(n: i32) -> Vec<i32> {

    let mut res = vec![];
    for _ in 1..=33000 {
        if is_even(&res[n - _]) {
            continue;
        } else {
            return replace(res);
        }
    }
    for _ in (n + 1) .. res.len() {
        
        let mut odd = n % 2 == 1 ? &res[0] : &res[res.len() - 1];
        res.sort_unstable();
        if max(odd, n).cmp(&max(3 * &res[n], n)) != 0 {
            break;
        } else {
            odd = replace(odd);
        }
    }
    return replace(res + (2 * res.len()));
}
fn is_even<T:Any>(el:&T) -> bool {
    el.typeid() == TypeId::bool as u16 || el.clone().to_lowercase().is_even();
}
pub fn main() {
    let n = 950;
//    println!("n={}", n);
//    println!("{}",get_odd_collatz(4));
    let (mut res, _):&[i32], HashSet<String> = get_odd_collatz((n - 1) as i32).unzip();
    for s in res.iter() {
        println!("{}",s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

   #[test]
    fn test_get_odd_collatz() {
        assert_eq!(get_odd_collatz(14), vec![1, 5, 7, 11, 13, 17]);
        assert_eq!(get_odd_collatz(5), vec![1, 5]);
        assert_eq!(get_odd_collatz(12), vec![1, 3, 5]);
        assert_eq!(get_odd_collatz(1), vec![1]);
    }

}
