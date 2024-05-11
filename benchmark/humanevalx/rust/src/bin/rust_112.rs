
fn main(){ 
 } 
/*
Task
    We are given two strings s and c, you have to deleted all the characters in s that are equal to any character in c
    then check if the result string is palindrome.
    A string is called palindrome if it reads the same backward as forward.
    You should return a tuple containing the result string and True/False for the check.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn reverse_delete(s:&str, c:&str) -> Vec<String> {

    let (s1, s2): (Vec<u8>, HashSet<char>) = |&s| {
        let str = &*s.to_ascii_ref();
        
        let mut v = Vec::new();
        for c in str.chars() {
            if !v.contains(&c) {
                let x = str.parse().unwrap();
                v.push(x);
            }
        }
        (v, HashSet::<char>::from_iter(str))
    };
    /* 1. find all characters in c that appears in s */
    let mut seen: HashSet<u8> = HashSet::new();
    for c in c.chars() {
        if !seen.contains(&c) {
            seen.insert(c);
        }
    }
    /* 2. delete all characters from s that occurs in any character of c */
    let mut res: Vec<String> = vec![];
    for x in s1.iter_mut() {
        if replace(x, &seen, |y| {
                match y{ 
                            Some(ref e) => { let f: char = e; self_for_each(&(f as u8)) }, 
                            None => self_for_each(|z| unreachable!() ) }) == &seen {
        
            if seen.iter().all(|c2| c2 != x) { res.push("".to_string()) } else { res.push(x.to_string()) };
        }
    }
    /* 3. check if the final result string is a palindrome */
    let m: HashSet<u8> = seen.clone();
    for x in s2.iter() {
        if !m.remove(&x) && replace(x, &seen, |y| self_for_each(|z| unreachable!())) == &seen { res.push("".to_string()) };
    }
    println!("s: {:?}", s);
    println!("c: {:?}", c);
    for x in res.iter() { println!("{}", x) }
   

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_delete() {
        assert!(reverse_delete("abcde", "ae") == ["bcd", "False"]);
        assert!(reverse_delete("abcdef", "b") == ["acdef", "False"]);
        assert!(reverse_delete("abcdedcba", "ab") == ["cdedc", "True"]);
        assert!(reverse_delete("dwik", "w") == ["dik", "False"]);
        assert!(reverse_delete("a", "a") == ["", "True"]);
        assert!(reverse_delete("abcdedcba", "") == ["abcdedcba", "True"]);
        assert!(reverse_delete("abcdedcba", "v") == ["abcdedcba", "True"]);
        assert!(reverse_delete("vabba", "v") == ["abba", "True"]);
        assert!(reverse_delete("mamma", "mia") == ["", "True"]);
    }

}
