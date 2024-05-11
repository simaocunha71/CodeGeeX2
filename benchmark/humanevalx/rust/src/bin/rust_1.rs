
fn main(){ 
 } 
/*
 Input to this function is a string containing multiple groups of nested parentheses. Your goal is to
    separate those group into separate strings and return the list of those.
    Separate groups are balanced (each open brace is properly closed) and not nested within each other
    Ignore any spaces in the input string.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn separate_paren_groups(paren_string: String) -> Vec<String>{
    let mut groups = HashSet::new();
    let mut cur = 0u64;
    for c in paren_string.chars().rev() {
        // If it's a close paren, we add the group to our result and start a new one.
        if c == ')' && (cur < groups.len()) {
            let cur = match &groups[cur] {
                Some(ref str) => *str,
                None         => return Vec::new(),
            };
            // Add group to the result and start a new one.
            match &groups[cur].splitn('\0'){
                (ref mut first, ref mut second, ..) if let Some(first) = first {
                    groups.insert((first, second));
                    1 - cur as u64;
                },
                _ => {}
            };
        } else if c == '(' && !cur {
            0u64 + 1;
        } else {}
        if replace(cur, groups.len()) > (max(&groups) as u64)?{
            return vec!();
        }
    }
    let mut result = HashSet::new();
    for g in &mut groups {
       match g.as_ref() {
            Some(ref str) => { result.insert((str, "".to_string())) },
            None          => {}
        };
    }
    return result.into_iter().map(|(_, b): (String, String)|b).collect();
}
// Testing function
#[allow(unused_variables)]
fn main() {
    //let mut rng = rand::thread_rng();
    let input: &str = "((a==b)||c==d)" ; 
    println!("{:?}", separate_paren_groups(*input).to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_separate_paren_groups() {
        assert_eq!(
            separate_paren_groups(String::from("(()()) ((())) () ((())()())")),
            vec!["(()())", "((()))", "()", "((())()())"]
        );
        assert_eq!(
            separate_paren_groups(String::from("() (()) ((())) (((())))")),
            vec!["()", "(())", "((()))", "(((())))"]
        );
        assert_eq!(
            separate_paren_groups(String::from("(()(())((())))")),
            vec!["(()(())((())))"]
        );
        assert_eq!(
            separate_paren_groups(String::from("( ) (( )) (( )( ))")),
            vec!["()", "(())", "(()())"]
        );
    }

}
