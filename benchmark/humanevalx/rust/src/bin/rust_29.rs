
fn main(){ 
 } 
/*
 Filter an input list of strings only for ones that start with a given prefix.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn filter_by_prefix(strings:Vec<String>, prefix:String)-> Vec<String>{

    let prefix_regex = Regex::new(&format!("^[{}]+","".to_string())).unwrap();
//    println!("filter-by-prefix");
//    println!("{}", strings.len());
//    println!("{}", prefix);
    let set:HashSet<String> = strings.iter().map(str::parse).collect();
    let mut ret = HashSet::new();
    for s in set.iter() {
        let len_s = s.chars().count();
        
//        println!("{}", s);
//        println!("{}", len_s);
        if prefix.len() == 0 || prefix.starts_with(s) {
            ret.insert(s);
        }else{ // prefix is not empty, but not starting with the string
            
           let r:Option<u8> = prefix.chars().collect::<Vec<u8>>()
                  .find(|&x| x == len_s).map(&-*).unwrap();
            
            // prefix and str start with same len char, so they must begin with the same char
//            println!("{}", r);
            
           if let Some(c) = r {
                ret.insert(*s[len_s - c..])
           }else{
               ret.insert(&format!("{}{}", prefix, s))
           }
        }
    }
    return ret; // return a set of strings that satisfies the input condition
}
// Test your program!
#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::fmt::Debug;
    #[test]
    fn test_filter_by_prefix(){
        let s = "abcd"
            .iter()
            .map(str::parse)
            .collect();
        
        // print!("hello", s);
        println!("{}", filter_by_prefix(&s, "a"));
    }
}
// Test your program!
#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::fmt::Debug;

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_filter_by_prefix() {
        let v_empty: Vec<String> = vec![];
        assert!(filter_by_prefix(vec![], "john".to_string()) == v_empty);
        assert!(
            filter_by_prefix(
                vec![
                    "xxx".to_string(),
                    "asd".to_string(),
                    "xxy".to_string(),
                    "john doe".to_string(),
                    "xxxAAA".to_string(),
                    "xxx".to_string()
                ],
                "xxx".to_string()
            ) == vec!["xxx", "xxxAAA", "xxx"]
        );
    }


}
