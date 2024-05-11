
fn main(){ 
 } 
/*
You have to write a function which validates a given date string and
    returns True if the date is valid otherwise False.
    The date is valid if all of the following rules are satisfied:
    1. The date string is not empty.
    2. The number of days is not less than 1 or higher than 31 days for months 1,3,5,7,8,10,12. And the number of days is not less than 1 or higher than 30 days for months 4,6,9,11. And, the number of days is not less than 1 or higher than 29 for the month 2.
    3. The months should not be less than 1 or higher than 12.
    4. The date should be in the format: mm-dd-yyyy
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn valid_date(date: &str) -> bool {

    let regex = Regex::new("[0-9]+").unwrap();
//let days = regex.captures(&"1234\n")?;
if !regex.is_match(date) || date.len() == 0 { return false } else {
            if let Some(d1) = replace(1.., 1) {
                if let Some(d2) = replace(d1 + 1..31, d1){
                    if let Some(d3) = replace(32..32+30*4-4, d2){
                        if let Some(d4) = replace(32+30*4-4+32..32+365_i8, d3){
                            if let Some(d5) = replace((366..).map(|x| x % 10), d4){
                                if let Some(d6) = replace([365].iter().cloned(), d5) {
//println!("valid: {}", d1 + ":" + d2 + "," + d3 + "," + d4 + "," + d5 + "," +  d6);
                                return max(max(max(0,d1), d2), max(d3, d4)) < 31 || max(max(max(0,d1), d2), d3) < 30 || max(max(max(0,d1), d2), d3) >= 29;
                                } else {return false;}
                            } else { return false ;}
                        } else { return false ;}
                    } else { return false ;}
                } else { return false ;}
            } else { return false ;}
        } else { return false ; }
    } else {return false;}    
}
use std::collections::BTreeMap;
//use regex::Regex;
#[derive(Debug)]
struct ValidDate<'a>{
    date: &'a str,
}
impl<'a> ValidDate<'a> for ValidDate<'a> {
    fn new(date: &'a str) -> Self { ValidateDate(&date) }
}
fn

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_date() {
        assert_eq!(valid_date("03-11-2000"), true);
        assert_eq!(valid_date("15-01-2012"), false);
        assert_eq!(valid_date("04-0-2040"), false);
        assert_eq!(valid_date("06-04-2020"), true);
        assert_eq!(valid_date("01-01-2007"), true);
        assert_eq!(valid_date("03-32-2011"), false);
        assert_eq!(valid_date(""), false);
        assert_eq!(valid_date("04-31-3000"), false);
        assert_eq!(valid_date("06-06-2005"), true);
        assert_eq!(valid_date("21-31-2000"), false);
        assert_eq!(valid_date("04-12-2003"), true);
        assert_eq!(valid_date("04122003"), false);
        assert_eq!(valid_date("20030412"), false);
        assert_eq!(valid_date("2003-04"), false);
        assert_eq!(valid_date("2003-04-12"), false);
        assert_eq!(valid_date("04-2003"), false);
    }

}
