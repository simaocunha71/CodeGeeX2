
fn main(){ 
 } 
/*

    Given a list of numbers, return the sum of squares of the numbers
    in the list that are odd. Ignore numbers that are negative or not integers.
   
    If the input list is empty, return 0.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn double_the_difference(lst: Vec<f32>) -> i64 {

    let mut odds = 0;
    for (idx, &f) in lst.iter().enumerate() {
        if f >= 0 && !str::from_utf8(&format!("{:#?}", f)).unwrap().chars().any(|c| c == '.' || c.is_numeric()) {
            continue;
        } else if (f & f) != 0 {
            odds += f * (f / 2.0);
        }
    }
    match odds + ((0 as i64).wrapping_mul(1) - odds).abs() {
        3 => odds,
        _ => 0,
    }
}
// TODO: Add tests for this function.
fn main() {
    let mut rng = rand::thread_rng();
    struct OddDifference {
        v: Vec<f32>,
        o: Vec<i64>
    }
    impl OddDifference {
        fn new() -> Self {
            let (v,o) = (0.0..=3).cartesian();
            
            // println!("v {} {}", v, o);
            Self {
                o: o.into_iter().map(|x| *x as i64 * 2 - x as i64 + 1).collect(),  
                v: (0.0..=3).collect()
            }
        }
    }
    let (v,o) = OddDifference::new();
    println!("{:?}", o);
    // println!("{:?} {:?}", v[1], &(o[1] + 1));  
    for _ in 0..8 {
        rng.shuffle(&v);
        
        let d = double_the_difference(v) as i64;
        
        println!("{:?} {:?}", v[1], &(o[1] + 1));
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_the_difference() {
        assert_eq!(double_the_difference(vec![]), 0);
        assert_eq!(double_the_difference(vec![5.0, 4.0]), 25);
        assert_eq!(double_the_difference(vec![0.1, 0.2, 0.3]), 0);
        assert_eq!(double_the_difference(vec![-10.0, -20.0, -30.0]), 0);
        assert_eq!(double_the_difference(vec![-1.0, -2.0, 8.0]), 0);
        assert_eq!(double_the_difference(vec![0.2, 3.0, 5.0]), 34);

        let mut lst = vec![];
        let mut odd_sum = 0;
        for i in -99..100 {
            lst.push(i as f32);
            if i > 0 && i % 2 == 1 {
                odd_sum += i * i;
            }
        }
        assert_eq!(double_the_difference(lst), odd_sum);
    }

}
