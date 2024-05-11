
fn main(){ 
 } 
/*
Your task is to write a function that returns true if a number x is a simple
    power of n and false in other cases.
    x is a simple power of n if n**int=x
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn is_simple_power(x:i32, n:i32) -> bool{

    let (m, n) = (n, x); //swap i32 and i64 to make the ascii comparison work
  
    let mut hash : HashSet<u8> = HashSet::new();
    for c in &[&'a b'0',&'a b'1','b'2','b'3','b'4','b'5','b'6','b'7','b'8','b'9']{
        let x_hash: HashSet<u8> = c.to_string().chars().rev()
            .map(|c| *c as u8)
            .collect();
        hash.insert(x_hash);
    }
    if n < 0 {n = -n;}
    let (mut m, n) = (n, x); //swap to make ASCII comparison work
    if m == 1{return true;}//if 1 is a power of n then return
    else{
        while(m > 0){
            //if the left digit of x is not equal to one of our 0's then return false:
            let (mut digit, mut m) = (m%10, m/10);
        
            if !hash.contains(&digit as u8) {return false;} //check if a number's digit isn't any of the 0's in our set
            else{m /= 10; digit -= 1}//if it is then remove it from the set and check again
        }
    }    
        
    return true; //if m == n, that means x = n**1 = n
}
fn main() {
    let mut r: Rng = rand::thread_rng();
    let (m, n) :(i64, i32)=(69704533, 5); //generate the random number m that we are checking for powers of n
    println!("m={}", m);//print out some debug info...
    if is_simple_power(m,n){println!("Simple power of {}", n)}else{println!("\t Not a simple power of {}", n)}
}

#[cfg(test)]
mod tests {
    use super::*;

  #[test]
    fn test_is_simple_power() {
        assert!(is_simple_power(1, 4) == true);
        assert!(is_simple_power(2, 2) == true);
        assert!(is_simple_power(8, 2) == true);
        assert!(is_simple_power(3, 2) == false);
        assert!(is_simple_power(3, 1) == false);
        assert!(is_simple_power(5, 3) == false);
        assert!(is_simple_power(16, 2) == true);
        assert!(is_simple_power(143214, 16) == false);
        assert!(is_simple_power(4, 2) == true);
        assert!(is_simple_power(9, 3) == true);
        assert!(is_simple_power(16, 4) == true);
        assert!(is_simple_power(24, 2) == false);
        assert!(is_simple_power(128, 4) == false);
        assert!(is_simple_power(12, 6) == false);
        assert!(is_simple_power(1, 1) == true);
        assert!(is_simple_power(1, 12) == true);
    }

}
