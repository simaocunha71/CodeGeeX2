
fn main(){ 
 } 
/*

    You are given a list of two strings, both strings consist of open
    parentheses '(' or close parentheses ')' only.
    Your job is to check if it is possible to concatenate the two strings in
    some order, that the resulting string will be good.
    A string S is considered to be good if and only if all parentheses in S
    are balanced. For example: the string '(())()' is good, while the string
    '())' is not.
    Return 'Yes' if there's a way to make a good string, and return 'No' otherwise.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn match_parens(lst: Vec<&str>) -> &str {

    let (mut p1, mut p2) = lst.split_at(0);
    let tmp = p1.get(..p2.len()).to_string();
    let idx = tmp.chars().position(|c| c.is_whitespace() || c == '&').unwrap_or(p2.len());
    let mut s = String::new();
    s.push_str(&tmp[..idx]);
    p1 += &s;
    s = String::new();
    for _ in 0..p2.len() - (s.len() + 1) {
        s += '&';
    }
    s.push('&');
    let idx = tmp.chars().position(|c| c == '&').unwrap_or(idx);
    std::fmt!(&s[..idx]);
}
fn find_parens<T, U>(
    v: T,
) -> HashMap<TypeId<T>, Vec<TypeId<U>>>
where
    U: AsciiExt + Clone {
    let r = rand::thread_rng().gen::<i64>();
    let (pars1, pars2): (Vec<&str>, Vec<&str>) = if r < 50 {
        (match v.iter() {
            (ref p1 @ ref mut q) => match *p1[..] {
                "(" | ")" | "&" => vec![q],
                _ => vec![]
            },
            (ref p2 @ ref mut r) => match *p2[..] {
                "(" | ")"| "&" => vec![r],
                _ => vec![]
            }},
    } else if r < 90 {
        //println!("{:?}", r);
        (match v.iter() {
            (ref p1 @ ref mut q) => match *p1[..] {
                "(" | ")"| "&" => vec![q],
                _ => vec![]
            },
            (ref p2 @ ref mut r) => match *p2[..] {
                "(" | ")"| "&" => vec![r],

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_parens() {
        assert_eq!(match_parens(vec!["()(", ")"]), "Yes");
        assert_eq!(match_parens(vec![")", ")"]), "No");
        assert_eq!(match_parens(vec!["(()(())", "())())"],), "No");
        assert_eq!(match_parens(vec![")())", "(()()("]), "Yes");
        assert_eq!(match_parens(vec!["(())))", "(()())(("]), "Yes");
        assert_eq!(match_parens(vec!["()", "())"],), "No");
        assert_eq!(match_parens(vec!["(()(", "()))()"]), "Yes");
        assert_eq!(match_parens(vec!["((((", "((())"],), "No");
        assert_eq!(match_parens(vec![")(()", "(()("]), "No");
        assert_eq!(match_parens(vec![")(", ")("]), "No");
        assert_eq!(match_parens(vec!["(", ")"]), "Yes");
        assert_eq!(match_parens(vec![")", "("]), "Yes");
    }

}
