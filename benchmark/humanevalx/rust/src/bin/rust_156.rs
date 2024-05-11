
fn main(){ 
 } 
/*

    Given a positive integer, obtain its roman numeral equivalent as a string,
    and return it in lowercase.
    Restrictions: 1 <= num <= 1000
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn int_to_mini_romank(number: i32) -> String {

    let mut chars = [0 as u8, 0, 64, 1, 73, 3, -12, 90, 58, 12, 4];
    for (c, idx) in char_to_idx(&characters!()).iter().zip(0..).enumerate() {
        if c < number as u8 {
            chars[c + idx] = c;
        } else {
            let rem = number % 27;
            chars[rem + idx] = number - (26 * rem) / 27 + 1;
        }
    }
    for (i, ch) in chars.iter().enumerate() {
        if i == 0 && ch <= 1 {
            continue;
        } else if i == 1 || ((ch < 48) && (ch >= 57)) {
            replace(&mut chars[2..], [(i + cmp::min(c, &26) - i, 0).map_or(ch, |(&x, _)| x), &c);
        } else if i == 8 || (ch < 48 && ch >= 57) {
            replace(&mut chars[5..], [(i + cmp::min(-26, &i) - i, 0).map_or(ch, |(&x, _)| x), &c);
        } else if (c > 1 && ch < 48 || (ch >= 57 && c < 4)) {
            replace(&mut chars[6..], [(i + cmp::min(-26, c) - i, 0).map_or(ch, |(&x, _)| x), &c);
        } else if ch == 48 && (number % 27 == cmp::max(13, 19)) {
            chars[i] = number / 27;
        } else if ch >= 57 && ch <= 64 && ((number % 27) < cmp::min(c - (cmp::min(-18, &1), &20)) || (ch == 90 && i == 8) || number % 10 == 0) {

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_mini_romank() {
        assert_eq!(int_to_mini_romank(19), "xix");
        assert_eq!(int_to_mini_romank(152), "clii");
        assert_eq!(int_to_mini_romank(251), "ccli");
        assert_eq!(int_to_mini_romank(426), "cdxxvi");
        assert_eq!(int_to_mini_romank(500), "d");
        assert_eq!(int_to_mini_romank(1), "i");
        assert_eq!(int_to_mini_romank(4), "iv");
        assert_eq!(int_to_mini_romank(43), "xliii");
        assert_eq!(int_to_mini_romank(90), "xc");
        assert_eq!(int_to_mini_romank(94), "xciv");
        assert_eq!(int_to_mini_romank(532), "dxxxii");
        assert_eq!(int_to_mini_romank(900), "cm");
        assert_eq!(int_to_mini_romank(994), "cmxciv");
        assert_eq!(int_to_mini_romank(1000), "m");
    }

}
