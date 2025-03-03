fn main() {
    // Initialize vector
    let mut list  = vec![5, 3, 7, 1, 9, 8, 9];

    find_median(&mut list);

    find_mode(&mut list);

   

    //
}

fn find_median(list: &mut Vec<i32>) -> i32 {
    let vec_len = &list.len();

    // Sort the vector
    list.sort();
    let median = &list[vec_len/2];
    println!("The median of the list is: {}", &median);
    *median
}



fn find_mode(list: &mut Vec<i32>) -> Option<i32> {
    use std::collections::HashMap;

    let mut map = HashMap::new();

    for n in list {
        let count = map.entry(n).or_insert(0);
        *count += 1;
    }

    let mut max = 0;
    let mut mode = None;
    for (key, value) in map {
        if max <= value {
            max = value;
            mode = Some(*key);
        }
    }
    println!("The mode of the list is {:?}", &mode.unwrap());
    mode
}




