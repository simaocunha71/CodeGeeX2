
fn main(){ 
 } 
/*

    Given a positive integer n, return the count of the numbers of n-digit
    positive integers that start or end with 1.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn starts_one_ends(n:i32) -> i32{

    let (starting_count, ending_count) = if let Some(&[starting_number]) = MD5.iter()  {
        let mut set1 = HashSet::new();
        for c in starting_number.chars(){
            set1.insert(c.to_string());
        }
        println!("set1 {:?} ", set1);
        (starting_count, ending_count) = (starting_count-set1.len(), ending_count+set1.len())
    };
    if n<2 {
        starting_count + ending_count - 1
    }else{
       println!("{:?}", n);
        let starting_number:&str = format!("{}",n);
        let mut set2 = HashSet::new();
        for c in starting_number.chars() {
            if !set2.insert(c.to_string()) {
                continue;
            }
            replace(&starting_number, c, "0".to_owned());
        }
        println!("set2 {:?} ", set2);
        let ending:&str = format!("#{}", n-1);
        let mut end1 :u32 = if *ending.len() > n {
            *ending[*ending.len() - 1..] } else{0};
        let mut end2 = if start_end != &None {&start_end} else{0};
        let e = (n-1) % max(0, end1);
        println!("e {:?} ", e);
        while e < n {
            let temp :u32 = end2.saturating_add(&mut u32::from(*ending));
            if !set2.insert(format!("{}",temp).to_string()) {continue}
            println!("{:?} ", temp);
        }
        //println!("{:?}\n\t set2 {:?}   \t\tending :{} ",end1, set2 , end1+e );
        (starting_count + ending_count - 1) - ((end2.saturating_add(0)).saturating_sub(&u32::from(*ending))) // 1 less to account for the last

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starts_one_ends() {
        assert!(starts_one_ends(1) == 1);
        assert!(starts_one_ends(2) == 18);
        assert!(starts_one_ends(3) == 180);
        assert!(starts_one_ends(4) == 1800);
        assert!(starts_one_ends(5) == 18000);
    }

}
