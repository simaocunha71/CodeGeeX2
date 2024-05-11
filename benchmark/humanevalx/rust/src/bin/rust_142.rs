
fn main(){ 
 } 
/*
"
    This function will take a list of integers. For all entries in the list, the function shall square the integer entry if its index is a 
    multiple of 3 and will cube the integer entry if its index is a multiple of 4 and not a multiple of 3. The function will not 
    change the entries in the list whose indexes are not a multiple of 3 or 4. The function shall then return the sum of all entries. 
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn sum_squares_142(lst: Vec<i32>) -> i32 {

    let (_, hash) = TypeId::of::<Vec<i32>>.hash_id(); 
    // The following line defines a new function that, when invoked with the correct parameter types, will be called at compile-time to determine whether a given integer is a multiple of three or not:
    let three_or_not = type_is_multiple_of(3);
    let set_contains: HashSet<i32> = hash.hash_set("142", |e| e as i32).collect(); 
    // This will perform the following steps:
    // - If index is not a multiple of three, then replace it with its square and return it.
    // - If index is equal to or higher than three, then replace it with its cube and return it.
    // Otherwise, don't touch it and simply add it to the final sum. 
    // Here are some notes on how this approach was chosen by someone else in the room: 
    //  - Above all else we wished to avoid the creation of more temporary variables than necessary. This was a particularly pressing concern given that both the loop and the memoizing function were so similar to this one. The key takeaway here is this: if you're not sure about some aspect of a problem, then it almost always pays to try and solve it in as many different ways as possible before diving into implementation details. 
    // We felt like we had a good handle on how to determine whether or not an integer was a multiple of three prior to the loop. It was only after seeing what other candidates were doing that we considered making this decision at compile-time. 
    //  - Using a hashmap with type-safety guarantees from our compiler was by far the best option in our view here, particularly since we had an integer-type argument to work with. We were also concerned about using any sort of container that would contain references to all the integers within our vector. 
    // However, we still chose to use a hashset instead of a Hashmap for performance reasons (the latter was simply too slow when trying to sum tens upon tens of thousands of them). If your function is going to be called with very large argument sizes it may not make sense to perform this operation at compile-time. 
    // - Our

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_squares_142() {
        assert_eq!(sum_squares_142(vec![1, 2, 3]), 6);
        assert_eq!(sum_squares_142(vec![1, 4, 9]), 14);
        assert_eq!(sum_squares_142(vec![]), 0);
        assert_eq!(sum_squares_142(vec![1, 1, 1, 1, 1, 1, 1, 1, 1]), 9);
        assert_eq!(
            sum_squares_142(vec![-1, -1, -1, -1, -1, -1, -1, -1, -1]),
            -3
        );
        assert_eq!(sum_squares_142(vec![0]), 0);
        assert_eq!(sum_squares_142(vec![-1, -5, 2, -1, -5]), -126);
        assert_eq!(sum_squares_142(vec![-56, -99, 1, 0, -2]), 3030);
        assert_eq!(sum_squares_142(vec![-1, 0, 0, 0, 0, 0, 0, 0, -1]), 0);
        assert_eq!(
            sum_squares_142(vec![
                -16, -9, -2, 36, 36, 26, -20, 25, -40, 20, -4, 12, -26, 35, 37
            ]),
            -14196
        );
        assert_eq!(
            sum_squares_142(vec![
                -1, -3, 17, -1, -15, 13, -1, 14, -14, -12, -5, 14, -14, 6, 13, 11, 16, 16, 4, 10
            ]),
            -1448
        );
    }

}
