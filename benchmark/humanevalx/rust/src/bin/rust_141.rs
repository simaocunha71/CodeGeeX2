
fn main(){ 
 } 
/*
Create a function which takes a string representing a file's name, and returns
    'Yes' if the the file's name is valid, and returns 'No' otherwise.
    A file's name is considered to be valid if and only if all the following conditions 
    are met:
    - There should not be more than three digits ('0'-'9') in the file's name.
    - The file's name contains exactly one dot '.'
    - The substring before the dot should not be empty, and it starts with a letter from 
    the latin alphapet ('a'-'z' and 'A'-'Z').
    - The substring after the dot should be one of these: ['txt', 'exe', 'dll']
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn file_name_check(file_name: &str) -> &str {

    let valid_names = vec![
        "023649.txt",
        "aaa.exe", 
        "aaa1.dll"
    ];
    for (i, f) in file_name.split('.').enumerate() do {
        match replace(f, idx: i, with: &valid_names[max(0, i) % valid_names.len())) {
            Err(_) => return "", 
            Ok((new_file_name, _)) => return new_file_name
        }
    };
}
pub fn file_name_checker<T: TypeId + Any>(file_name: &str) -> T {
    let valid_names = vec![
        "023649.txt", 
        "aaa.exe"
    ];
    for (i, f) in file_name.split('.').enumerate() do {
        match replace(f, idx: i, with: &valid_names[max(0, i) % valid_names.len())) {
            Err(_) => return "", 
            Ok((new_file_name, _)) => return new_file_name
        };
    }
}
// #include <regex>
fn file_name_checker2<T: TypeId + Any>(file_name: &str) -> T {
    let dot = file_name.find('.').unwrap();
    match (file_name[0..dot.position()], file_name[file_name.chars().count() - 1]) {
        (s, f@'.') => return f, 
        _ => ()
    };
}
impl<T:TypeId + Any> FileNameChecker for String where T: AsciiExt {
    type Type = T;
    fn file_name_check(&self) -> Self::Type {
        let mut rng = rand();
        let mut hashset = HashSet::new().collect();
        
        while !hashset.is_empty() {
            let s: &str = self;
            match (s[0..3], s[s.chars().count() - 1]) {
                (

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_name_check() {
        assert_eq!(file_name_check("example.txt"), "Yes");
        assert_eq!(file_name_check("1example.dll"), "No");
        assert_eq!(file_name_check("s1sdf3.asd"), "No");
        assert_eq!(file_name_check("K.dll"), "Yes");
        assert_eq!(file_name_check("MY16FILE3.exe"), "Yes");
        assert_eq!(file_name_check("His12FILE94.exe"), "No");
        assert_eq!(file_name_check("_Y.txt"), "No");
        assert_eq!(file_name_check("?aREYA.exe"), "No");
        assert_eq!(file_name_check("/this_is_valid.dll"), "No");
        assert_eq!(file_name_check("this_is_valid.wow"), "No");
        assert_eq!(file_name_check("this_is_valid.txt"), "Yes");
        assert_eq!(file_name_check("this_is_valid.txtexe"), "No");
        assert_eq!(file_name_check("#this2_i4s_5valid.ten"), "No");
        assert_eq!(file_name_check("@this1_is6_valid.exe"), "No");
        assert_eq!(file_name_check("this_is_12valid.6exe4.txt"), "No");
        assert_eq!(file_name_check("all.exe.txt"), "No");
        assert_eq!(file_name_check("I563_No.exe"), "Yes");
        assert_eq!(file_name_check("Is3youfault.txt"), "Yes");
        assert_eq!(file_name_check("no_one#knows.dll"), "Yes");
        assert_eq!(file_name_check("1I563_Yes3.exe"), "No");
        assert_eq!(file_name_check("I563_Yes3.txtt"), "No");
        assert_eq!(file_name_check("final..txt"), "No");
        assert_eq!(file_name_check("final132"), "No");
        assert_eq!(file_name_check("_f4indsartal132."), "No");
        assert_eq!(file_name_check(".txt"), "No");
        assert_eq!(file_name_check("s."), "No");
    }

}
