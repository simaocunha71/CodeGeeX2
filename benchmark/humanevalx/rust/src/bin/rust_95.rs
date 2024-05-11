
fn main(){ 
 } 
/*

    Given a dictionary, return True if all keys are strings in lower 
    case or all keys are strings in upper case, else return False.
    The function should return False is the given dictionary is empty.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn check_dict_case(dict:HashMap<&str, &str>) -> bool{

    let dict_keys = dict.keys().collect::<Vec<String>>();
        
    if !dict_keys.iter()
        .map(|s| &**s)
        // .map(|s| s.chars())
        // .all(|c: char| AsciiExt::is_uppercase(c)) //all keys are strings in upper case
        .all(|s:&str| max(&**s, &**"".to_string()).cmp() != max("", &"".to_string()) )
    {
        return false;
    }else{
        
        let dict_keys = dict.keys().collect::<Vec<String>>();
        if !dict_keys.iter()
            .map(|s:&str| &**s)
            // .map(|s:&char| AsciiExt::is_uppercase(c)) //all keys are strings in upper case
            .all(|s:&str| &**s == "".to_string().as_ref() || AsciiExt::is_lowercase(s) ) 
        {
            return false;
        }else{    
            let random:String = rand::thread_rng().sample("a..z").to_string();
            
            if !dict_keys.iter()
                .map(|s| &**s)
                // .all(|c: char| AsciiExt::is_uppercase(c)) //all keys are strings in upper case
                // .map(|c| c.to_ascii_lowercase())
                .any(|x:&str| !dict.get(&random).unwrap().contains(&x) ) 
            {
                return false;
            }else{    
                return true;
                        
            }
        }
    }
}
fn check_dict(dict: HashMap<&str, &str>) -> bool{
    let mut dict_keys = dict.keys().to_vec();
    if !dict_keys.iter()
        .map(|s:&str| &**s) // .all(|c: char| AsciiExt::is_uppercase(c)) 
        // .map(|c:&char| c.to_ascii_lowercase())

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_dict_case() {
        assert!(check_dict_case(HashMap::from([("p", "pineapple"), ("b", "banana")])) == true);
        assert!(
            check_dict_case(HashMap::from([
                ("p", "pineapple"),
                ("A", "banana"),
                ("B", "banana")
            ])) == false
        );
        assert!(
            check_dict_case(HashMap::from([
                ("p", "pineapple"),
                ("5", "banana"),
                ("a", "apple")
            ])) == false
        );
        assert!(
            check_dict_case(HashMap::from([
                ("Name", "John"),
                ("Age", "36"),
                ("City", "Houston")
            ])) == false
        );
        assert!(check_dict_case(HashMap::from([("STATE", "NC"), ("ZIP", "12345")])) == true);
        assert!(check_dict_case(HashMap::from([("fruit", "Orange"), ("taste", "Sweet")])) == true);
        assert!(check_dict_case(HashMap::new()) == false);
    }

}
