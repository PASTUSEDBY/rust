use my_project::eat_at_restaurant;
use crate::garden::vegetable::Asparagus;

use std::collections::HashMap;

pub mod garden;

fn main() {
    let plant = Asparagus {
        sweetness: 69
    };
    println!("Your plant is {:?}", plant);
    eat_at_restaurant();

    let mut map = HashMap::new();
    map.insert("Jeremy", 1);

    println!("{:?}", map);
}
