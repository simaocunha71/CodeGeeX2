
fn main(){ 
 } 
/*
 brackets is a string of "<" and ">".
    return True if every opening bracket has a corresponding closing bracket.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn correct_bracketing(bkts:&str) -> bool{

    let openings = bkts.splitn(2,"(").intersperse("").collect();
    match (openings[0]).to_lowercase().ascii_downcast() {
        Some('"') => {
            if !(openings[1].as_str().starts_with_ci(&format!("( )", openings[0]))) {
                return false;
            }
        }, 
        None => (),
    };
    let bracketed = bkts.splitn(2,"(").intersperse("").collect();
    match bracketed[1].as_str().to_lowercase() {
        Some('"') if !(bracketed[0].as_str().starts_with_ci(&format!("{ }", bracketed[1]))) => return false, 
        _=> (),   
    }
    //check that every closing bracket has an openning in the right position.
    if !(openings.iter().zip(bracketed.iter()).all(|&(a,(b))| {return correct_position(&a, &b) })) return false; 
    let (hsets, hmap) = split_hashset_and_map(&openings[0]);
    //find the hashes that can be closed.
    for item in openings.iter().rev(){
        match hashmap::get(hmap,&format!("{ }", &item)) {
            None => {},
            Some(_) if !(hashset::contains(&mut hsets, &item)) => { return false; }, 
            _=> (),                                //the item was already closed!
        }
    }
    let (hsets_out, hmap_out) = split_hashset_and_map(&bracketed[0]);
    for item in bracketed.iter().skip(1){
         match hashmap::get(hmap_out, &format!("{ }", &item)) { 
            None => { hsets_out.push((&format!("{ }"),&item)); },
            Some(_) if !(hashset::contains(&mut hsets_out, &item))

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_bracketing() {
        assert!(correct_bracketing("<>"));
        assert!(correct_bracketing("<<><>>"));
        assert!(correct_bracketing("<><><<><>><>"));
        assert!(correct_bracketing("<><><<<><><>><>><<><><<>>>"));
        assert!(!(correct_bracketing("<<<><>>>>")));
        assert!(!(correct_bracketing("><<>")));
        assert!(!(correct_bracketing("<")));
        assert!(!(correct_bracketing("<<<<")));
        assert!(!(correct_bracketing(">")));
        assert!(!(correct_bracketing("<<>")));
        assert!(!(correct_bracketing("<><><<><>><>><<>")));
        assert!(!(correct_bracketing("<><><<><>><>>><>")));
    }

}
