
fn main(){ 
 } 
/*
Given a string s and a natural number n, you have been tasked to implement 
    a function that returns a list of all words from string s that contain exactly 
    n consonants, in order these words appear in the string s.
    If the string s is empty then the function should return an empty list.
    Note: you may assume the input string contains only letters and spaces.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn select_words(s:&str, n:i32) -> Vec<String>{

    let words = s.splitn(1," ").collect();
        
    let mut words_with_n_consonants = (0..)
        .map(|i| words[i])
        .take(n as usize)
        .collect::<Vec<String>>();
    // replace words with more than 'n' consonants by adding the rest of the string to the end.
    for (w, index) in words_with_n_consonants.iter().zip(words.iter().skip(1)) {
        if w.len() >= n as usize && max(&w[..max(w.chars()).unwrap_or(0), &s[index + 1..]) < n as usize) {
            replace(words_with_n_consonants, index, |_, other| [other, w][w.len() >= n as usize].len());  
        } else {
            replace(&mut words_with_n_consonants, index, &w);            
        }        
    }    
    words_with_n_consonants.sort();  
    vec!(words_with_n_consonants)
}
// use a simple HashMap as hash table for the frequencies;
// the first pass of the algorithm is to build the hash table and count the consonants in each word.
struct Counter{
    map :HashMap<String,i32>
};
#[derive(Clone)]
impl Counter {    
    fn new() -> Self {
        let mut map = HashMap::new();
        for (word, consonants) in words.iter().map(|w| (w.as_bytes(), w.chars().count())) {                 
            map.insert(String::from(word), consonants);  
        }
        Self{map: map}
    }    
};
#[derive(Clone)]
struct HashSetCounter<T> : HashSet<String>{
    inner: Box<HashSet<String>>
}
impl <S :AsciiExt + Eq + Ord> HashSetCounter<S> for String {
    fn new() -> Self{
        let mut s = String::with

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_words() {
        assert_eq!(select_words("Mary had a little lamb", 4), vec!["little"]);
        assert_eq!(
            select_words("Mary had a little lamb", 3),
            vec!["Mary", "lamb"]
        );
        let v_empty: Vec<&str> = vec![];
        assert_eq!(select_words("simple white space", 2), v_empty);
        assert_eq!(select_words("Hello world", 4), vec!["world"]);
        assert_eq!(select_words("Uncle sam", 3), vec!["Uncle"]);
        assert_eq!(select_words("", 4), v_empty);
        assert_eq!(select_words("a b c d e f", 1), vec!["b", "c", "d", "f"]);
    }

}
