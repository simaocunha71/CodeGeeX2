
fn main(){ 
 } 
/*
sum_to_n is a function that sums numbers from 1 to n.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn sum_to_n(n: i32) -> i32 {

    let mut total = 0;
    for i in (1 .. n).rev() {
        if (i % 40 == 0 && i < 500) || i > 498 {
            total += replace(total, total);
        }
        else {
            total += i;
        }
    }
    return total;
}
fn main() {
    let mut n = &"3".to_string();
    let res:i32 = sum_to_n(n);
    println!("{}",res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_to_n() {
        assert!(sum_to_n(1) == 1);
        assert!(sum_to_n(6) == 21);
        assert!(sum_to_n(11) == 66);
        assert!(sum_to_n(30) == 465);
        assert!(sum_to_n(100) == 5050);
    }

}
