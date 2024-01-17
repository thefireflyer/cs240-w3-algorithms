///////////////////////////////////////////////////////////////////////////////

use tailcall::tailcall;

///////////////////////////////////////////////////////////////////////////////

/// Returns index of given target in a sorted list.
///
/// Inputs:
/// - `arr: &[i32]`
///     - The sorted list to check in
///
/// - `target: i32`
///     - The target value to check for
///
/// Output variants:
/// - `Some(index)`
///     - `index` is the position of `target` in `arr`
/// - `None`
///     - `target` is not in `arr`
///
pub fn binary_search_iterative<T>(arr: &[T], target: &T) -> Option<usize>
where
    T: Ord,
{
    let mut start: usize = 0;
    let mut end: usize = arr.len();

    while start < end {
        let size = end - start;
        let midpoint = start + (size / 2);
        let item = &arr[midpoint];
        if item == target {
            return Some(midpoint);
        }

        if item < &target {
            start = midpoint + 1;
        }
        if item > &target {
            end = midpoint;
        }
    }
    None
}

//---------------------------------------------------------------------------//

/// Returns index of given target in a sorted list.
///
/// Inputs:
/// - `arr: &[i32]`
///     - The sorted list to check in
///
/// - `target: i32`
///     - The target value to check for
///
/// Output variants:
/// - `Some(index)`
///     - `index` is the position of `target` in `arr`
/// - `None`
///     - `target` is not in `arr`
///
pub fn binary_search_recursive<T>(arr: &[T], target: &T) -> Option<usize>
where
    T: Ord,
{
    #[tailcall]
    fn inner<T>(arr: &[T], target: &T, lower: usize, upper: usize) -> Option<usize>
    where
        T: Ord,
    {
        if lower < upper {
            let size = upper - lower;
            let midpoint = lower + (size / 2);
            let item = &arr[midpoint];

            if item == target {
                Some(midpoint)
            } else if item < target {
                inner(arr, target, midpoint + 1, upper)
            } else {
                inner(arr, target, lower, midpoint)
            }
        } else {
            None
        }
    }

    inner(arr, target, 0, arr.len())
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    use std::vec;

    use super::*;

    fn helper<T>(arr: &[T], target: T, expected: Option<usize>)
    where
        T: Ord,
    {
        if arr.len() > 0 {
            for ind in 0..arr.len() - 1 {
                assert!(arr[ind] < arr[ind + 1]);
            }
        }

        let lin_res = binary_search_iterative(&arr, &target);
        let rec_res = binary_search_recursive(&arr, &target);

        assert_eq!(lin_res, rec_res);
        assert_eq!(rec_res, expected);
    }

    #[test]
    fn test_empty_list() {
        helper(&vec![], 30, None);
        helper(&vec![], -50, None);
        helper(&vec![], 0, None);
    }

    #[test]
    fn test_nonexistent_target() {
        helper(&vec![1], 0, None);
        helper(&vec![1, 2], -4, None);
        helper(&vec![0, 1, 3], 2, None);
        helper(&vec!["t", "te", "tes", "test"], "not there", None);
    }

    #[test]
    fn test_existent_target() {
        helper(&vec![1], 1, Some(0));
        helper(&vec![1, 2], 1, Some(0));
        helper(&vec![1, 2], 2, Some(1));
        helper(&vec![1, 2, 3], 1, Some(0));
        helper(&vec![1, 2, 3], 2, Some(1));
        helper(&vec![1, 2, 3], 3, Some(2));
        helper(&vec!["t", "te", "tes", "test"], "t", Some(0));
        helper(&vec!["t", "te", "tes", "test"], "te", Some(1));
        helper(&vec!["t", "te", "tes", "test"], "tes", Some(2));
        helper(&vec!["t", "te", "tes", "test"], "test", Some(3));
        helper(&vec!["x", "y", "z"], "x", Some(0));
        helper(&vec!["x", "y", "z"], "y", Some(1));
        helper(&vec!["x", "y", "z"], "z", Some(2));
    }

    #[test]
    fn test_big() {
        let mut arr: Vec<i32> = Vec::with_capacity((2 as i32).pow(30) as usize);
        for i in 0..(2 as i32).pow(30) {
            arr.push(i);
        }

        helper(&arr, 0, Some(0));
        helper(
            &arr,
            (2 as i32).pow(30) - 1,
            Some(((2 as i32).pow(30) - 1).try_into().unwrap()),
        );
        helper(&arr, (2 as i32).pow(30), None);
        helper(&arr, 50, Some(50));
    }
}

///////////////////////////////////////////////////////////////////////////////
