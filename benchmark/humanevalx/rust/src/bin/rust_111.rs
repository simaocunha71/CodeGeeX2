
fn main(){ 
 } 
/*
Given a string representing a space separated lowercase letters, return a dictionary
    of the letter with the most repetition and containing the corresponding count.
    If several letters have the same occurrence, return all of them.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn histogram(test:&str) -> HashMap<char, i32>{

    let re = Regex::new(r"[a-z]+").unwrap();
    println!("{}", test);
    let chunks:Vec<_>=re.finds_all("{test}").map(|chunk| chunk).collect();
    println!("{}", chunks);
    if !chunks.is_empty() && re.captures(String::from(&test)).unwrap().0.len() > 1 {
        // if one char is repeated more than once, we need to track both instances of it
        let mut hmap:HashMap<char, i32> = HashMap::new();
        
        chunks.iter().for_each(|c| {
            match c{
                "a" => {histogram("aa")},
                "b" => {histogram("ba")}
                _ => (),
            }
        });
        *hmap |= hmap["a"].and_then(max) + 1; // add a and b to both counts
        chunks.iter().for_each(|c| {
            match c{
                "aa" => (*histogram)["b"],
                "ba" =>(*histogram)["a"]+1,
                _ => (),
            }
        })
        .collect();
        return hmap;
    } else  {
         // else we can just increment the count of each char in the test by one
        let mut chunks = re.captures(String::from(&test)).unwrap().0.iter()
            .cloned()
            .collect();
        *hmap |= chunks.clone();
        // return map
        
    }
}
fn main(){
    let test_str = String::from("aaa");
    let histogram:HashMap<char, i32> = histogram(&test);
    for (key, value) in histogram{
        println!("{} has {} times", key.to_string(), value);
    }
}*/
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_b_and_c() -> bool {
        let mut hmap:HashMap<char, i

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_histogram() {
        assert!(histogram("a b b a") == HashMap::from([('a', 2), ('b', 2)]));
        assert!(histogram("a b c a b") == HashMap::from([('a', 2), ('b', 2)]));
        assert!(
            histogram("a b c d g")
                == HashMap::from([('a', 1), ('b', 1), ('c', 1), ('d', 1), ('g', 1)])
        );
        assert!(histogram("r t g") == HashMap::from([('r', 1), ('t', 1), ('g', 1)]));
        assert!(histogram("b b b b a") == HashMap::from([('b', 4)]));
        assert!(histogram("r t g") == HashMap::from([('r', 1), ('t', 1), ('g', 1)]));
        assert!(histogram("") == HashMap::new());
        assert!(histogram("a") == HashMap::from([(('a', 1))]));
    }

}
