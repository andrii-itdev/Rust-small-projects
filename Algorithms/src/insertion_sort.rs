
pub fn insert<T>(array : &mut Vec<T>, from_pos : usize, to_pos : usize)
where T : Copy
{
    let temp = array[from_pos];
    let mut i = from_pos;
    while i > to_pos {
        array[i] = array[i-1];
        i -= 1;
    }
    array[to_pos] = temp;
}

pub fn insert_sorted<T>(array : &mut Vec<T>, position : usize)
where T : PartialOrd + Copy {
    let mut i = 0;
    while i < position && array[i] < array[position] { i += 1; }
    insert(array, position, i)
}

pub fn sort<T>(array : &mut Vec<T>)
where T : PartialOrd + Copy {
    let length = array.len();
    if length > 1 {
        for i in 1..length {
            insert_sorted(array, i);
        }
    }
}

