
fn main(){ 
 } 
/*
You are given a string s.
    Your task is to check if the string is happy or not.
    A string is happy if its length is at least 3 and every 3 consecutive letters are distinct
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn is_happy(s:&str) -> bool{

    let len = s.len();
    if len < 3 || ![1..=len].iter().all(|v| v%3 == Some(2)) {
        println!("{}", s);
        return false;
    }
    //let mut res: Vec<char> = vec!();
    let is_happy = s.windows(3).all(|a| a.iter().all(|c| !matches::Regex::is_match(".*\\1$")));
    println!("{}", is_happy);
    //let is_happy: &[char; 20] = &s;
    return *is_happy;
}
fn main() {
    let mut rng = rand::thread_rng();
    for _ in 0..13 {
        let s = String::<char>::from_chars(vec![rng.permutation(&['a'-'z']), rng.permutation(&['a'-'z'])]);
        
        if is_happy(&s) == true {
            //println!("{} is happy", &s);
        }else{
            println!("{} is not happy:  {}", &s, String::from_chars(vec![s.chars().max()]) );
            
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_happy() {
        assert!(is_happy("a") == false);
        assert!(is_happy("aa") == false);
        assert!(is_happy("abcd") == true);
        assert!(is_happy("aabb") == false);
        assert!(is_happy("adb") == true);
        assert!(is_happy("xyy") == false);
        assert!(is_happy("iopaxpoi") == true);
        assert!(is_happy("iopaxioi") == false);
    }

}
