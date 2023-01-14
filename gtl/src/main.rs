pub mod gtl;

use crate::gtl::generics::generics;
use crate::gtl::lifetimes::lifetimes;
use crate::gtl::traits::traits;

fn main() {
    println!("Generics");
    generics();
    println!("Traits");
    traits();
    println!("Lifetimes");
    lifetimes();
}
