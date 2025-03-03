fn main() {
    // Initialize vector
    let mut list  = vec![5, 3, 7, 1, 9, 8];

    find_median(&mut list);

   

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




