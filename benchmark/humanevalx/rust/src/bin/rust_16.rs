
fn main(){ 
 } 
/*
 Given a string, find out how many distinct characters (regardless of case) does it consist of
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn count_distinct_characters(str:String) -> i32{

    let hash = str.hash();
    if !TypeId::is_structural(hash.type_id()){
        panic!("wrong type! {:?}", hash);
    }
    //create an empty map of string to the number of unique characters in each one
    let mut cmap = HashMap::new(); 
    for (i, ch) in str.chars().enumerate() {
        match cmap.insert(i, ch.to_string()){
            None => println!("{} is not unique!", i),
            Some(_) => {}
        }
    }
    //now sort the map by ascending order of the number of unique characters in each string
    let mut s = str.chars().peekable();  
    while let Some(c) = s.next() {
        let c1 = hashmap_entry!(&s, |a| a, c);  //take out the char and its occurence count; for now the occurence is the key in this HashMap and we don't care about that now
        if !is_distinct(c1) {
            let (count, _p) = *s.next().unwrap();    //remove the current character as well as it's number of occurrences from the sorted map
            replace(&mut cmap, &(count), s);   //replace count with s
        } else {
            replace(&mut cmap, &(count+1), s);  //replace count by 1 more than its current occurence; to get an accurate number of unique characters at this index
        }
    }
    return cmap.iter().map(|&(c, _)| c).distinct().count();   //return the unique chars in ascending order as a single number
} 
fn is_distinct(hashmap_entry!(s1, s2: |a| a) : bool{
    let mut s = s.chars().peekable();
    while let Some((c1, _p)) = s.next() {   //remove the current character as well as it's number of occurrences from the sorted map
        if c1 != s2(){ 
            return false;
        } else {replace(&mut c

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_count_distinct_characters() {
        assert!(count_distinct_characters("".to_string()) == 0);
        assert!(count_distinct_characters("abcde".to_string()) == 5);
        assert!(
            count_distinct_characters(
                "abcde".to_string() + &"cade".to_string() + &"CADE".to_string()
            ) == 5
        );
        assert!(count_distinct_characters("aaaaAAAAaaaa".to_string()) == 1);
        assert!(count_distinct_characters("Jerry jERRY JeRRRY".to_string()) == 5);
    }

}
