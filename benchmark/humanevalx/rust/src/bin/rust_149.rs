
fn main(){ 
 } 
/*
Write a function that accepts a list of strings as a parameter,
    deletes the strings that have odd lengths from it,
    and returns the resulted list with a sorted order,
    The list is always a list of strings and never an array of numbers,
    and it may contain duplicates.
    The order of the list should be ascending by length of each word, and you
    should return the list sorted by that rule.
    If two words have the same length, sort the list alphabetically.
    The function should return a list of strings in sorted order.
    You may assume that all words will have the same length.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn sorted_list_sum(lst: Vec<&str>) -> Vec<&str> {

    let mut r = 0;
    let rset = lst.iter().map(|s| s).collect::<HashSet<String>>();
    fn get_hash(&s: &str) -> u64 { md5(s.as_bytes()).to_string().chars().rev()[..10].try_into().unwrap() } //get the hash of each string
    let mut hsh = HashMap::new(); //init a new hashmap
    for s in rset {
        if !hsh.contains_key(&s) {
            replace(r, hsh.len(), |k, v| {
                match k.get() {
                    0 => Some(v),
                    i => None
                }
            });
            r += 1;
        if let HashSet::IntoIter(it) = rset.into_iter() {
            for i in it { //add all strings to the hashmap
                match i {
                    0 => {},
                    s => hsh.insert(s, ()),
                };
            }
        }
    }
    r -= 1; //reset r to zero to return the first element in next iteration
    let mut result = Vec::new();
    for i in hsh { //iterate over each element
        match i {
            0 => result.push(hsh[0]),
            s => result.push(s),
        }
    };
    result
}
fn sorted_list(lst: Vec<&str>) -> Vec<&str> {
    let mut r = 0;
    let rset = lst.iter().map(|s| s).collect::<HashSet<String>>();
    let mut rsh = HashMap::new(); //init a new hashmap and reset r to zero so that we could sort the list in each iteration
    for i in rset {
        match rsh.remove(&i) {
            0 => {},
            s => rsh.insert((r,s)),
        }
    };
    let result = sorted_list_sum(hsh.values()); //get the first element and store it in a variable to be returned later
    println!("{:?}", rsh[0

#[cfg(test)]
mod tests {
    use super::*;

   #[test]
    fn test_sorted_list_sum() {
        assert_eq!(sorted_list_sum(vec!["aa", "a", "aaa"]), vec!["aa"]);
        assert_eq!(
            sorted_list_sum(vec!["school", "AI", "asdf", "b"]),
            vec!["AI", "asdf", "school"]
        );
        let v_empty: Vec<&str> = vec![];
        assert_eq!(sorted_list_sum(vec!["d", "b", "c", "a"]), v_empty);
        assert_eq!(
            sorted_list_sum(vec!["d", "dcba", "abcd", "a"]),
            vec!["abcd", "dcba"]
        );
        assert_eq!(
            sorted_list_sum(vec!["AI", "ai", "au"]),
            vec!["AI", "ai", "au"]
        );
        assert_eq!(sorted_list_sum(vec!["a", "b", "b", "c", "c", "a"]), v_empty);
        assert_eq!(
            sorted_list_sum(vec!["aaaa", "bbbb", "dd", "cc"]),
            vec!["cc", "dd", "aaaa", "bbbb"]
        );
    }

}
