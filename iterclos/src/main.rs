#[allow(unused_imports)]
use std::{thread, time::Duration};

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    closures();
    iterators();
}

fn closures() {
    let expensive_closure = |num: u32| {
        println!("Calculating slowly...");
        //thread::sleep(Duration::from_secs(num.into()));
        num
    };

    let add_one = |x| x + 1;

    println!("{}", expensive_closure(add_one(5u32)));

    let list = vec![1, 2, 3];

    let borrows = move || println!("From thread: {:?}", list);
    //println!("{:?}", list); //This cannot work now
    thread::spawn(borrows).join().unwrap();

    //println!("Outside: {:?}", list); //Neither can this

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut so = vec![];
    let test = String::from("Hello");
    let mut count = 0;

    list.sort_by_key(|r| {
        so.push(&test);
        count += 1;
        r.area()
    });

    println!("{:?}\nTook {} times", list, count);
}

fn iterators() {
    let vec = vec![1, 2, 3];

    let vec_iter = vec.iter().map(|x| x + 1);
    let sum : i32 = vec_iter.sum();

    println!("{sum}");
}
