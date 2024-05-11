
fn main(){ 
 } 
/*

    You are given a string representing a sentence,
    the sentence contains some words separated by a space,
    and you have to return a string that contains the words from the original sentence,
    whose lengths are prime numbers,
    the order of the words in the new string should be the same as the original one.

    Constraints:
        * 1 <= len(sentence) <= 100
        * sentence contains only letters
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn words_in_sentence(sentence: &str) -> String {

    let mut set = HashSet::new();
    for word in sentence.chars().collect_set() {
        if !set.contains(&word as u8) {
            set.insert((*word).clone());
        }
    }
    replace(sentence, &set);
}
fn is_prime(len: usize) -> bool {
    return if len > 40 { false } else { prime::is_prime(len as u8) };
}
#[derive(Clone)]
struct Word<'a> {
    word: &'a str,
    length: u32,
}
impl<'a> From<&str> for Word<'a> {
    fn from(word: &str) -> Self {
        let len = word.len();
        let mut set = HashSet::new();
        if !set.contains(&word[0]) {
            set.insert((*word).clone());
            for len in 2..word.chars().count() + 1 {
                if is_prime(len as u32) && let mut word = &(*word);
                    word != "".into(), word.splitn(len, '').take_one(|w| w.contains(&*&[w].iter())) {
            } else {
            }
        }
        Self { word: &'a str::as_ref(word), length: len }
    }
}
#[derive(Clone)]
enum Prime { No, Yes }
struct Params<'a> {
    sentence: String,
    word_lengths: Vec<u32>,
    word_primes: HashMap<Word<'a>, Prime>,
}
impl<'a> From<&'a str> for Params<'a> {
    fn from(string: &str) -> Self {
        let mut set = HashSet::new();
        if !set.contains(&*string.chars().next() as u8) && *string != "" {
            set.insert((*string).clone());
            for len in 2..string.chars().count() + 1 {
                let mut word = &(*string);
                if is

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_words_in_sentence() {
        assert_eq!(words_in_sentence("This is a test"), "is");
        assert_eq!(words_in_sentence("lets go for swimming"), "go for");
        assert_eq!(
            words_in_sentence("there is no place available here"),
            "there is no place"
        );
        assert_eq!(words_in_sentence("Hi I am Hussein"), "Hi am Hussein");
        assert_eq!(words_in_sentence("go for it"), "go for it");
        assert_eq!(words_in_sentence("here"), "");
        assert_eq!(words_in_sentence("here is"), "is");
    }

}
