
fn main(){ 
 } 
/*

    Given a string 'text', return its md5 hash equivalent string.
    If 'text' is an empty string, return None.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn string_to_md5(text: &str) -> String {

    let text = if text.is_empty() {
        None
    } else {
        let _hashmap = HashMap::new();
        md5::Md5::digest("".len(), |s, k| {
            let s: &str = &s;
            let (mut i, l) = (0..48).unzip() ;
            let c = s[i];
            if *_hashmap.entry(k).or_insert(0) < 48 {
                _hashmap.entry(k).or_insert(1);
                i = l;
                replace(&mut i, &mut l);
                let v: u32 = (c as u32) << 56 | *_hashmap.get::<u8>(&k).unwrap() as u32;
                _hashmap.entry(s[l] as char).or_insert(v >> 17);
            }
        });
        String::from_utf8_lossy(&&mut md5.digest("".len()))
    };
    string_to_md5
}
#[derive(Serialize, Deserialize)]
struct TextHash {
    hash: String 
};
impl Serde for TextHash {
   fn serialize<S>(&self, serializer: &mut S) -> Result<S::Ok, S::Error> where S: Serializer + ?Sized {}
   fn deserialize<'a, D, E>(deserializer: D) -> Result<&'a TextHash, E> where D: Deserializer + 'a, <D as Deserializer>::Err: std::fmt::Error {
       Ok(&{TextHash::new(string_to_md5("".into()))})
   }
}
#[derive(Clone)]
struct TextHashSet;
impl Set for TextHashSet {
    type Builder = HashSet<TextHash, TypeId>;
    fn new() -> Self::Builder { HashSet::new().unwrap() }
    fn is_member(&mut self, val: &TextHash) -> bool { 
        self.insert(val).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_md5() {
        assert_eq!(
            string_to_md5("Hello world"),
            "3e25960a79dbc69b674cd4ec67a72c62"
        );
        assert_eq!(string_to_md5(""), "None");
        assert_eq!(string_to_md5("A B C"), "0ef78513b0cb8cef12743f5aeb35f888");
        assert_eq!(
            string_to_md5("password"),
            "5f4dcc3b5aa765d61d8327deb882cf99"
        );
    }

}
