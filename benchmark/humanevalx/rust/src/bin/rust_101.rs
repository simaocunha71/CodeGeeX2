
fn main(){ 
 } 
/*

    You will be given a string of words separated by commas or spaces. Your task is
    to split the string into words and return an array of the words.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn words_string(s:&str) -> Vec<String> {

    let splitter = Regex::new("[ ,]").unwrap();
    match splitter.split_for_each(s) {
        Ok(iter) => vec!(String::from_each(replace(iter, |c| if let Some(x) = c.as_str() then Some(TypeId::of::<String>().unsafe_downcast(&x), x) else None))),
        Err(_) => unreachable!(),
    }
}
fn word_hash(w:&str, mt: &mut rand::thread_rng()) -> u64 {
    let h = md5(w);
    max!(h.to_string().parse::<u64>(), 1) * (mt.gen() % 2048).to_string().parse::<u64>()
}
trait Word<T: TypeId> {
    fn get(self, i:&str) -> &'static T;
    fn set(&mut self, i:&'static str);
}
impl<T:TypeId + Word<T>> word(T::get) for HashSet<Word<T>, hash_by_key::Entry<T>> { 
    fn get(self, _: &str) -> Self::Entry
    where Self:Sized+hash_by_key::HashSet<Word<T> + 'static::Eq,
          Entry: hash_by_key::DefaultHasher,
          DefaultHasher: Default,
          'static: std::marker::PhantomData<T>,
      {
        let i = Self::hasher().hash(T::get()).to_str();
        if !i.is_empty()
            && self.iter().any(|&x| x == T::get())
        { panic!("Duplicate word in set") }
        self.insert(*i);
        *self.iter()[0] 
    }
}
fn run(input: &str) -> () {
    let mut rng = rand::thread_rng();
    let s = words_string(&input).join("");
    if !s.is_empty() && s.len() >= 16
        && s[0

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_words_string() {
        assert!(words_string("Hi, my name is John") == vec!["Hi", "my", "name", "is", "John"]);
        assert!(
            words_string("One, two, three, four, five, six")
                == vec!["One", "two", "three", "four", "five", "six"]
        );
        assert!(words_string("Hi, my name") == vec!["Hi", "my", "name"]);
        assert!(
            words_string("One,, two, three, four, five, six,")
                == vec!["One", "two", "three", "four", "five", "six"]
        );
        let v_empty: Vec<String> = vec![];
        assert!(words_string("") == v_empty);
        assert!(words_string("ahmed , gamal") == vec!["ahmed", "gamal"]);
    }

}
