use std::collections::HashMap;

fn main() {
    vectors();
    strings();
    hashmaps();
}

fn vectors() {
    let vn = vec![1, 2, 3];
    println!("{:?}", vn);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);

    if let Some(x) = v.get(0) {
        println!("I got {x}");
    }

    for i in &vn {
        print!("{i} ");
    }

    println!("\n{:?}", v);

    for i in &mut v {
        *i += 5;
    }

    println!("{:?}", v);
}

fn strings() {
    let hello = String::from("नमस्ते");

    let mut s1 = String::from("po");
    s1.push('g');

    let s3 = hello + " " + &s1;

    println!("{s3}");

    for ch in s1.chars() {
        print!("{ch} ");
    }

    println!();
}

fn hashmaps() {
    let mut results: HashMap<String, u8> = HashMap::new();

    results.insert("James".to_string(), 0);
    results.insert("Jessy".to_string(), 100);

    results.entry("James".to_string()).or_insert(100);
    results.entry("Meowth".to_string()).or_insert(50);

    for (key, value) in &results {
        println!("{key} : {value}");
    }

    let text = "Some text I decided to text you with you";

    let mut counts: HashMap<String, u8> = HashMap::new();

    for word in text.split_whitespace() {
        let count = counts.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", counts);
}
