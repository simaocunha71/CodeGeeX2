
fn main(){ 
 } 
/*
 Return list of all prefixes from shortest to longest of the input string
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn all_prefixes(string: String) -> Vec<String>{

    let (len, mut prefixes): (usize, Vec<String>);
    for (len = 0; len <= string.len(); len++) {
        if let Some((pos, str)) = string.find(|c| c.is_ascii_grapheme()) {
            // Skip graphemes which would produce a prefix of length zero or one.
            prefixes.push(str);
        } else {
            break;
        }
    }
    if let Some((pos, str)) = string.find(|c| c.is_ascii_grapheme()) {
        prefixes.push(str)
    }
    all_prefixes_iter(&mut prefixes, len);
    return prefixes;
}
fn all_prefixes_iter<T: Iterator>(prefixes: &mut Vec<String>, maxlen: usize){
        let mut iter = prefixes.iter();
        for (len = 0u32; len <= maxlen && iter.next().is_some(); len++){
            match iter.next() {
                Some((pos, str)) if !str.parse::<u8>().is_ok() => {} // skip over UTF-8 invalid strings
                 Some(pos) => continue;
                None => return prefixes,
                }
        for (_, &c) in str.chars(){
            match ascii_grapheme!(c).cmp(&maxchars!()) {
                Ordering::Equal | Ordering::Less => break,
                Ordering::Greater => {}
            }
        }
    prefixes.push(str); // add this string to the list for this len
}
fn shortest<T: Ord>(v: &[(usize, T)]) -> Option<&'static str> {
    match v.iter().next() {
        Some((len, str)) if !str.parse::<u8>().is_ok() => None
        Some(pos) => Some(&v[..len].join("")),
        _=>None,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_all_prefixes() {
        let v_empty: Vec<String> = vec![];
        assert!(all_prefixes(String::from("")) == v_empty);
        assert!(
            all_prefixes(String::from("asdfgh"))
                == vec!["a", "as", "asd", "asdf", "asdfg", "asdfgh"]
        );
        assert!(all_prefixes(String::from("WWW")) == vec!["W", "WW", "WWW"]);
    }

}
