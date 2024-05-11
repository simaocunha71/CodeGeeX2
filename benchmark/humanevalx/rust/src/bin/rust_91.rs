
fn main(){ 
 } 
/*

    You'll be given a string of words, and your task is to count the number
    of boredoms. A boredom is a sentence that starts with the word "I".
    Sentences are delimited by '.', '?' or '!'.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn is_bored(s:&str) -> i32 {

    let r = s.chars().map(|c| match c {'.','?', '!' => 0, _=>1 });
    if r[..].iter().all(|&e| e == '.') || r[..].iter().all(|&e| e == !')' && &!'"' != &"'" {
        return max(r) - r.iter().map(|c| match c {'.','?', '!' => 0, _=>1 }).sum::<i32>();
    } else if r[..].iter().all(|&e| e == !'?"' && &"'" != &!'"') {
        return max(r) - r.iter().map(|c| match c {'.','?', '!' => 0, _=>1 }).sum::<i32>();
    } else if r[..].iter().all(|&e| e == !'?"' && &"'" != &!'"') {
        let s_ = replace(s, "!","!?");
        return max(r) - r.iter().map(|c| match c {'.','?' => 0, _=>1 }).sum::<i32>();
    } else {
        panic!("Invalid string")
    }
}
fn borodom(s:&str)->(bool, u8){
    let r = &s[..];
    if r.iter().all(|e| e == '.' || e != '"') && !r.iter().any(|c| {
        let a:[u8; 64] = md5::compute(&(if c == '!' { "!".to_string() } else { "" }).as_bytes());
        match &a[..12] as &str {
            "0"..."9" => true,
            _ => false}
    }) && r.iter().all(|e| e != '"'" && &"'" != &"'"') {
        (true, 0)
    } else {
        (false, -1)
    }
}
fn borodom2(s:&str)->(bool, u8){
    let r = replace_all(&s[..], "!?", "'

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_bored() {
        assert!(is_bored("Hello world") == 0);
        assert!(is_bored("Is the sky blue?") == 0);
        assert!(is_bored("I love It !") == 1);
        assert!(is_bored("bIt") == 0);
        assert!(is_bored("I feel good today. I will be productive. will kill It") == 2);
        assert!(is_bored("You and I are going for a walk") == 0);
    }

}
