
fn swap<T>(array: &mut Vec<T>, i: usize)
where T : Clone {
    let temp = array[i].clone();
    array[i] = array[i + 1].clone();
    array[i + 1] = temp;
}

fn sort_ifless<T>(array : &mut Vec<T>, position : usize)
where T : PartialOrd + Clone {

    for i in 0..position {
        if array[i] > array[i + 1] {
            swap(array, i)
        }
    }
}

/// ### Bubble sort algorithm
/// 
/// Worst-case performance:	O(n2) comparisons, O(n2) swaps
/// 
/// Average performance:    O(n2) comparisons, O(n2) swaps
/// 
/// Best-case performance:  O(n) comparisons, O(1) swaps
/// 
pub fn sort<T>(array : &mut Vec<T>)
where T : PartialOrd + Clone {
    let length = array.len();
    if length > 1 {
        let mut i = length;
        while i > 0 {
            sort_ifless(array, i - 1);
            i -= 1;
        }
    }
}

