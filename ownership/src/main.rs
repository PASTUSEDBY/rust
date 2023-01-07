fn main() {
    let mut s1 = String::from("hello");
    let length = len(&s1);

    println!("\"{s1}\" has length {length}");

    change(&mut s1);
    let length = len(&s1);
    println!("\"{s1}\" has length {length}");

    let s2 = first_word(&s1);

    println!("First word is {}, with length {}", s2, s2.len());
}

fn len(str : &String) -> usize {
    str.len()
}

fn change(s : &mut String) {
    s.push_str(" world");
}

fn first_word(s : &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s
}
