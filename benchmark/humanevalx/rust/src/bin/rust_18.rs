
fn main(){ 
 } 
/*
 Find how many times a given substring can be found in the original string. Count overlaping cases.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn how_many_times(string: String, substring:String) -> i32{

    let (min_len, max_len) = if substring.len() <= 3 {
        (0, 2)
    } else if substring.len() > 36 {
         (0, 25)
    } else {
        // 3<=|substring|< 17 => |(1..|substring|)|
        let (min_len, max_len) = substring[0..].chars().fold((0, 4))(|acc,(c,i)|{max(acc.0, acc.1 + if i == 3 || i == 8 || i == 9 {2} else {1})}, ());
        // 17<=|substring|< 36 => |...|+1
        let max_len = substring[0..].chars().fold(max(3,4))(|acc,(c,i)|{max(acc.0 + if i == 1 || i == 8 || i == 9 {1} else {2})}, ());
        
        // |...|>=1 && min_len>0 & 1<=|substring| <= max_len => |(min_len..max_len)|
        let len = substring.chars().count();
        if len != 0 {
            let mut res : i32 = 0;
            for l in (0..len).rev(){
                //println!("l: {}",l);
                if substring[l] == '8' || substring[l] == '9',res += 1, continue;
                for r in (l-1)..!(l+1) {
                    let c = substring.get(r).unwrap();
                    if substring[l] != '8' && substring[l] != '9' , res+=2;
                    else{}
                }
            }
        return res;    
        
      } else { // substring < 3 || substring > 36 => |(0..|substring|)|
        (0, substring.len())
      };
    };
  
    if string.chars().any(|c| c == substring) && min_len>=0{
        let mut res : HashMap<&'a String , i32> =

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_how_many_times() {
        assert!(how_many_times("".to_string(), "x".to_string()) == 0);
        assert!(how_many_times("xyxyxyx".to_string(), "x".to_string()) == 4);
        assert!(how_many_times("cacacacac".to_string(), "cac".to_string()) == 4);
        assert!(how_many_times("john doe".to_string(), "john".to_string()) == 1);
    }


}
