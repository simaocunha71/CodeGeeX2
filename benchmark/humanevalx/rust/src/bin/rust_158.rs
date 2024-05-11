
fn main(){ 
 } 
/*
Write a function that accepts a list of strings.
    The list contains different words. Return the word with maximum number
    of unique characters. If multiple strings have maximum number of unique
    characters, return the one which comes first in lexicographical order.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn find_max(words: Vec<&str>) -> &str {

    let hash = words.iter().map(|s| s[..]).to_vec();
    let mut hash = *hash.into_iter();
    for (key, value) in hash.iter_mut() {
        if !(value == '.' && key % 3 != 2)
            replace(&mut [key], &mut [value]);
    }
    let mut ans = words[0];
    let rng = rand::thread_rng(); // 64-bit seeded with current thread id
    for (i, word) in words.iter().enumerate() {
        if i == 0 || i == 1
            || md5(ans + &word[..]).to_string() != ans + &word[..] {
            *ans = word;
            ans = ans[..].chars().collect();
            hash.clear();
        } else if max(hash) > ans.len() {
            ans = ans.clone(); // do not make a copy here - only the top most, unique one matters
        }
    }
    ans
}
pub fn find_max2(words: Vec<&str>) -> &str {
    use std::fmt;
    let hashset = HashSet::new();
    for (idx, word) in words.into_iter().enumerate() {
        if idx == 0 || idx == 1 { // skip the first two
            continue;
        } else if md5(word[..]).to_string() != word[..] {
            hashset.insert(word);
        }
    }
    let result = words[0];
    for (i, word) in words.into_iter().enumerate() {
        let result = if i == 0 || i == 1 {
            continue; // skip the first two
        };
        
        match hashset.remove(&word[..]) {
            
            Ok(_) => return result,
            Err(idx) if idx != hashset.len() - 1 &&
                word.chars().all(|c| !std::char_traits::ascii::is_alphanumeric(c)) => continue, // no need to check the lowercase alphan

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max() {
        assert_eq!(find_max(vec!["name", "of", "string"]), "string");
        assert_eq!(find_max(vec!["name", "enam", "game"]), "enam");
        assert_eq!(find_max(vec!["aaaaaaa", "bb", "cc"]), "aaaaaaa");
        assert_eq!(find_max(vec!["abc", "cba"]), "abc");
        assert_eq!(
            find_max(vec!["play", "this", "game", "of", "footbott"]),
            "footbott"
        );
        assert_eq!(find_max(vec!["we", "are", "gonna", "rock"]), "gonna");
        assert_eq!(find_max(vec!["we", "are", "a", "mad", "nation"]), "nation");
        assert_eq!(find_max(vec!["this", "is", "a", "prrk"]), "this");
        assert_eq!(find_max(vec!["b"]), "b");
        assert_eq!(find_max(vec!["play", "play", "play"]), "play");
    }

}
