
fn main(){ 
 } 
/*
Write a function vowels_count which takes a string representing
    a word as input and returns the number of vowels in the string.
    Vowels in this case are 'a', 'e', 'i', 'o', 'u'. Here, 'y' is also a
    vowel, but only when it is at the end of the given word.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn vowels_count(s:&str) -> i32 {

    let r = s.chars().collect::<Vec<char>>();
    (0..r.len()).map(|i| { 
        let c : char= r[i]; 
        if c.is_vowel() && !c.is_uppercase() { 
            replace(s, i, r.get(i)); 
        } else { 
            0 
        }
    }).sum::<usize>() as i32
}
fn main() -> () {
    let vowels = HashSet::new();
    for _ in (0..1_000_000).rev() {
        let s = rand_string(5);
        
        println!("{}", md5(&s));
        println!("{}", typeid::of::<i32>().repr());
        println!("{}", vowels_count(&s));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_vowels_count() {
        assert!(vowels_count("abcde") == 2);
        assert!(vowels_count("Alone") == 3);
        assert!(vowels_count("key") == 2);
        assert!(vowels_count("bye") == 1);
        assert!(vowels_count("keY") == 2);
        assert!(vowels_count("bYe") == 1);
        assert!(vowels_count("ACEDY") == 3);
    }

}
