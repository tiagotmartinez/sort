use fastrand;
use std::{collections::{HashMap, HashSet}, fmt::Debug, iter::repeat_with, time::Instant};

use sort::*;

/// Assert that `v` is in increasing order.
fn assert_ordered<T: Ord + Debug>(v: &[T]) {
    for i in 1..v.len() {
        assert!(
            v[i - 1] <= v[i],
            "ordering failed at index {i}: {:?} > {:?}",
            v[i - 1],
            v[i]
        );
    }
}

/// Return a sequence of `n` random `usize` values.
fn random_sequence(n: usize) -> Vec<usize> {
    repeat_with(|| fastrand::usize(..10000)).take(n).collect()
}

/// Return a sequence of `n` increasing `usize` values (i.e., `v[i] == i`).
fn increasing_sequence(n: usize) -> Vec<usize> {
    (0..n).collect()
}

/// Return a sequence of `n` decreasing `usize` values.
/// The reverse of [`increasing_sequence`].
fn decreasing_sequence(n: usize) -> Vec<usize> {
    (0..n).rev().collect()
}

/// Return a sequence of `n` equal `usize` values.
fn equal_sequence(n: usize) -> Vec<usize> {
    vec![42; n]
}

/// Return a sequence of `n - 1` equal values plus a smaller one.
fn last_out_of_order(n: usize) -> Vec<usize> {
    let mut v = vec![42; n];
    *v.last_mut().unwrap() = 41;
    v
}

/// Return a sequence of a large value followed by `n - 1` equal ones larger than it.
fn first_out_of_order(n: usize) -> Vec<usize> {
    let mut v = vec![42; n];
    *v.first_mut().unwrap() = 43;
    v
}

const REPETITIONS: usize = 100;
const TIME_LIMIT: u128 = 500;

/// Expands to calling the given sorting function (with name) with all given
/// sorting order functions.  Collect timing statistics and print results.
macro_rules! test_orders {
    ( $name:expr , $sort_fn:expr , $( $vec_fn:expr ),+ $( , )? ) => {
        {
            let max_name_length = [
                $(
                    stringify!($vec_fn).len(),
                )+
            ].iter().max().copied().unwrap();

            let mut results: HashMap<String, f64> = HashMap::new();

            $(
                let vec_name = stringify!($vec_fn).to_string();
                print!("testing {} with {:<width$} : ", $name, &vec_name, width = max_name_length);
                let mut n = 128;
                loop {
                    let started = Instant::now();
                    for _ in 0..REPETITIONS {
                        let mut v = $vec_fn(n);
                        $sort_fn(&mut v);
                        assert_ordered(&v);
                    }
                    let elapsed = started.elapsed();
                    if elapsed.as_millis() >= TIME_LIMIT {
                        let speed = (n as f64 / elapsed.as_secs_f64()) * REPETITIONS as f64;
                        println!("{n:12} in {:5} ms = {:>15.2} elements/s", elapsed.as_millis(), speed);
                        results.insert(vec_name, speed);
                        break;
                    } else {
                        n *= 2;
                    }
                }
            )+

            results
        }
    }
}

/// Expands to calling `test_orders` with all given sorting functions and their names.
macro_rules! test_sorts {
    ( $( $sort_fn:expr ),+ $( , )? ) => {
        {
            let max_name_length = [
                $(
                    stringify!($sort_fn).len(),
                )+
            ].iter().max().copied().unwrap();

            let mut results: HashMap<String, HashMap<String, f64>> = HashMap::new();

            $(
                let sort_name = stringify!($sort_fn).to_string();
                let x = test_orders!(
                    format!("{:<width$}", &sort_name, width = max_name_length), $sort_fn,
                    random_sequence,
                    increasing_sequence,
                    decreasing_sequence,
                    equal_sequence,
                    last_out_of_order,
                    first_out_of_order,
                );
                results.insert(sort_name, x);
            )+

            results
        }
    }
}

fn tabulate(table: &HashMap<String, HashMap<String, f64>>) {
    let sort_names: Vec<_> = table.keys().collect();
    let max_sort_name = sort_names
        .iter()
        .map(|s| s.len())
        .max()
        .unwrap_or(0);

    let vec_names: HashSet<_> = table.iter().flat_map(|(_, v)| v.keys()).collect();
    let max_vec_name = vec_names
        .iter()
        .map(|s| s.len())
        .max()
        .unwrap_or(0);

    print!("{:>width$} |", "", width = max_sort_name);
    for vec_name in vec_names.iter() {
        print!(" {vec_name:>width$} |", width = max_vec_name);
    }
    println!();

    for sort_name in sort_names.iter() {
        print!("{:<width$} |", sort_name, width = max_sort_name);
        for vec_name in vec_names.iter() {
            let value = table.get(*sort_name).unwrap().get(*vec_name).unwrap();
            print!(" {:>width$.2} |", value, width = max_vec_name);
        }
        println!();
    }

}

fn main() {
    let results = test_sorts!(
        gnome_sort,
        bubble_sort,
        selection_sort,
        insertion_sort,
        shell_sort,
        heap_sort,
        quick_sort,
        quick_sort_3,
        merge_sort_top_down,
        merge_sort_bottom_up,
        native_sort,
        native_unstable_sort,
    );

    println!();
    tabulate(&results);
}
