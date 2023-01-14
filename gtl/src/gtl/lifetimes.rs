#[derive(Debug)]
struct Test<'a> {
    _data: &'a str
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn lifetimes() {
    let s1 = String::from("abcd");
    let s2 = "xyz";
    let s3: &'static str = "I live always";

    let long = longest(s1.as_str(), s2);
    println!("Longest string is {}", long);

    let test = Test { _data: long };
    println!("{:?} {s3}", test);
}
