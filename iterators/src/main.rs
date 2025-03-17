fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    
    for val in v1_iter {
        println!("Got: {val}");
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoe_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}


// ALL iterators implement the method Iterator:
// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;

//     // methods with default implementations elided
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("Sandels"),
            },
            Shoe {
                size: 9,
                style: String::from("Sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("Boot"),
            },
        ];

        let result = shoe_in_size(shoes, 10);

        assert_eq!(result,
        vec![
            Shoe {
                size: 10,
                style: String::from("Sandels"),
            },
            Shoe {
                size: 10,
                style: String::from("Boot"),
            },
        ]);
    }
}

// #[test]
// fn iterator_demonstration() {
//     let v1 = vec![1, 2, 3];
//     // Note we make v1_iter mutable
//     let mut v1_iter = v1.iter();
//     // This iterates over immutable references.
//     // If we wanted to make them mutable
//     // we use v1.into_iter instead of v1.iter
//     // And if we want to iterate over those mutable references
//     // we use iter_mut instead of iter.
//     assert_eq!(v1_iter.next(), Some(&1));
//     assert_eq!(v1_iter.next(), Some(&2));
//     assert_eq!(v1_iter.next(), Some(&3));
//     assert_eq!(v1_iter.next(), None);
// }

// #[test]
// fn iterator_sum() {
//     let v1 = vec![1, 2, 3];

//     let v1_iter = v1.iter();

//     let total: i32 = v1_iter.sum();

//     // We aren’t allowed to use v1_iter
//     // after the call to sum because
//     // sum takes ownership of the iterator we call it on.
//     assert_eq!(total, 6);
// }


