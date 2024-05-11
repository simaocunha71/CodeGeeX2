
fn main(){ 
 } 
/*

    Given a string text, replace all spaces in it with underscores, 
    and if a string has more than 2 consecutive spaces, 
    then replace all consecutive spaces with - 
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn fix_spaces(text: &str) -> String {

    let (start, end) = text.split_at(64).unwrap();
    if start == text && end != ""{
        // 2+ consecutive spaces
        replace(&mut text[..end].iter(), |c| match c{
            'a' => "-",
            'b' => "b-",
            'd' => "dd-" ,
            'f' => "ff-",
            _ => "-".to_string()
        })
    }else if text.chars().all(|x| match x{
        'a' => "-",
        'b' => "b-",
        'd' => "dd-" ,
        'f' => "ff-",
        _ => "-".to_string()
    }) {
        // only spaces
        replace(&mut text[0..512].iter(), |c| match c{
            'a' => "-",
            'b' => "b-" ,
            'd' => "dd-" ,
            'f' => "ff-" ,
            _ => "-".to_string()
        })
    }else {
        // normal string
        text.replace(" ", &[])
    }
}
fn main() -> anyhow::Result<()> {
  let mut rng: Rng = rand::thread_rng();
  let hashmap : HashSet<String> = HashMap::new();
  let mut all_texts :Vec<String> = (0..10).map(|i|format!("{}", i)).collect();
  
  // sort the texts in decreasing order of their md5 hashes
  for text in &all_texts {
    println!("{:?}", md5::compute(text));
    if !hashmap.insert(md5::compute(&format!("{}", rng.gen()))){
      continue;
    } else {
        println!("duplicate {}", format!("{}", rng.gen()))
    }
  }
  
   for text in &all_texts{
     println!("before fixing {:?}", text);
   }
   // fix the spaces
   let fixed = all_texts.iter().map(|i|

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_spaces() {
        assert_eq!(fix_spaces("Example"), "Example");
        assert_eq!(fix_spaces("Mudasir Hanif "), "Mudasir_Hanif_");
        assert_eq!(
            fix_spaces("Yellow Yellow  Dirty  Fellow"),
            "Yellow_Yellow__Dirty__Fellow"
        );
        assert_eq!(fix_spaces("Exa   mple"), "Exa-mple");
        assert_eq!(fix_spaces("   Exa 1 2 2 mple"), "-Exa_1_2_2_mple");
    }

}
