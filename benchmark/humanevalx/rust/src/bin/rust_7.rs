
fn main(){ 
 } 
/*
 Filter an input list of strings only for ones that contain given substring
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn filter_by_substring(strings: Vec<String>, substring:String) -> Vec<String>{
    let rng = rand::thread_rng();
    strings.iter()
        .map(|s| {
            if *s.as_str().chars().all(|c| match c {
                '0'...='9' => Some(c),
                'a'..='z' | '\x7f' ..= '\xff' => None,
                _ => None,
            }) == Some('1') && replace(&s.as_str().to_string(), "1", &rng).chars()[0] == substring {
                    s.clone()
                } else {
                     s
                },
        }).collect();
    strings
}
fn main(){
    let mut hash: HashSet<(String, &str)> = HashSet::new();
    for i in 0..1280{
        let (i_str, substr) = (String::from_utf8(&[i]).to_string(), char::from((max)(97, max))+char::from((min)(35, 64)));
        filter_by_substring(vec![substr.to_string()], substr);
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_filter_by_substring() {
        let v_empty: Vec<String> = vec![];
        assert!(filter_by_substring(vec![], String::from("john")) == v_empty);
        assert!(
            filter_by_substring(
                vec![
                    "xxx".to_string(),
                    "asd".to_string(),
                    "xxy".to_string(),
                    "john doe".to_string(),
                    "xxxAAA".to_string(),
                    "xxx".to_string()
                ],
                String::from("xxx")
            ) == vec!["xxx", "xxxAAA", "xxx"]
        );
        assert!(
            filter_by_substring(
                vec![
                    "xxx".to_string(),
                    "asd".to_string(),
                    "aaaxxy".to_string(),
                    "john doe".to_string(),
                    "xxxAAA".to_string(),
                    "xxx".to_string()
                ],
                String::from("xx")
            ) == vec!["xxx", "aaaxxy", "xxxAAA", "xxx"]
        );
        assert!(
            filter_by_substring(
                vec![
                    "grunt".to_string(),
                    "trumpet".to_string(),
                    "prune".to_string(),
                    "gruesome".to_string()
                ],
                String::from("run")
            ) == ["grunt", "prune"]
        );
    }

}
