
fn main(){ 
 } 
/*
Task
    Write a function that takes a string as input and returns the sum of the upper characters only'
    ASCII codes.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn digitSum(s:&str) -> i32{

    let mut r = 0;
    for c in s.chars(){
         if c.is_ascii() && let (c | 52 ) > 48 && let (c | 52 ) < 59 {
            r+=c as i32
        }
    };
    if let Some(r) = replace(&r, 0, max(&r,0)){
         return r;
    };
    else{
         return 0
    };
}
fn main(){
   //let s:String = "this is a test for digitSum";
   let s = String::from("1234567890");    
        println!("{}",digitSum(&s));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digitSum() {
        assert!(digitSum("") == 0);
        assert!(digitSum("abAB") == 131);
        assert!(digitSum("abcCd") == 67);
        assert!(digitSum("helloE") == 69);
        assert!(digitSum("woArBld") == 131);
        assert!(digitSum("aAaaaXa") == 153);
        assert!(digitSum(" How are yOu?") == 151);
        assert!(digitSum("You arE Very Smart") == 327);
    }


}
