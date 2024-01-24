# CS 240 | Week 3 | Algorithms

[![Unit testing](https://github.com/thefireflyer/cs240-w3-algorithms/actions/workflows/test.yml/badge.svg)](https://github.com/thefireflyer/cs240-w3-algorithms/actions/workflows/test.yml)


| | |
|-|-|
| Author | Aidan Beil |
| Date | 17/1/2024 |
| Class | CS240 2963 |
| Professor | Darrell Criss |

---

### Organization

- Pseudo code
    - [linear search](pseudocode/linear_search.pseudocode)
    - [binary search](pseudocode/binary_search.pseudocode)
    - [insertion sort](pseudocode/insertion_sort.pseudocode)
- Actual code
    - [main](src/main.rs)
    - [linear search](src/linear_search.rs)
    - [binary search](src/binary_search.rs)
    - [insertion sort](src/random_loop.rs)

---

### Usage

`cargo run`

> ```
> ------------linear search-------------
> [iterative] 8128705 @ Some(8101) 
> [recursive] 8128705 @ Some(8101)
> 
> [iterative] 5842193 @ None 
> [recursive] 5842193 @ None
> 
> ------------binary search-------------
> [iterative] 8128705 @ Some(8101) 
> [recursive] 8128705 @ Some(8101)
> 
> [iterative] 5842193 @ None 
> [recursive] 5842193 @ None
> 
> -------------standard lib-------------
> 8128705 @ Ok(8101) 
> 5842193 @ Err(5838)
> ```


`cargo test --release`

- <p style="color:red;">~8 GiB memory usage</p>

`cargo test -- --skip test_big`

> ```
> running 10 tests
> test binary_search::tests::test_empty_list ... ok
> test binary_search::tests::test_existent_target ... ok
> test binary_search::tests::test_nonexistent_target ... ok
> test linear_search::tests::test_empty_list ... ok
> test linear_search::tests::test_existent_target ... ok
> test linear_search::tests::test_nonexistent_target ... ok
> test random_loop::tests::random_cases ... ok
> test random_loop::tests::reverse_sorted_cases ... ok
> test random_loop::tests::sorted_cases ... ok
> test random_loop::tests::special_cases ... ok
> 
> test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s
> ```

---

### Sectioning

`///////////////////////////////////////////////////////////////////////////////`

`//---------------------------------------------------------------------------//`

`//...........................................................................//`


### Sources

[1] [Grokking Algorithms](https://livebook.manning.com/book/grokking-algorithms-second-edition/chapter-1/v-4/)

[2] [Core Rust source code](https://doc.rust-lang.org/src/core/slice/mod.rs.html#2837)

[3] [Creusot binary search example code](https://github.com/xldenis/creusot/blob/master/creusot/tests/should_succeed/vector/04_binary_search.rs)

[4] [Rust documentation](https://doc.rust-lang.org/std/primitive.usize.html#impl-Div-for-usize)


---
