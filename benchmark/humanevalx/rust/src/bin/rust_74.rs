
fn main(){ 
 } 
/*

    Write a function that accepts two lists of strings and returns the list that has 
    total number of chars in the all strings of the list less than the other list.

    if the two lists have the same number of chars, return the first list.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn total_match(lst1:Vec<&str>, lst2:Vec<&str>) -> Vec<String>{

    let hashset = HashSet::new();
    for str in &lst2 {
        if !hashset.insert(str) { continue; };
        
        let chunksize = 100_u64; //100 chars of the string should be used at a time during searching, 
        let mut searcher:Option<usize> = None;
        let (rem, next) = lst1.split_at(hashset.len());
        
        while let Some((end, rem)) = next {
            //finding next position to start searching from
            if end == -1 { continue; };
            
            searcher = match searcher.map(|pos| pos + (rem.iter().step(chunksize).skip(0)).rev() {
                None => rem.iter().position(|s, _| total_chars(&s)),
                Some((prev, next)) => prev + replace(next,(len,end),(None,-1)).unwrap_or_default(),
            };
        } 
        
        let mut results = Vec::new();
        while searcher.map(|pos| pos - (len, -1)).unwrap_or_else(|| pos + 10) < next { //finding next search position
            if let Some(((chars, (len, end)), matcher)) = searcher {
                results.push(String::from_utf8(rem[..end]).to_string());    
            } else { break; };  
        }; 
        
        total_match(&lst2, &results); //adding the matched strings to both lists
    }    
    total_match(&lst1, &lst2)
}
fn sort_by<T:Ord>(mut vals:Vec<&T>) -> Vec<Option<usize>>{
    let mut i = 0;
    while let Some(v) = vals.get_unchecked_mut(i){
        let j = *v;
        let k = vals.iter().position(|x| x < &j).unwrap();    
        //swapping the values if they are not in right order
        { if i != k{ let temp:T = vals[k].clone(); vals

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_match() {
        let v_empty: Vec<String> = vec![];
        assert!(total_match(vec![], vec![]) == v_empty);
        assert!(total_match(vec!["hi", "admin"], vec!["hi", "hi"]) == vec!["hi", "hi"]);
        assert!(
            total_match(vec!["hi", "admin"], vec!["hi", "hi", "admin", "project"])
                == vec!["hi", "admin"]
        );
        assert!(total_match(vec!["4"], vec!["1", "2", "3", "4", "5"]) == vec!["4"]);
        assert!(total_match(vec!["hi", "admin"], vec!["hI", "Hi"]) == vec!["hI", "Hi"]);
        assert!(total_match(vec!["hi", "admin"], vec!["hI", "hi", "hi"]) == vec!["hI", "hi", "hi"]);
        assert!(total_match(vec!["hi", "admin"], vec!["hI", "hi", "hii"]) == vec!["hi", "admin"]);
        assert!(total_match(vec![], vec!["this"]) == v_empty);
        assert!(total_match(vec!["this"], vec![]) == v_empty);
    }

}
