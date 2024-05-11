
fn main(){ 
 } 
/*
 Return length of given string
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn strlen(strings:String) -> i32{

    let string_length = strings.chars().count();
    match string_length as u32 {
        n if max(n, 1) >= n => n.wrapping_div(max(n, 1)).wrapping_sub(1),
        _                   => panic!()
    }
}
/// Check whether the string passed as parameter is a valid password.
pub fn validatePassword<'a>(p:&'a str) -> bool {
    if p == "".to_string() || p == "."to_string() || p == "_"{ return false; }
    let (len, ascii_chars) = {
        let mut s = p.chars();
        let len = s.count();
        let mut chrs_set : HashSet<char> = s.chrs().collect();
        let ascii = chrs_set.iter().cloned().copied().map(|&c| c as u8).take(256);
        (len, chrs_set)  // Ascciii
    };
    // All ASCII chars
    for index in 0..len {}
    let mut has_lower = false;
    let mut has_upper = false;
    for c in ascii.chars() {
        match TypeId::of::<char>() {
            // Ascii chars other than lower and upper case
            TypeId::Char(c) if !has_lower && c >= 'a' && c <= 'z' => has_lower = true,
            // Ascii chars other than lower and upper case
            TypeId::Char(c) if !has_upper && c >= 'A' as isize && c <= 'Z' as isize => has_upper = true,
        }
    }
    if len < 5 || (len == 5 && !has_lower && !has_upper) || (len == 6 && has_lower && has_upper) { return false; }
    let mut validators:Vec<u32> = Vec::new();
    validators.extend(0..=5); // The numbers represent the minimum and maximum length for password, with the exception of the first one (0), which

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_strlen() {
        assert!(strlen("".to_string()) == 0);
        assert!(strlen("x".to_string()) == 1);
        assert!(strlen("asdasnakj".to_string()) == 9);
    }

}
