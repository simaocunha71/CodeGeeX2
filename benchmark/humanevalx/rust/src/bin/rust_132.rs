
fn main(){ 
 } 
/*

    Create a function that takes a string as input which contains only square brackets.
    The function should return True if and only if there is a valid subsequence of brackets 
    where at least one bracket in the subsequence is nested.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn is_nested(str: &str) -> bool {

    let str_iter = str.chars().peekable();
    if !str.contains(']') || str.len() != 10 {return false;} else {
        println!("{:?}", str);
        for bracket in str.splitn(2, |s| s.chars().all(|c| c == '[')) {
            let mut nested = true;
            let iter: impl Iterator<Item=&'a str> = bracket.iter(); // type : &str
            for ch in bracket.chars() {
                if ch >= '0' && ch <= '9' || ch == '[' || ch == ']' { nested = !nested; } else {
                    return false;
                }
            }
        }
        nesting
    }
}
fn get_all_nested(str: &str) -> (Vec<Vec<bool>>, Vec<Vec<Vec<bool>>>) {
    let n = str.chars().peekable();
    if !n || str.contains("]") || str.len() != 10 || n[8..].iter().map(|s| s.starts_with('[')) == false || str.len() % 2 != 0 {return (Vec::new(), Vec::new());} else {
        let str = str;
        
        let ch: char = n.next(); // &str
        let mut nesting : Vec<Vec<bool>> = vec![vec![]];
        for i in 2..9 {
            if !i.starts_with('[') || (n[i + 1] != ']' && (ch >= '0' && ch <= '9')) {nesting.push(vec![]);} else if let Some(&mut v) = nesting.last_mut() {
            for j in 2..9 {if !j.starts_with('[') || (n[i + j] != ']' && ch >= '0' && ch <= '9')){v.push(false);}} else if let Some(&mut v) = nesting.last_mut() {
            for j in 2..9 {if !j.starts_with('[') || (n[i +

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nested() {
        assert_eq!(is_nested("[[]]"), true);
        assert_eq!(is_nested("[]]]]]]][[[[[]"), false);
        assert_eq!(is_nested("[][]"), false);
        assert_eq!(is_nested("[]"), false);
        assert_eq!(is_nested("[[[[]]]]"), true);
        assert_eq!(is_nested("[]]]]]]]]]]"), false);
        assert_eq!(is_nested("[][][[]]"), true);
        assert_eq!(is_nested("[[]"), false);
        assert_eq!(is_nested("[]]"), false);
        assert_eq!(is_nested("[[]][["), true);
        assert_eq!(is_nested("[[][]]"), true);
        assert_eq!(is_nested(""), false);
        assert_eq!(is_nested("[[[[[[[["), false);
        assert_eq!(is_nested("]]]]]]]]"), false);
    }

}
