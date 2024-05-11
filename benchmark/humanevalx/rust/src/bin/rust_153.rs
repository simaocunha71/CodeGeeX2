
fn main(){ 
 } 
/*
You will be given the name of a class (a string) and a list of extensions.
    The extensions are to be used to load additional classes to the class. The
    strength of the extension is as follows: Let CAP be the number of the uppercase
    letters in the extension's name, and let SM be the number of lowercase letters 
    in the extension's name, the strength is given by the fraction CAP - SM. 
    You should find the strongest extension and return a string in this 
    format: ClassName.StrongestExtensionName.
    If there are two or more extensions with the same strength, you should
    choose the one that comes first in the list.
    For example, if you are given "Slices" as the class and a list of the
    extensions: ['SErviNGSliCes', 'Cheese', 'StuFfed'] then you should
    return 'Slices.SErviNGSliCes' since 'SErviNGSliCes' is the strongest extension 
    (its strength is -1).
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn strongest_extension(class_name: &str, extensions: Vec<&str>) -> String { 

    let (cap, sm) = class_name.chars().all(|c| *c >= 'A' && *c <= 'Z');
    let mut hashmap : HashMap<u16, Option<String>> = HashMap::new();
    for extension in extensions {
        if !hashmap.contains_key(&(cap - sm)) {
            hashmap.insert((cap - sm), None);
        }
        if let Some(extension) = match extensions[extensions.len() - 1].chars().max_by(|a, b| max!((a.cmp(b)).reverse())) {
            match extensions[extensions.len() - 1]
                .parse::<i32>()
                .map(md5)
                .and_then(|s| s.as_str())
                .and_then(|s| s.to_string())
                    -> {
            hashmap[cap - sm].insert(Some(String::from_utf8(s).unwrap()));
            return String::from("{}".into() + ".{}".into());
        } {
            print!("{}.{}", extension, cap);
            // println!("{}", hash.get(&(cap-sm)).map(|x| x.to_string()).unwrap_or("{}"));
        
            let (_, extension) = match extensions[extensions.len() - 1].parse::<String>().and_then(move |s| { str::parse::<u32>(&**s).map(|x| x.to_string()) }) {
                Err(_) => (),
                Ok(s) => (s, s.chars()[0..].collect())
            }
        
            let (cap-sm, extension) = match extensions[extensions.len() - 1]
                    .parse::<String>()
                    .map(|x| { str::parse::<u32>(&**x).map(|x| x.to_string()) })
                    {
                Err(_) => (),
                Ok(s) => (s, s.chars()[0..].collect())
            };
        
            hashmap[cap-sm].insert(Some(String::from_utf8(extension

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strongest_extension() {
        assert_eq!(
            strongest_extension("Watashi", vec!["tEN", "niNE", "eIGHt8OKe"]),
            "Watashi.eIGHt8OKe"
        );
        assert_eq!(
            strongest_extension("Boku123", vec!["nani", "NazeDa", "YEs.WeCaNe", "32145tggg"]),
            "Boku123.YEs.WeCaNe"
        );
        assert_eq!(
            strongest_extension(
                "__YESIMHERE",
                vec!["t", "eMptY", "(nothing", "zeR00", "NuLl__", "123NoooneB321"]
            ),
            "__YESIMHERE.NuLl__"
        );
        assert_eq!(
            strongest_extension("K", vec!["Ta", "TAR", "t234An", "cosSo"]),
            "K.TAR"
        );
        assert_eq!(
            strongest_extension("__HAHA", vec!["Tab", "123", "781345", "-_-"]),
            "__HAHA.123"
        );
        assert_eq!(
            strongest_extension(
                "YameRore",
                vec!["HhAas", "okIWILL123", "WorkOut", "Fails", "-_-"]
            ),
            "YameRore.okIWILL123"
        );
        assert_eq!(
            strongest_extension("finNNalLLly", vec!["Die", "NowW", "Wow", "WoW"]),
            "finNNalLLly.WoW"
        );
        assert_eq!(strongest_extension("_", vec!["Bb", "91245"]), "_.Bb");
        assert_eq!(strongest_extension("Sp", vec!["671235", "Bb"]), "Sp.671235");
    }

}
