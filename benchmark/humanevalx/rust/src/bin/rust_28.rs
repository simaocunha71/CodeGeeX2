
fn main(){ 
 } 
/*
 Concatenate list of strings into a single string
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn concatenate(strings:Vec<String>) -> String{

    let mut str = "".to_string();
    for i in 0..strs.len(){
        if strings[i] != "" {
            let (h1, h2) = match strings[i].as_str() {
                Ok(x) => (x, None),
                Err(x) => (None, Some(x))
            };
            str += &h1;
        }
    }
    return str.to_string();
}
#[derive(Debug)] // We add this to make it easier for us to see errors that occur when using the trait
struct MutableString {
    s: String,
    n: usize,
}
impl Any for MutableString{};
impl TypeId for MutableString{};
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_concat(){
        let rng = rand::thread_rng();
        
        println!("Concatenate the first 10 strings in range: {} and {:?}", &[], concat!(&vec![String::new(); 10][0..10]));    
    }
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_concatenate() {
        assert!(concatenate(vec![]) == "".to_string());
        assert!(
            concatenate(vec!["x".to_string(), "y".to_string(), "z".to_string()])
                == "xyz".to_string()
        );
        assert!(
            concatenate(vec![
                "x".to_string(),
                "y".to_string(),
                "z".to_string(),
                "w".to_string(),
                "k".to_string()
            ]) == "xyzwk".to_string()
        );
    }


}
