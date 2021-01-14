#![allow(dead_code)]

// Uncomment these to have Rust compile the other files as well.
mod lib2;
mod lib3;

// Part 1. Implementing Functions. Taken from Fall 2016's Rust class.
// Write unit tests for you functions.

// Problem 1
// Implement the sum function on slices. Do not use the predefined sum function.
fn sum(slice: &[i32]) -> i32 {
    let mut total = 0;
    for elem in slice.iter() {
        total += *elem;
    }
    total
}
#[test]
fn test_sum() {
    let vs = vec![1, 2, 3, 4, 5];
    assert_eq!(sum(&vs), 15);
}

// Problem 2.
// Make unique. Create a new vector which contains each item in the vector
// only once! Much like a set would.
// Please implement this using a for loop.
fn unique(vs: &Vec<i32>) -> Vec<i32> {
    let mut u: Vec<i32> = Vec::with_capacity(vs.len());
    for elem in vs {
        if !u.contains(elem) {
            u.push(*elem);
        }
    }
    u
}

#[test]
fn test_unique() {
    let v = vec![1, 1, 2, 2, 3, 4, 4, 5];
    let u = unique(&v);
    assert_eq!(vec![1, 2, 3, 4, 5], u);
}

// Problem 3.
// return a new vector containing only elements that satisfy `pred`.
fn filter(vs: &Vec<i32>, pred: &dyn Fn(i32) -> bool) -> Vec<i32> {
    let mut filtered: Vec<i32> = Vec::new();
    for elem in vs {
        if pred(*elem) {
            filtered.push(*elem);
        }
    }
    filtered
}

#[test]
fn filter_tests() {
    assert_eq!(
        filter(&vec![1, 2, 3, 4, 5, 6], &|n| n % 2 == 0),
        vec![2, 4, 6]
    );
}

// Problem 4
// Given starting fibonacci numbers n1 and n2, compute a vector
// where v[i] is the ith fibonacci number.
fn fibonacci(n1: i32, n2: i32, how_many: usize) -> Vec<i32> {
    let mut fibs: Vec<i32> = Vec::new();
    fibs.push(n1);
    fibs.push(n2);
    for i in 2..how_many {
        fibs.push(fibs[i - 1] + fibs[i - 2]);
    }
    fibs
}

#[test]
fn test_fibs() {
    let fibs = fibonacci(0, 1, 10);
    assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
}

// Problem 5
// Create a function which concats 2 strs and returns a String.
// You may use any standard library function you wish.
fn str_concat(s1: &str, s2: &str) -> String {
    s1.to_string() + s2
}

#[test]
fn test_str_concat() {
    assert_eq!(String::from("catdog"), str_concat("cat", "dog"));
}

// Problem 6
// Create a function which concats 2 string and returns a String.
// You may use any standard library function you wish.
fn string_concat(s1: &String, s2: &String) -> String {
    let mut s3 = String::from(s1);
    s3.push_str(s2);
    s3
}

#[test]
fn test_string_concat() {
    assert_eq!(
        String::from("bananahamburger"),
        string_concat(&String::from("banana"), &String::from("hamburger"))
    );
}

// Problem 7
// Convert a Vec<String> into a Vec<u64>. Assume all strings
// are correct numbers! We will do error handling later. Use
// `.expect("ignoring error")` to ignore Result from parse()
// See https://doc.rust-lang.org/std/primitive.str.html#method.parse
// Use turbo fish! Do not use type inference for parse()
fn concat_all(v: Vec<String>) -> Vec<u64> {
    let mut u: Vec<u64> = Vec::new();
    for elem in v.iter() {
        u.push(elem.parse::<u64>().expect("ignore!"));
    }
    u
}

#[test]
fn test_concat_all() {
    assert_eq!(
        concat_all(vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4")
        ]),
        vec![1, 2, 3, 4]
    );
}

// Implement concat_all using map, parse (with turbo fish), and collect()
// Check out how the lecture does something similar:
// https://github.com/upenn-cis198/lecture2/blob/f54753527c1dabbd5e55c2f48a19745768769beb/src/lib.rs#L362
fn concat_all_with_map(v: Vec<String>) -> Vec<u64> {
    v.into_iter()
        .map(|s| s.parse::<u64>().expect("ignore"))
        .collect::<Vec<u64>>()
}

#[test]
fn test_concat_all_with_map() {
    assert_eq!(
        concat_all(vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4")
        ]),
        vec![1, 2, 3, 4]
    );
}
