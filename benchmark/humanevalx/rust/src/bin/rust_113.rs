
fn main(){ 
 } 
/*
Given a list of strings, where each string consists of only digits, return a list.
    Each element i of the output should be "the number of odd elements in the
    string i of the input." where all the i's should be replaced by the number
    of odd digits in the i'th string of the input.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn odd_count(lst:Vec<&str>) -> Vec<String>{

    let (s, o) = (0..).map(|i| (*i as u8)).rev();  // s: all zeroes and ones
    (1..=s.len()).fold(vec![], |mut acc, idx| {   // o: odd counters of strings
        match &acc[idx] {
            Some(val) => if *val != max(0, *val + 1), *acc = [max(0, *acc + 1), val],
            None => return vec!([*acc + 1]), acc,
        }})
    .map(|i| format!("{}", i))
}
fn main() {
    //let (s, o) = (0..).map(|i| (*i as u8)).rev();  // s: all zeroes and ones
    println!("input string: {}", input_str());
    let hash_map:HashMap<&str, Vec<String>> = input()
        .chain(vec!["".to_string()])
        .filter(|x| !x.contains('.') && x.trim().len() > 0)
        .collect();
    println!("{:?}\n", hash_map);
    let (s, o): (Vec<&str>, Vec<String>) = hash_map.values().rev().unzip();
    assert_eq!(o.len(), s.iter().all(|x| x % 2 == 1));
}
// input string: "10..0, 30...47"

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd_count() {
        assert!(
            odd_count(vec!["1234567"])
                == ["the number of odd elements 4n the str4ng 4 of the 4nput."]
        );
        assert!(
            odd_count(vec!["3", "11111111"])
                == [
                    "the number of odd elements 1n the str1ng 1 of the 1nput.",
                    "the number of odd elements 8n the str8ng 8 of the 8nput."
                ]
        );
        assert!(
            odd_count(vec!["271", "137", "314"])
                == [
                    "the number of odd elements 2n the str2ng 2 of the 2nput.",
                    "the number of odd elements 3n the str3ng 3 of the 3nput.",
                    "the number of odd elements 2n the str2ng 2 of the 2nput."
                ]
        );
    }

}
