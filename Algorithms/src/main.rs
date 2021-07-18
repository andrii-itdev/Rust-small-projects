
extern crate rand;

mod merge_sort;
mod binary_search;
mod bubble_sort;
mod insertion_sort;
mod selection_sort;
mod quick_sort;
mod heap_sort;

use rand::Rng;
use std::{fmt::{Debug, Display}, time::Instant, usize};

use binary_search::search as binary_search;
use merge_sort::merge_sort as merge_sort;
use bubble_sort::sort as bubble_sort;
use insertion_sort::sort as insertion_sort;
use selection_sort::sort as selection_sort;
use quick_sort::sort as quick_sort;
use heap_sort::sort as heap_sort;


fn get_random_vector(n : usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(rng.gen_range(0, n));
    }
    v
}

fn is_sorted_asc<T>(array : &Vec<T>) -> bool
where T : PartialOrd {
    if array.len() > 1 {
        let mut prev = &array[0];
        for i in 1..array.len() {
            if array[i] < *prev {
                return false;
            }
            prev = &array[i];
        }
    }
    true
}

fn binsearch_item<T>(array : &Vec<T>, what : &T, print_array : bool)
where T : PartialEq + PartialOrd + Debug + Display {
    
    if print_array {
        println!("len: {}  {:?}\n", array.len(), array);
    }

    let now = Instant::now();

    let search_option = binary_search(&array, &what);

    println!("elapsed: {:?}", now.elapsed());

    if let Some(index) = search_option {
        println!("Item {} was found at position {}", what, index);
    }
    else{
        println!("Item {} was not found!", what);
    }
}

fn test_binsearch(num: usize, print_array: bool) {
    let mut array = get_random_vector(num);
    array.sort();
    let mut rng = rand::thread_rng();
    let what = rng.gen_range(0, num);
    binsearch_item(&array, &what, print_array);
}

fn test_sort_method<T>(f_sort : fn(array : &mut Vec<T>), array : &mut Vec<T>, print_array : bool)
where T : Debug + PartialOrd {
    let now = Instant::now();

    if print_array {
        println!("len: {}  {:?}\n", array.len(), array);
    }
    
    f_sort(array);
    
    if print_array {
        println!("sorted: {:?}\n", array);
    }

    println!("elapsed: {:?}", now.elapsed());
    println!("is sorted: {:?}\n", is_sorted_asc(array));
}

fn test_sort_method_for_random_array(f_sort : fn(array : &mut Vec<usize>), num : usize, print_array : bool) {
    let mut array = get_random_vector(num);
    println!("{:?}", array);
    test_sort_method(f_sort, &mut array, print_array)
}


fn main() {

    let is_debug_mode = true;
    // Test Sorting algorithms
    let num : usize = if is_debug_mode { 10 } else { 10000 };
    let print_array = is_debug_mode;

    println!("Bubble Sort");
    test_sort_method_for_random_array(bubble_sort, num, print_array);
    println!("Merge Sort");
    test_sort_method_for_random_array(merge_sort, num, print_array);
    println!("Insert Sort");
    test_sort_method_for_random_array(insertion_sort, num, print_array);
    println!("Selection Sort");
    test_sort_method_for_random_array(selection_sort, num, print_array);
    println!("Quick Sort");
    test_sort_method_for_random_array(quick_sort, num, print_array);

    println!("Heap Sort");
    test_sort_method_for_random_array(heap_sort, num, print_array);

    // Test Binary search
    let num = 100;
    println!("Binary search");
    test_binsearch(num, print_array);
}
