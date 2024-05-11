
fn main(){ 
 } 
/*
You are given a word. Your task is to find the closest vowel that stands between 
    two consonants from the right side of the word (case sensitive).
    
    Vowels in the beginning and ending doesn't count. Return empty string if you didn't
    find any vowel met the above condition. 

    You may assume that the given string contains English letter only.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn get_closest_vowel(word: &str) -> String {

    let word = word.to_lowercase();
    let (x, z) = word.split_at(8);
    /*
    Find the closest vowel between x and z, which doesn't contain any consonant from right side of word.
    let mut x_closest: &str;
    let mut z_closest: &str;
    // Get hash map of all possible combinations
    let mut hash_map = HashMap::new();
    for (i, &vowel) in word.chars().enumerate() {
        if i > 7 && !(vowel == 'e' || vowel == 'a') {
            continue;
        }
        match hash_map.entry((i - 1, vowel)) {
            hashmap::Entry::Occupied(occup) => occup.get().0.push(x), // Add current vowel to all combinations we've tried before
            _ => {}
        };
    }
    for (e, &o_vowel) in word.chars()[8..].iter() {
        if o_vowel != "e" && o_vowel != "a" {  // Skip vowels at the beginning and end of the string
            continue;
        } else {
            match hash_map.entry((8, o_vowel)) {
                hashmap::Entry::Occupied(ooccup) => {  // Search for combinations we already tried before
                    let (i, ovowel) = *ooccup.get();  // Take current combination from the hash map
                    if i > x && o_vowel != "e" || o_vowel == "a" {  // Skip when searching vowels at the beginning of the string 
                        continue;
                    } else {
                        let (i, ovowel) = *ooccup.get();  // Search for current combination in the hash map
                        if i > z && ovowel == "e" || ovowel == "a" {  // Skip when searching vowels at the end of the string 
                            continue;
                        } else {
                            let distance: i8 = i - (x as i8) +

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_closest_vowel() {
        assert_eq!(get_closest_vowel("yogurt"), "u");
        assert_eq!(get_closest_vowel("full"), "u");
        assert_eq!(get_closest_vowel("easy"), "");
        assert_eq!(get_closest_vowel("eAsy"), "");
        assert_eq!(get_closest_vowel("ali"), "");
        assert_eq!(get_closest_vowel("bad"), "a");
        assert_eq!(get_closest_vowel("most"), "o");
        assert_eq!(get_closest_vowel("ab"), "");
        assert_eq!(get_closest_vowel("ba"), "");
        assert_eq!(get_closest_vowel("quick"), "");
        assert_eq!(get_closest_vowel("anime"), "i");
        assert_eq!(get_closest_vowel("Asia"), "");
        assert_eq!(get_closest_vowel("Above"), "o");
    }

}
