
fn main(){ 
 } 
/*
 brackets is a string of "(" and ")".
    return True if every opening bracket has a corresponding closing bracket.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn correct_bracketing_parenthesis(bkts:&str) -> bool{

    let brackets = bkts.splitn(2, "()");
   //let brackets = bkts.parse::<Vec<char>>().expect("Unable to parse string into array of chars!");
    //let brackets_str = brackets[0];
    if let Ok(brackets) = brackets.chunks(2) {
        for (i, bracket_pair) in brackets.iter()
            .zip(brackets.iter().skip(1).cloned())
            .enumerate()
        {
          //let pair:&[char; 2] = &bracket_pair;
            match i {
                0 => if let Ok(s) = bracket_pair.chars().map(|c| c.to_string()).collect(),
                1 => if let Ok(s) = bracket_pair.iter()
                .copied() //clone the iterator!
                .fold(&mut String::with_capacity(brackets[0].len()), |acc, current| { //combine acc and current with the join function, taking (str) as parameter instead of T
                    let s1 = &acc;
                    if let Ok(s2) = current.clone().into_iter()
                        .map(|c| c).collect(), then let next_str =
                            format!("{}", s1).to_string();
                    else, return (format!("{}", s1)).to_string();
                    //let s2:String = "".into();
                    if match brackets.len() {
                        0 => true,
                        _ => false
                    } {//match brackets.len {
                     let str = match next_str.chars().map(|c| c).collect(), next_str{
                            0 => "{}.{}", // "().
                            n => format!("{}".chars().collect::<String>())
                        }
                    ;
                    println!("BRAK: {:?} {:?} {:?}", str, brackets[0], brackets[1]);
                    if let Ok(expected_str) = expect(&str), then return true;
                    
                match pairs(brackets[0] & ".".to_string(), bracket_pair

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_bracketing_parenthesis() {
        assert!(correct_bracketing_parenthesis("()"));
        assert!(correct_bracketing_parenthesis("(()())"));
        assert!(correct_bracketing_parenthesis("()()(()())()"));
        assert!(correct_bracketing_parenthesis("()()((()()())())(()()(()))"));
        assert!(!(correct_bracketing_parenthesis("((()())))")));
        assert!(!(correct_bracketing_parenthesis(")(()")));
        assert!(!(correct_bracketing_parenthesis("(")));
        assert!(!(correct_bracketing_parenthesis("((((")));
        assert!(!(correct_bracketing_parenthesis(")")));
        assert!(!(correct_bracketing_parenthesis("(()")));
        assert!(!(correct_bracketing_parenthesis("()()(()())())(()")));
        assert!(!(correct_bracketing_parenthesis("()()(()())()))()")));
    }

}
