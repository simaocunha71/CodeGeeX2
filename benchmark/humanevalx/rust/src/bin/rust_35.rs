
fn main(){ 
 } 
/*
Return maximum element in the list.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn maximum(nmbs:Vec<i32>) -> i32{

    let mut max = 0;
    for nmb in &nmbs[..] {
        if max < nmb{
            max = nmb }
    }
    max
}
fn main() {
    println!("{}", maximum(vec![1,2,3,4]));
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_maximum() {
        assert!(maximum(vec![1, 2, 3]) == 3);
        assert!(maximum(vec![5, 3, -5, 2, -3, 3, 9, 0, 124, 1, -10]) == 124);
    }


}
