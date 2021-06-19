

fn find_min_index<T>(array : &Vec<T>, start_pos : usize) -> Option<usize>
where T : PartialOrd +Copy {
    if array.len() - start_pos < 2 {
        return None;
    }
    let mut smallest_index = start_pos;
    for i in (start_pos+1)..array.len() {
        if array[i] < array[smallest_index] {
            smallest_index = i;
        }
    }
    if smallest_index == start_pos {
        return None;
    }
    else {
        return Some(smallest_index)
    }
}

fn temp<T>(a : &mut T, b : T) -> T
where T : Copy {
    let temp = *a;
    *a = b;
    temp
}

/// ### Selection sort algorithm
/// 
/// Worst-case performance:	O(n2) comparisons, O(1) swaps
/// 
/// Average performance:    O(n2) comparisons, O(1) swaps
/// 
/// Best-case performance:  O(n2) comparisons, O(1) swaps
/// 
pub fn sort<T>(array : &mut Vec<T>)
where T : PartialOrd + Copy {
    let length = array.len();
    if length > 1 {
        for i in 0..length {
            if let Option::Some(index) = find_min_index(array, i) {
                let b = array[i];
                array[i] = temp(&mut array[index], b);
            }
        }
    }
}