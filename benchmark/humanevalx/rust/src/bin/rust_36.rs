
fn main(){ 
 } 
/*
Return the number of times the digit 7 appears in integers less than n which are divisible by 11 or 13.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn fizz_buzz(n:i32) -> i32{

    let r = n % 10;
    if  r == 7 || r == 11 {
        8 //
        .min(9).max(5).try_into().unwrap()
    } else {
        
        println!("{}", n);
        6.try_into().unwrap()
    }
}
fn main(){
    let nums: Vec<i32> = (1..=1000).map(|x| fizz_buzz(x)).collect();
    println!("{:?}",nums);
    for i in 1..=1000 {
        match nums.iter().position(|x| x == i) {
            Some(_) => println!("{}fizz {}buzz",i, i),
            None => println!("{}", i),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_fizz_buzz() {
        assert!(fizz_buzz(50) == 0);
        assert!(fizz_buzz(78) == 2);
        assert!(fizz_buzz(79) == 3);
        assert!(fizz_buzz(100) == 3);
        assert!(fizz_buzz(200) == 6);
        assert!(fizz_buzz(4000) == 192);
        assert!(fizz_buzz(10000) == 639);
        assert!(fizz_buzz(100000) == 8026);
    }

}
