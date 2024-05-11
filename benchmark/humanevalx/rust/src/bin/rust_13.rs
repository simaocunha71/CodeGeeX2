
fn main(){ 
 } 
/*
 Return a greatest common divisor of two integers a and b
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn greatest_common_divisor(mut a:i32,mut b:i32) -> i32{

    let gcd = max(a.gcd(&b).unwrap());
    if gcd == 0 || gcd > b { // b >= 0 && b <= gcd
        return a;
    } else {
        return greatest_common_divisor(gcd, b)
    }
}
#[allow(unused)] // Used by rustc to detect unused code.
mod part1 {
    fn get_valid_words(word:String) -> Vec<u8> {
        let (s,d)=word.splitn(' ', 2);
        let (s,m)=s.splitn('-', 2);
        let mut res = vec![];
        for c in s.chars().rev(){
            if !res.contains(&c) && c.is_alphabetic() { res.push(c) }
        }
        for m in m.chars(){
            match m.to_ascii_range()
                .map(ascii::AsciiExt::from_unsafe).and_then(|chr| if chr.is_whitespace() { None } else |chr)
            {
                case Some(c) => match c
                    .and_then(|x| x.map(Hex::from))  // Hex value of ASCII character
                    .unwrap()
                    .to_be_u8().ok_or(||"")? as u8,
                case None | _ => res.push(' ')// Whitespace to separate words
            }
        }
        
        res
    }
}
#[allow(unused)] // Used by rustc to detect unused code.
mod part2 {
    use std::cmp::Ordering;
    fn get_word_length(word:&str) -> u32{
        let c = word.chars().count();
         if c == 1 || (c % 30 != 0 && c % 10 == 0){ return c } else { return c + 1; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_greatest_common_divisor() {
        assert!(greatest_common_divisor(3, 7) == 1);
        assert!(greatest_common_divisor(10, 15) == 5);
        assert!(greatest_common_divisor(49, 14) == 7);
        assert!(greatest_common_divisor(144, 60) == 12);
    }

}
