use std::ops::Deref;

#[derive(Debug)]
pub enum List<T> {
    Empty,
    Cons(T, Box<List<T>>)
}

impl<T> List<T> {
    pub fn from(data: T) -> Self {
        List::Cons(data, Box::new(List::Empty))
    }

    pub fn append(self, data: T) -> Self {
        List::Cons(data, Box::new(self))
    }

    pub fn remove(self) -> Self {
        match self {
            List::Empty => self,
            List::Cons(_, inner) => *inner
        }
    }
}

#[derive(Debug)]
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn print_str(x: &str) {
    println!("{x}");
}

#[derive(Debug)]
pub struct Custom {
    pub data: String
}

impl Drop for Custom {
    fn drop(&mut self) {
        println!("Data has been thrown off now, {}", self.data);
    }
}
