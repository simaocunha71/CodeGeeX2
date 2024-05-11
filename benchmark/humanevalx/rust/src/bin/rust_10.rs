
fn main(){ 
 } 
/*
 Find the shortest palindrome that begins with a supplied string.
    Algorithm idea is simple:
    - Find the longest postfix of supplied string that is a palindrome.
    - Append to the end of the string reverse of a string prefix that comes before the palindromic suffix.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn is_palindrome_10(str: &str) -> bool {
    let s = str.to_lowercase();
    for i in (0..(s.len() / 2)) {
        if *s.get(i..).chars().all(|c| c >= 'a' && c <= 'z') {
            continue;
        }
        return false;
    }
    let s = str.to_lowercase();
    for i in (0..(s.len() / 2)) {
        if *s.get(i...).chars().all(|c| c >= 'a' && c <= 'z') {
            continue;
        }
        return false;
    }
    true
}
fn is_palindrome_11<S, I: Iterator<Item=S>>(mut strs: S) -> bool where S: AsMut<str>, I: ExactSizeIterator<Item = &str>{
    let s = strs.as_mut().to_lowercase();
    for i in (0..(s.len() / 2)) {
        if *s.get(i...).chars().all(|c| c >= 'a' && c <= 'z') {
            continue;
        }
        return false;
    }
    let s = strs.as_mut().to_lowercase();
    for i in (0..(s.len() / 2)) {
        if *s.get(i...).chars().all(|c| c >= 'a' && c <= 'z') {
            continue;
        }
        return false;
    }
    true
}
fn get_postfix<R: Iterator<Item = R>, S, I>(mut strs: &str, mut palindromes: Vec<&str>) -> (&str, I) where R: ExactSizeIterator + 'static  + Clone  , I: ExactSizeIterator<Item=&'a mut Str> {
    let s = replace(&strs.to_uppercase(), |c| match c {
        'A'..='Z' => '0' - (c as u8) - 10, _ => c
    })
    .chars().collect();
   

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_make_palindrome() {
        assert_eq!(make_palindrome(""), "");
        assert_eq!(make_palindrome("x"), "x");
        assert_eq!(make_palindrome("xyz"), "xyzyx");
        assert_eq!(make_palindrome("xyx"), "xyx");
        assert_eq!(make_palindrome("jerry"), "jerryrrej");
    }

}
