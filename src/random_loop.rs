///////////////////////////////////////////////////////////////////////////////

use core::fmt;

use tailcall::tailcall;

///////////////////////////////////////////////////////////////////////////////

// --- Quick visualization
//
// [5  9  1  0  6  4]
//     ?
//  ^  .
// [5  9  1  0  6  4]
//        ?
//     ^  .
//  ^  1<>9
//  1<>5  .
// [1  5  9  0  6  4]
//           ?
//        ^  .
//     ^  0<>9
//  ^  0<>5  .
//  0<>1  .  .
// [0  1  5  9  6  4]
//              ?
//           ^  .
//        ^  6<>9
// [0  1  5  6  9  4]
//                 ?
//              ^  .
//           ^  4<>9
//        ^  4<>6  .
//     ^  4<>5  .  .
// [0  1  4  5  6  9]

//---------------------------------------------------------------------------//

// --- Public documentation
//
/// Sorts the provided slice in ascending order.
///
/// - Inputs
///     | `arr: &mut [T]`
///     | The slice to sort (mutable)
///
/// - Side-effects
///     | Sorts elements in `arr`
///
//
// --- Function signature
//
// We'll use the generic `<T>` type for each element in the array.
//
// Rust requires us to explicitly mark mutable objects.
// We're definitely changing the input, so we'll mark `arr` as `&mut`
//
// We saw `[T]` slices last week. They're just a way to reference blocks
// of data we only care about for a little while.
//
// `where T: Ord + ...` is how we talk about interfaces in Rust.
// Here, we're saying that `T` should always implement:
// - an interface for ordering (`Ord`), so we can use `<`
// - an interface for pretty printing (`Debug`)
// - and an interface for making objects with the same contents as another
// object (`Clone`)
//
pub fn insertion_sort_iterative<T>(arr: &mut [T])
where
    T: Ord + Clone,
{
    /*
    -- Main algorithm (insertion sort)

    For each element in `arr`
        |
        | Save the value of the current element as `key`
        |
        | Let's just say that previous elements are already sorted.
        | We'll see why as we continue.
        |
        | Since we're assuming previous items are sorted, we can just take the
        | current element and walk backwards to find it's correct spot.
        |
        | Really, we're going to walk backwards and continually swap elements
        | to make room for our current element when we find its correct spot.
        |
        | To do this, we'll create a reverse index, `rev_ind`, and initialize
        | it to one less than our current index. (we already know we're not
        | bigger than ourselves)
        |
        | Then, we'll iterate backwards until we find an element that isn't
        | smaller than us. Because we're already swapping each pair of
        | elements, we're actually already done. Last iteration, we swapped
        | ourselves directly into the right spot.
        |
        | One last note, we'll check to make sure we're not going to walk off
        | the edge of the list with a simple index check.
        |
        | This means each element walks itself into its sorted spot and ends up
        | leaving our whole list sorted.

    That's it!

    */

    // Iterate over every element
    for index in 1..arr.len() {
        // save the current value
        let key = arr[index].clone();

        // initialize our reverse index (skipping ourselves)
        let mut rev_ind = index - 1;

        // make sure we're smaller than the leftward element
        while key < arr[rev_ind] {
            // swap places with the leftward element
            arr.swap(rev_ind + 1, rev_ind);

            // double check we're not going to walk off the edge of the list
            if rev_ind == 0 {
                // if we are, abort! time to move to the next key element
                break;
            }

            // walk leftwards
            rev_ind -= 1;
        }
    }
}

//---------------------------------------------------------------------------//

// --- Public documentation
//
/// Sorts the provided slice in ascending order.
///
/// - Inputs
///     | `arr: &mut [T]`
///     | The slice to sort (mutable)
///
/// - Side-effects
///     | Sorts elements in `arr`
///
pub fn insertion_sort_recursive<T>(arr: &mut [T])
where
    T: Ord + Clone + fmt::Debug,
{
    // The `#[tailcall]` macro is explained in linear_search.rs

    /*
    [Inner algorithm]

        |
        | We're pretty much just re-using our existing code, but replacing
        | the for loops with recursive functions.
        |
        | In our iterative code, we have two for loops
        | (1) walk through each key, left to right
        | (2) walk, backwards, from each key to the beginning of the list,
        |       right to left
        |
        | So, we'll have two functions:
        | (1) `inner`, moving left to right
        | (2) `walk_backwards`, moving right to left, starting at a given key
        |
        | We'll pass index to both as a parameter and adjust it each cycle
        |

    */

    #[tailcall]
    fn walk_backwards<T: Ord>(arr: &mut [T], index: usize, key: T) {
        // check if we're smaller than the leftward element
        // and that we're not about to walk off the edge of the list
        if key < arr[index] && index > 0 {
            // swap the values of arr at index + 1 and index
            arr.swap(index + 1, index);
            // continue walking backwards
            walk_backwards(arr, index - 1, key);
        }
        // check if we're smaller than the leftward element
        else if key < arr[index] {
            // swap the values of arr at index + 1 and index
            arr.swap(index + 1, index);
            // we'll just move on at this point so that we don't walk off the
            // edge of the list
        }
        // we're not smaller than the element,
        // time to move on to the next key
    }

    #[tailcall]
    fn inner<T: Ord + Clone>(arr: &mut [T], index: usize) {
        // check if we're still on a valid element
        if index < arr.len() {
            // start walking backwards with the current element as the key
            walk_backwards(arr, index - 1, arr[index].clone());
            // continue on to the next key element
            inner(arr, index + 1);
        }
        // if not, just implicitly return...
    }

    // start the whole algorithm on the second element
    // (we already know the first isn't smaller than itself)
    inner(arr, 1);
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    fn helper(cases: Vec<Vec<i32>>) {
        for case in cases {
            let mut real_iter = case.clone();
            let mut real_rec = case.clone();
            let mut expected = case.clone();

            insertion_sort_iterative(&mut real_iter);
            insertion_sort_recursive(&mut real_rec);

            expected.sort();

            assert_eq!(real_iter, expected);
            assert_eq!(real_rec, expected);
        }
    }

    #[test]
    fn special_cases() {
        helper(vec![vec![], vec![1]])
    }

    #[test]
    fn random_cases() {
        helper(vec![
            vec![1, 3, 2],
            vec![1, 3, 2],
            vec![3, 3, 3],
            vec![3, 3, 2],
            vec![593, 52, 0, 40104, 20, 19, 2, 30, 8],
            vec![5, 23, 6, 8, 9, 0, 2],
        ])
    }

    #[test]
    fn sorted_cases() {
        helper(vec![
            vec![1, 2],
            vec![1, 2, 3],
            vec![0, 2, 5, 6, 8, 9, 23],
            vec![-503, 1, 203, 585, 900],
        ]);
    }

    #[test]
    fn reverse_sorted_cases() {
        helper(vec![
            vec![2, 1],
            vec![3, 2, 1],
            vec![5, 4, 3, 2, 1, 0, -40],
            vec![23, 9, 8, 6, 5, 2, 0],
        ]);
    }

    #[test]
    fn test_big_sorted() {
        let big_number = (2 as i32).pow(5);
        let mut arr: Vec<i32> = Vec::with_capacity(big_number as usize);
        for i in 0..big_number {
            arr.push(i);
        }

        helper(vec![arr]);
    }

    #[test]
    fn test_big_rev_sorted() {
        let big_number = (2 as i32).pow(5);
        let mut arr: Vec<i32> = Vec::with_capacity(big_number as usize);
        for i in big_number..0 {
            arr.push(i);
        }

        helper(vec![arr]);
    }
}

///////////////////////////////////////////////////////////////////////////////
