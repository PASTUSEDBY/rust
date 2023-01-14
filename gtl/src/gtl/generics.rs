use std::cmp::{PartialOrd, PartialEq};

#[derive(PartialEq)]
struct Point<T> {
    x : T,
    y : T
}

fn smallest<T: PartialOrd>(arr : &[T]) -> &T {
    if arr.len() == 0 {
        panic!("Array length is zero, cannot determine smallest element!");
    }

    let mut smallest = &arr[0];

    for elem in arr {
        if elem < smallest {
            smallest = elem;
        }
    }

    smallest
}

pub fn generics() {
    let arr = [5, 2, 3, -1, 4];
    println!("{}", smallest(&arr));

    let p1 = Point { x: 5.9, y: 4.9 }; 
    let p2 = &p1;
    let p3 = Point { y: 6.0, ..p1 };

    println!("{}, {}", p1 == *p2, p3 == *p2);
}
