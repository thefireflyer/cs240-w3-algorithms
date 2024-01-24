///////////////////////////////////////////////////////////////////////////////

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Error;

use crate::{
    binary_search::{binary_search_iterative, binary_search_recursive},
    linear_search::{linear_search_iterative, linear_search_recursive},
    random_loop::{insertion_sort_iterative, insertion_sort_recursive},
};

mod binary_search;
mod linear_search;
mod random_loop;

///////////////////////////////////////////////////////////////////////////////

/*
[Assignment structure]

    |
    | - linear_search.rs
    |       - iterative linear search
    |       - recursive linear search
    |       - relevant testing
    |
    |
    | - binary_search.rs
    |       - iterative binary search
    |       - recursive binary search
    |       - relevant testing
    |
    |
    | - random_loop.rs
    |       - iterative insertion sort
    |       - recursive insertion sort
    |       - relevant testing
    |


[Testing]

    |
    | RECOMMENDED - `cargo test -- --skip test_big`
    |
    | Too fast to notice
    |
    | ALL - `cargo test --release`
    |
    | ~8 GiB memory requirement
    | ~5s on my machine
    |


[Main function]

    |
    | See main() below
    |
    | Loads `numbers-3.txt` from the root directory
    |
    | Numbers are sorted with insertion sort
    | (both iteratively and recursively)
    |
    | Results verified against standard lib results
    |

*/

///////////////////////////////////////////////////////////////////////////////

fn main() -> Result<(), Error> {
    let input = parse_file("numbers-3.txt")?;

    let mut iter = input.clone();
    insertion_sort_iterative(&mut iter);

    let mut rec = input.clone();
    insertion_sort_recursive(&mut rec);

    println!("{:-^38}", "linear search");

    println!(
        "[iterative] 8128705 @ {:?} \n[recursive] 8128705 @ {:?}\n",
        linear_search_iterative(&iter, &8128705),
        linear_search_recursive(&rec, &8128705)
    );

    println!(
        "[iterative] 5842193 @ {:?} \n[recursive] 5842193 @ {:?}\n",
        linear_search_iterative(&iter, &5842193),
        linear_search_recursive(&rec, &5842193)
    );

    println!("{:-^38}", "binary search");

    println!(
        "[iterative] 8128705 @ {:?} \n[recursive] 8128705 @ {:?}\n",
        binary_search_iterative(&iter, &8128705),
        binary_search_recursive(&rec, &8128705)
    );

    println!(
        "[iterative] 5842193 @ {:?} \n[recursive] 5842193 @ {:?}\n",
        binary_search_iterative(&iter, &5842193),
        binary_search_recursive(&rec, &5842193)
    );

    println!("{:-^38}", "standard lib");
    let mut standard = input.clone();
    standard.sort();

    println!(
        "8128705 @ {:?} \n5842193 @ {:?}\n",
        standard.binary_search(&8128705),
        standard.binary_search(&5842193)
    );

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////

fn parse_file(path: &str) -> Result<Vec<i32>, Error> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let arr: Vec<i32> = buffered
        .lines()
        .map(|val| val.unwrap().parse::<i32>().unwrap())
        .collect();

    Ok(arr)
}

///////////////////////////////////////////////////////////////////////////////
