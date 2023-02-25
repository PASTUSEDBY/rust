use smartptr::*;

fn main() {
    boxes();
    derefs();
    drops();
}

fn boxes() {
    let list = List::from(5);
    let list = list.append(6);
    println!("{:?}", list);
    let list = list.remove().remove().remove().remove();
    println!("{:?}", list);
}

fn derefs() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let test = MyBox::new(String::from("Test"));
    print_str(&test);
}

fn drops() {
    let c = Custom { data: String::from("hello") };
    println!("Created");
    drop(c);
    println!("End of main");
}
