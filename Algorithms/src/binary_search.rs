

fn search_index<T>(array : &Vec<T>, what : &T, begin : usize, end : usize) -> Option<usize>
where T : std::cmp::PartialEq + std::cmp::PartialOrd {
    let diff = end - begin;
    if diff > 0 {
        let middle = (begin + end) / 2;

        if array[middle] == *what {
            return Option::Some(middle);
        }
        else if diff > 1 { 
            if array[middle] > *what {
                return  search_index(array, what, begin, middle);
            }
            else {
                return  search_index(array, what, middle, end);
            }
        }
    }
    
    return Option::None;
    
}

pub fn search<T>(array : &Vec<T>, what : &T) -> Option<usize>
where T : std::cmp::PartialEq + std::cmp::PartialOrd {
    if array.len() == 0 {
        return Option::None;
    }
    else {
        return search_index(array, what, 0, array.len())
    }
}