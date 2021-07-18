use std::fmt;

/*
fn swap(&mut self, a: usize, b: usize) {
    unsafe {
        // Can't take two mutable loans from one vector, so instead just cast
        // them to their raw pointers to do the swap
        let pa: *mut T = &mut self[a];
        let pb: *mut T = &mut self[b];
        ptr::swap(pa, pb);
    }
}
*/

fn heapify<T>(arr : &mut Vec<T>, heap_length : usize, parent_index : usize)
where T : PartialOrd + Clone {
    let mut largest = parent_index;
    let left = parent_index * 2 + 1;
    let right = left + 1;

    if left < heap_length && arr[left] > arr[largest] {
        largest = left;
    }
    if right < heap_length && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != parent_index {
        arr.swap(largest, parent_index);
        heapify(arr, heap_length, largest);
    }
}

fn build_heap<T>(arr : &mut Vec<T>, length : usize)
where T : PartialOrd + Clone {
    let mut last_parent = length / 2;

    while last_parent > 0 {
        heapify(arr, length, last_parent - 1);
        last_parent -= 1;
    }
}

pub fn sort<T>(arr : &mut Vec<T>)
where T : Ord + Clone + fmt::Debug {
    let length = arr.len();
    if length > 1 {
        build_heap(arr, length);
        for i in (1..length).rev() {
            arr.swap(0, i);
            heapify(arr, i, 0);
            //print!("{:?} ", i);
        }
    }
}