// Part 3.
// Lifetimes and move semantics.

// Problem 1.
// What went wrong? Copy strings properly.
#[test]
fn copy_string_test() {
    let str1 = String::from("foo");
    let str2 = str1.clone();
    assert_eq!(str1, str2);
}

// Problem 2.
// Question 2: How come it works fine here?
#[test]
fn copy_int_test() {
    let i1 = 1;
    let i2 = i1;
    assert_eq!(i1, i2);
}

// Problem 3.
// These two don't work either. Fix by changing the type of "string" in the function
// copy_me ONLY, and by adjusting the parameter to "copy_me" where it's called.
#[test]
fn eat_me_test() {
    let str1 = String::from("foo");
    assert_eq!(str1, copy_me(&str1));
}

#[test]
fn eat_me_test2() {
    let str1 = String::from("foo");
    let str2 = copy_me(&str1);
    assert_eq!(str1, str2);
}

fn copy_me(string: &String) -> String {
    string.clone()
}

// Problem 4.
// Can you implement this function?
// Add a lifetime specifier to the return type if you think it will help.
// If not, why not.
// fn new_ref_string() -> &String {
//     &String::from("cat")
// }

// Problem 5.
// Can you implement this function?
// Add a lifetime specifier to the return type if you think it will help.
// If not, why not.
fn new_ref_str() -> &'static str {
    "hello"
}

// Problem 6.
// Now we know how to implement this type of function. Implement it and write a test
// pick_longest_tests2() which passes all tests.
fn pick_longest2<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s2.len() > s1.len() {
        s2
    } else {
        s1
    }
}

#[test]
fn test_pick_longest2() {
    assert_eq!(pick_longest2("catdog", "cat"), "catdog");
    assert_eq!(pick_longest2("dog", "catdog"), "catdog");
    assert_eq!(pick_longest2("", "a"), "a");
    assert_eq!(pick_longest2("", ""), "");
    assert_eq!(pick_longest2("🌶🌶🌶", "🌶🌶"), "🌶🌶🌶");
}

// Problem 7.
// Write a function with a type signature which type checks the following test:
// and passes the test.
// This function compares it's second argument againsts all elements in it's first
// argument, returning those that are less than (<).

fn find_lesser_values<'a>(v: &'a Vec<&str>, x: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for &s in v.iter() {
        if s < x {
            result.push(s);
        }
    }
    result
}

#[test]
fn find_lesser_values_test() {
    assert_eq!(
        find_lesser_values(&vec!["foo", "bar", "foobar"], "zzzzzzzz"),
        vec!["foo", "bar", "foobar"]
    );
    assert_eq!(
        find_lesser_values(&vec!["foo", "bar", "foobar"], "bars"),
        vec!["bar"]
    );
    // Add more tests.
}

// Problem 8
// Move semantics present intersting API design choices not found in other languages.
// HashMap is an example of such a API.
// https://doc.rust-lang.org/std/collections/struct.HashMap.html

// Specifically, the Entry API:
// https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html

use std::collections::HashMap;

// Implement the following function which converts pairs from a vector,
// into key-value pairs in a hashmap.

fn vector_to_hashmap(v: &Vec<(i32, String)>) -> HashMap<i32, String> {
    let mut result: HashMap<i32, String> = HashMap::new();
    for (i, s) in v.iter() {
        result.insert(*i, s.to_string());
    }
    result
}

// Rust prevents us from deleting entries while iterating... Rewrite
// this function to delete all entries in hashmap where the keys
// are negative.
fn delete_negative_keys(h: &mut HashMap<i32, i32>) {
    // This fails, uncomment to see error.
    let mut to_delete = vec![];
    for k in h.keys() {
        if *k < 0 {
            to_delete.push(*k);
        }
    }
    for k in to_delete {
        h.remove(&k);
    }
}

#[test]
fn test_delete_negative_keys() {
    let mut h: HashMap<i32, i32> = HashMap::new();
    for i in -10..10 {
        if i < 0 {
            h.insert(i, -1);
        } else {
            h.insert(i, 1);
        }
    }
    delete_negative_keys(&mut h);
    assert_eq!(h.remove(&-2), None);
    assert_eq!(h.remove(&-4), None);
    assert_eq!(h.remove(&-7), None);
}

// For all entries in `add`: (k, v)
// If `k` exists in `merged`, append `v` to the value of `merged[k]`.
// If that `k` doesn't exist in `merged`, add the (k, v) to `merged`.

// Use the Entry API:
// https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html
// Use `or_insert` and `and_modify`.
fn merge_maps(merged: &mut HashMap<String, String>, add: HashMap<String, String>) {
    for (k, v) in add {
        merged.entry(k).and_modify(|e| e.push_str(&v)).or_insert(v);
    }
}

#[test]
fn test_merge_maps() {
    let mut h1: HashMap<String, String> = HashMap::new();
    let mut h2: HashMap<String, String> = HashMap::new();
    h1.insert(String::from("cat"), String::from("dog"));
    h1.insert(String::from("tomato"), String::from("basil"));
    h2.insert(String::from("cat"), String::from("hamster"));
    h2.insert(String::from("dog"), String::from("hamster"));

    merge_maps(&mut h1, h2);
    assert_eq!(
        h1.get(&String::from("cat")),
        Some(&"doghamster".to_string())
    );
    assert_eq!(h1.get(&String::from("tomato")), Some(&"basil".to_string()));
}
