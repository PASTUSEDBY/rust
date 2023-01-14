use std::fmt::Debug;

trait Functor<T> {
    type Func<U>;

    fn map<F, U>(&self, f: F) -> Self::Func<U>
    where F: Fn(&T) -> U;
}

impl<T> Functor<T> for Vec<T> {
    type Func<U> = Vec<U>;

    fn map<F, U>(&self, f: F) -> Self::Func<U>
        where F: Fn(&T) -> U {
        let mut vec = Vec::new();

        for elem in self {
            vec.push(f(elem));
        }

        vec
    }
}

impl<T> Functor<T> for Option<T> {
    type Func<U> = Option<U>;

    fn map<F, U>(&self, f: F) -> Self::Func<U>
        where F: Fn(&T) -> U {
        match self {
            None => None,
            Some(x) => Some(f(x))
        }
    }
}

trait Summary {
    fn summarize(&self) -> String;
}

trait Test {
    fn test(&self);
}

impl<T: Debug> Test for T {
    fn test(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
struct Article {
    author: String,
    content: String
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.content, self.author)
    }
}

fn notify<T>(item: &T) 
where T: Summary + Debug {
    println!("{}", item.summarize());
}

pub fn traits() {
    let article = Article {
        author: String::from("Someone"),
        content: String::from("Today's weather is clear.")
    };

    notify(&article);
    article.test();
    3.test();
    "Hello".test();
    let a : [String;0] = [];
    a.test();
    Some(&a).test();

    let vec = vec!["Hello", "World", "This", "Is"];

    println!("{:?}", vec.map(|x| x.len()));
}
