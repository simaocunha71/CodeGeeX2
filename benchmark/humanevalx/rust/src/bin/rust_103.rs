
fn main(){ 
 } 
/*
You are given two positive integers n and m, and your task is to compute the
    average of the integers from n through m (including n and m). 
    Round the answer to the nearest integer and convert that to binary.
    If n is greater than m, return -1.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn rounded_avg(n:i32, m:i32) -> String{

    let n_iter = Iter::new(n..=m); 
    // Create a collection of the average of numbers from 1 to n in range [0;m].
    let mut avg = 0 as u64; 
    for x in &n_iter {
        *avg += (x.to_string().parse::<f64>().unwrap()).to_int32() as u64;
    }
    String::from_utf8(format!("{:010}", avg))
}
fn gen_rounded_avgs(){
    // Generate random positive integers between 1 and 10,000.
    let mut rng = rand::thread_rng();
    if rng.gen::<f64>() < 0.3 {  
        return;
    }
    println!("{}", rounded_avg(500_000, 999_998));
    gen_rounded_avgs();
}
#[derive(Copy, Clone)] // To avoid a compiler error of `TypeId`s being different.
struct RndAvg<T> { _id: TypeId }
fn main() { 
    print!("{}", gen_rounded_avgs());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rounded_avg() {
        assert!(rounded_avg(1, 5) == "11");
        assert!(rounded_avg(7, 13) == "1010");
        assert!(rounded_avg(964, 977) == "1111001010");
        assert!(rounded_avg(996, 997) == "1111100100");
        assert!(rounded_avg(560, 851) == "1011000001");
        assert!(rounded_avg(185, 546) == "101101101");
        assert!(rounded_avg(362, 496) == "110101101");
        assert!(rounded_avg(350, 902) == "1001110010");
        assert!(rounded_avg(197, 233) == "11010111");
        assert!(rounded_avg(7, 5) == "-1");
        assert!(rounded_avg(5, 1) == "-1");
        assert!(rounded_avg(5, 5) == "101");
    }

}
