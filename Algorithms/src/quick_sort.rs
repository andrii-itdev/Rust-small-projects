use std::usize;

fn swap<T>(arr : &mut Vec<T>, i : usize, j : usize)
where T : Clone {
    let temp = arr[i].clone();
    arr[i] = arr[j].clone();
    arr[j] = temp;
}

fn partition<T>(begin: usize, end: usize, arr: &mut Vec<T>, p_index : usize) -> usize
where T : Clone + PartialOrd {
    let mut i = begin;
    let mut j = end - 1;
    let p = arr[p_index].clone();

    while i < j {
        while i < j && arr[i] <= p {
            i += 1;
        }
        while i < j && arr[j] > p {
            j -= 1;
        }
        if arr[i] > arr[j] {
            swap(arr, i, j);
        }
    }
    i
}

fn quick_sort<T>(arr : &mut Vec<T>, begin : usize, end : usize, p_index : usize)
where T : PartialOrd + Clone {
    if end - begin == 2 && arr[begin] > arr[end - 1] {
            swap(arr, begin, end - 1);
    }
    else if end - begin > 2 {
        let p = partition(begin, end, arr, p_index);

        quick_sort(arr, begin, p, p - 1);
        quick_sort(arr, p, end, p);
    }
}

/// ### Quick sort algorithm
/// 
/// Worst-case performance:	O(n2)
/// 
/// Average performance:    O(n log n)
/// 
/// Best-case performance:  O(n log n) 
/// 
pub fn sort<T>(arr : &mut Vec<T>)
where T : PartialOrd + Copy {
    if arr.len() > 1 {
        quick_sort(arr, 0, arr.len(), 0);
    }
}
