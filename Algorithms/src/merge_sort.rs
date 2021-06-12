
fn top_down_merge<T>(array_from : &Vec<T>, array_to : &mut Vec<T>, begin : usize, mid : usize, end : usize)
where T : Ord + Copy
{
    let mut i = begin;
    let mut j = mid;
    let mut k = begin;
    while k < end 
    {
        if  j >= end || (i < mid && array_from[i] < array_from[j]) {
            array_to[k] = array_from[i];
            i += 1;
        }
        else{
            array_to[k] = array_from[j];
            j += 1;
        }
        k += 1;
    }
}

fn top_down_split_merge<T>(array_to : &mut Vec<T>, array_from : &mut Vec<T>, begin : usize, end : usize)
where T : Ord + Copy
{
    //let diff = end as i32 - begin as i32;
    if end > begin + 1 {
        let mid = (end + begin) / 2;
        
        top_down_split_merge(array_from, array_to, begin, mid);
        top_down_split_merge(array_from, array_to, mid, end);
        top_down_merge(&array_from, array_to, begin, mid, end);
    }
}

fn copy_array<T>(array_from : &Vec<T>, array_to : &mut Vec<T>)
where T : Copy + Copy
{
    for i in array_from {
        array_to.push(*i);
    }
}

pub fn merge_sort<T>(array : &mut Vec<T>)
where T : Ord + Copy
{
    let length = array.len();
    let mut array_copy : Vec<T> = Vec::with_capacity(length);
    copy_array(&array, &mut array_copy);
    top_down_split_merge(array, &mut array_copy, 0, length);
}

