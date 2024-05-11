
fn main(){ 
 } 
/*
 Input to this function is a string represented multiple groups for nested parentheses separated by spaces.
    For each of the group, output the deepest level of nesting of parentheses.
    E.g. (()()) has maximum two levels of nesting while ((())) has three.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn parse_nested_parens(paren_string:String) -> Vec<i32>{
    let paren_string = replace(&paren_string, &"(\" => " => 1); //remove the start and ending parentheses for easy processing. 
    let mut paren_numbers:Vec<i32> = vec![]; //initializing our numbers vector with a value of [] to make room for our nested numbers.
    let mut paren_levels: HashSet<i32> = HashSet::new(); //initializing our parsed paren levels with an empty set so we don't double count anything.
    let pattern = Regex::new("^[0-9]+$").unwrap(); //creating a pattern to search for the numbers in our string, which will be in the form of "1 2" in our end result.
    let mut group_index:usize;//this number will eventually represent the index position that we are on and can be counted up to (as an integer) in order to give us the current maximum nested parentheses level.
    //searches through the string for nested sets of numbers, then uses those numbers as our parsed nesting levels to determine what the deepest parsed set was.
    let paren_string = pattern.finds(&paren_string).map(|number|{//let's get all of the numbers that were searched out by the regex and store them in a new vector.
        for number in pattern.finds(&paren_string){
            paren_numbers.push(number);//this stores our parsed numbers into our new vector "paren_numbers".
            if group_index == 0{//if this is the first time that we are parsing a number, then set the level of nesting to be equal to the current set of nested numbers we have found.
                let levels = paren_numbers.iter().count();//taking in our vector "paren_numbers", iterating over it and counting how many times the same value is present.
                paren_levels.extend(levels); //then adding those count values to our new set of nested levels, which will be used as a reference on where we left off from the last time we parsed this string to make sure we only store unique levels and don't count something more than once for our final output.
            }else{//otherwise we will just simply add onto our level counter

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_parse_nested_parens() {
        assert!(
            parse_nested_parens(String::from("(()()) ((())) () ((())()())")) == vec![2, 3, 1, 3]
        );
        assert!(parse_nested_parens(String::from("() (()) ((())) (((())))")) == vec![1, 2, 3, 4]);
        assert!(parse_nested_parens(String::from("(()(())((())))")) == vec![4]);
    }

}
