
fn main(){ 
 } 
/*
 For a given string, flip lowercase characters to uppercase and uppercase to lowercase.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

pub fn flip_case(string: String) -> String{

    let (uppercase, lowercase): (String, String); 
        = match string.chars().rev()[..] {
                [h, ..] if h.is_ascii() => (vec![h].into(), vec![]),
                _                         => (vec![string[0]].into(), vec![string[127]..])
        };
    let cased: Vec<char> = uppercase.iter().chain(lowercase).cloned().collect();
    replace(&cased, 
             |&x| if *max_str!(x) > 0 && x == 'X' { 
                     'x'.into()
                 } else { 
                     x.into(), 
              })
}
fn max_str<T: AsMut, S>(v: T) -> T where S:AsRef<[u8]>,
    for<'a> 
        T 
        : AsMut
            + Clone + FnOnce(&'a)
            + Copy
+ 'static 
{
    let s = v.as_ref(); // this works because we only pass T to a function with type `S` as an &str or String which is why we can do this
                      // if we passed in a Vec<u8> it would fail here since the type `T` is not compatible with the type `S` - and
                      // this would be even more noticeable when `v` is a reference - which is why we clone the value before passing 
                      // it to the function to get rid of any references that might point at mutable data.
    AsRef::<S>::as_ref(s).max() // the max() call could technically return a reference but in this case its an empty slice so we can just pass that into our type T which is why this works with &str also.
}
#[cfg(test)] 
mod tests {
    use super::*;
    #[test]
    fn test_case() {
        let str = "lLllo0-123"; // string which starts out lowercase and flips to upper case afterwards
        
        let cased: Vec<char> = flip_

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_flip_case() {
        assert!(flip_case("".to_string()) == "".to_string());
        assert!(flip_case("Hello!".to_string()) == "hELLO!".to_string());
        assert!(
            flip_case("These violent delights have violent ends".to_string())
                == "tHESE VIOLENT DELIGHTS HAVE VIOLENT ENDS".to_string()
        );
    }

}
