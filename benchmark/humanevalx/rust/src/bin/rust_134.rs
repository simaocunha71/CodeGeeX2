
fn main(){ 
 } 
/*

    Create a function that returns True if the last character
    of a given string is an alphabetical character and is not
    a part of a word, and False otherwise.
    Note: "word" is a group of characters separated by space.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn check_if_last_char_is_a_letter(txt: &str) -> bool {

    let s = txt.chars().rev();
    for _ in 0..s.len() - 1 {}
    match s[s.len()-1] {
        'A'...='Z',
        'a'...='z',
        'C'...='L',
        c if let Some(h) = char_to_hashmap(c).and_then(|h| h.iter().rev()) {
            match h.len() { 1 => true, _ => false }
        }
        _ => false
    }
}
pub fn finder_last_letter_is_a_letter(word: &str) -> bool {
    check_if_last_char_is_a_letter(word.to_lowercase())
}
fn char_to_hashmap<'a>(character: 'a) -> Option<&'a str> {
    match character {
        'C', 'L', 'G' => Some("central", "south", "east"),
        'D', 'U', 'F', 'J' => Some("west", "north", "down", "up"),
        c => Some(format!("{}")),
    }.and_then(|h| h.iter().rev())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_if_last_char_is_a_letter() {
        assert_eq!(check_if_last_char_is_a_letter("apple"), false);
        assert_eq!(check_if_last_char_is_a_letter("apple pi e"), true);
        assert_eq!(check_if_last_char_is_a_letter("eeeee"), false);
        assert_eq!(check_if_last_char_is_a_letter("A"), true);
        assert_eq!(check_if_last_char_is_a_letter("Pumpkin pie "), false);
        assert_eq!(check_if_last_char_is_a_letter("Pumpkin pie 1"), false);
        assert_eq!(check_if_last_char_is_a_letter(""), false);
        assert_eq!(check_if_last_char_is_a_letter("eeeee e "), false);
        assert_eq!(check_if_last_char_is_a_letter("apple pie"), false);
    }

}
