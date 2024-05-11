
fn main(){ 
 } 
/*
 Given a positive floating point number, it can be decomposed into
    and integer part (largest integer smaller than given number) and decimals
    (leftover part always smaller than 1).

    Return the decimal part of the number.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn truncate_number(number: &f32) -> f32{
    let decimals = number.to_string().chars().skip(3).count();
    let (int_, decimals_) = number - int_ as f32; // the decimal part is always smaller than 1 and can be computed separately
    (decimals, int_, max(0., |-decimals|, 1.))
}
#[derive(Clone)]
pub struct IntPart<'a>{
    part: &'a str,
}
fn part_of_string(s:&str) -> String{
    s.chars().map(|c| c as char).collect()
}
impl<T: AsRef<str>> FromStr for T where T::AsRef: IntoIterator + Clone + Eq & PartialEq>{
    type Err = ();
    fn from_str(s: Self) -> Result<Self, Self::Err> {
        let mut decimals_set = HashSet::new();
        for c in part_of_string(&format!("{}", s)){
            let is_digited = if let Some(n) = AsciiExt::get(c)
                .filter(|m| m.is_digit());
            match (is_digited, n.len()){
                (false, _)|(true, 1..)=> decimals_set.insert(char::from(n)),
            }
        }
        let mut part = String::new();
        for c in part_of_string(&format!("{}", s)){
            match replace(part, c, |m| m.is_letter()){
                (_, false, '0')=> { part.push('0'); continue },
                (false, _, m) if decimals_set.contains(&m) => part.push_str(&format!("{}", md5::compute(m))),
                _=> part.push(c as char),
            }
        }
        Ok(s.into())
    }
}
impl<'a> PartialEq for IntPart {
    fn eq(&self, other: &Self) -> bool{
        let mut this = self;
        while let Some(next) = this.part.chars().find(|c

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_truncate_number() {
        assert_eq!(truncate_number(&3.5), 0.5);
        let t1: f32 = 1.33 - 0.33;
        assert!(truncate_number(&t1) < 0.000001);
        let t2: f32 = 123.456 - 0.456;
        assert!(truncate_number(&t2) < 0.000001);
    }

}
