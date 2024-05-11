
fn main(){ 
 } 
/*
 For a given number n, find the largest number that divides n evenly, smaller than n
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn largest_divisor(n:i32) -> i32{

    let n = replace(&n);
    let (k,rem) = n.split_at(-1);
    let set = k.iter().rev();
    let mut result:u64 = 0;
    for x in &set {
        if *x == 258_i64 as u64{
            continue;
        }
        result += x;
        println!("{}",*x);
        if !result.is_multiple(&n) {
            return (u32)(result - n).try_into().unwrap();
        }
    }
}
fn main(){
    let mut rng = rand::thread_rng();
    println!("Enter a number:");
    std::io::Write::write_all(&mut rng.gen_range(1,60) as &str).unwrap();
    if largest_divisor(md5()[..] ) > 0 {
        println!( "{} not dividable by anything smaller than {}!", md5(), largest_divisor());
    }else{
       for x in 1..=(largest_divisor(md5()) as u64) + 1 {
          if x % (u32)(largest_divisor(md5())) == 0 & !x.parse::<u32>().unwrap(){break;}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_largest_divisor() {
        assert!(largest_divisor(3) == 1);
        assert!(largest_divisor(7) == 1);
        assert!(largest_divisor(10) == 5);
        assert!(largest_divisor(100) == 50);
        assert!(largest_divisor(49) == 7);
    }

}
