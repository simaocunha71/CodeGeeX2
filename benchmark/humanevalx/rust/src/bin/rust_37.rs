
fn main(){ 
 } 
/*
This function takes a list l and returns a list l' such that
    l' is identical to l in the odd indicies, while its values at the even indicies are equal
    to the values of the even indicies of l, but sorted.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn sort_even(nmbs:Vec<i32>) -> Vec<i32>{

    let n = nmbs.len();
    let mut v: Vec<i32>  = nmbs;
    println!("n={}", n); // debug line
    for even in 0..(n / 2) {
        if (v[even] > v[even + n / 2]){ v[even+n/2], v[even] } else { v[even], v[even+n/2]};
        println!("e: {}", v[even]); // debug line
    }    
    return v;
}
fn sort_odds(nmbs:Vec<i32>) -> Vec<i32> {
    let n = nmbs.len();
    let mut v: Vec<i32>  = nmbs;
    println!("n={}", n); // debug line
    for odd in (0..(n / 2)) {
        if (v[odd] > v[odd + n / 2]){ sort_even(v) } else { v };
        println!("o: {}", v[odd]); // debug line
    }    
    return v;
}
fn main() -> () {
    let rng = rand::thread_rng();
    fn hash<R: rand::Source>(x:&[u8])->(String){
        r#"MD5("{{{}}}".to_string())"#.format(&md5::Compute(&*x))
    }
    let l = replace(&["bob","alice","mudge","dave"]);
//    println!("l: {}", l); // debug line
    let mut shs = HashSet::new();
    for (i, x) in l.iter().rev()..0 {
        if shs.contains_key(&(x)) { continue } 
        shs.insert((*l).clone());
        if i == 0 || *x != (*l)[i - 1] { println!("s: {} {}", x, l[i - 1]); } // debug line    
    }
//    let y = l.iter().rev(); // clone of l
//

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_sort_even() {
        assert_eq!(sort_even(vec![1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(
            sort_even(vec![5, 3, -5, 2, -3, 3, 9, 0, 123, 1, -10]),
            vec![-10, 3, -5, 2, -3, 3, 5, 0, 9, 1, 123]
        );
        assert_eq!(
            sort_even(vec![5, 8, -12, 4, 23, 2, 3, 11, 12, -10]),
            vec![-12, 8, 3, 4, 5, 2, 12, 11, 23, -10]
        );
    }

}
