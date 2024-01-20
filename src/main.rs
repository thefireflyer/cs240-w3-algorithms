///////////////////////////////////////////////////////////////////////////////

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Error;
use linear_search::linear_search_iterative;
use random_loop::{insertion_sort_iterative, insertion_sort_recursive};

use crate::{
    binary_search::{binary_search_iterative, binary_search_recursive},
    linear_search::linear_search_recursive,
};

mod binary_search;
mod linear_search;
mod random_loop;

///////////////////////////////////////////////////////////////////////////////

fn main() -> Result<(), Error> {
    let input = parse_file("numbers-3.txt")?;

    let mut iter = input.clone();
    insertion_sort_iterative(&mut iter);

    let mut rec = input.clone();
    insertion_sort_recursive(&mut rec);

    println!("{:-^38}", "linear search");

    println!(
        "iterative: {:?} \n recursive: {:?}\n",
        linear_search_iterative(&iter, &8128705),
        linear_search_recursive(&rec, &8128705)
    );

    println!(
        "iterative: {:?} \n recursive: {:?}\n",
        linear_search_iterative(&iter, &5842193),
        linear_search_recursive(&rec, &5842193)
    );

    println!("{:-^38}", "binary search");

    println!(
        "iterative: {:?} \n recursive: {:?}\n",
        binary_search_iterative(&iter, &8128705),
        binary_search_recursive(&rec, &8128705)
    );

    println!(
        "iterative: {:?} \n recursive: {:?}\n",
        binary_search_iterative(&iter, &5842193),
        binary_search_recursive(&rec, &5842193)
    );

    println!("{:-^38}", "standard lib");
    let mut standard = input.clone();
    standard.sort();

    println!(
        "8128705 @ {:?} \n 5842193 @ {:?}\n",
        standard.binary_search(&8128705),
        standard.binary_search(&5842193)
    );

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////

pub fn parse_file(path: &str) -> Result<Vec<i32>, Error> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let arr: Vec<i32> = buffered
        .lines()
        .map(|val| val.unwrap().parse::<i32>().unwrap())
        .collect();

    Ok(arr)
}

///////////////////////////////////////////////////////////////////////////////
