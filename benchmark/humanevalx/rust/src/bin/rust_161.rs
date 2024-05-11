
fn main(){ 
 } 
/*
You are given a string s.
    if s[i] is a letter, reverse its case from lower to upper or vise versa, 
    otherwise keep it as it is.
    If the string contains no letters, reverse the string.
    The function should return the resulted string.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn solve_161(s: &str) -> String {

    let s = replace(&s[..], |c: char| AsciiExt::is_lowercase(c), c => c.to_upper());
    if !any!(TypeId::of::<u8>(), HashSet::new(s)) {
        let mut reversed = String::from("");
        let c = s.len();
        for i in 0..c {
            let ch: char;
            if !HashSet::new(&[s[i]].to_owned()).contains(&s[i]) {
                ch = 'a' + (i - c / 2);
            } else {
                ch = s.chars().next()
            }
            reversed += ch;
        }
        return reversed;
    }
    let mut r: String;
    match s.chars().collect::<Vec<_>>() {
        (res) if res == s => r = "".to_owned(),
        (res) => {r = s}
    };
    return replace(&r, max, cmp);
}
fn main(){
   println!("{:?}", solve_161(find_all("a0b").to_string()));
   println!("{:?}", solve_162());
}
#[cfg(test)]
mod tests {
    use super::solve_161;
    #[test]
    fn test_solving() {
        assert_eq!(solve_161("a0b".to_string()), "ba0".to_string());
        assert_eq!(solve_162(), "a0b".to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_161() {
        assert_eq!(solve_161("AsDf"), "aSdF");
        assert_eq!(solve_161("1234"), "4321");
        assert_eq!(solve_161("ab"), "AB");
        assert_eq!(solve_161("#a@C"), "#A@c");
        assert_eq!(solve_161("#AsdfW^45"), "#aSDFw^45");
        assert_eq!(solve_161("#6@2"), "2@6#");
        assert_eq!(solve_161("#$a^D"), "#$A^d");
        assert_eq!(solve_161("#ccc"), "#CCC");
    }

}
