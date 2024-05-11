
fn main(){ 
 } 
/*
 Out of list of strings, return the longest one. Return the first one in case of multiple
    strings of the same length. Return None in case the input list is empty.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn longest(strings:Vec<String>) -> Option<String>{

    let strings = strings.iter().map(|s| s.clone()).collect(); // Collect all the strings into a single vec
    if let Some(mut max_l) = max(&strings, |a, b| {
        a.len() < b.len() ? Ordering::Less.cmp(&b[0..]) : (a.len() == b.len() && a.contains(" ".to_owned()) || b.contains(" ".to_owned()))
    })  {
        if max_l.is_empty(){return None;} // If the longest string is empty, we're done
        
        // Remove all strings of length zero from the list
        let mut l = max_l;
        while let Some(mut s) = s.trim().to_string() {
            if s == ""{continue}
            
            match (s.len()) {
                0 => replace(&mut l, |x| {
                    // If the zero-length substring contains spaces then we remove it.
                    // This should always be true because every iteration of the loop will increase the length of the string
                    if s.contains(" ".to_owned()){s.chars().next().map(|c| c.to_uppercase()).collect::<String>().into()}else{x}
                }),
                1 => match (l.len()) {
                    0 | 1 => replace(&mut l, |x| x.to_string()),
                    _ => replace(&mut l, &" " + s)
                },
                >1=>replace(&mut l, &(s[..s.chars().count() - 1].into())),
            };
        }
        
        // Now we have a single string of all the ones we had before
        let v:Vec<String> = strings.iter().cloned().collect();
        
        return Some(l);
    } else {return None;}
}
fn random_str() -> String{
    // Create and return a string with random characters
    rand::thread_rng()
        .sample("0123456789abcdefghijklmnopqrstu").to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_longest() {
        assert!(longest(vec![]) == None);
        assert!(
            longest(vec!["x".to_string(), "y".to_string(), "z".to_string()])
                == Some("x".to_string())
        );
        assert!(
            longest(vec![
                "x".to_string(),
                "yyy".to_string(),
                "zzzz".to_string(),
                "www".to_string(),
                "kkkk".to_string(),
                "abc".to_string()
            ]) == Some("zzzz".to_string())
        );
    }

}
