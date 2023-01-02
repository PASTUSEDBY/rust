use std::io;

const TEMP_TODAY : u32 = 273 + 21;

fn main() {
    test();
    shadow();
    types();
    println!("{}", add(1.5, 2.5));
    conditions();
    loops();
    named_loops();
}


fn test() {
    let output = "The value of x is:";
    let mut x = 5;
    println!("{} {}", output, x);
    x = 6;
    println!("{} {}", output, x);
    println!("Today's temperature is {} K", TEMP_TODAY);
}

fn shadow() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 5;
        println!("Inner scope has {}", x);
    }

    println!("This is x: {}", x);
}

fn types() {
    let tup = (1, 6.9, 'A');
    let y = tup.1;

    println!("y is: {}", y);

    let a : [u8; 5] = [6, 2, 10, 20, 100];
    println!("{:?} and {:?}", &a, &[6; 10]);

    println!("Enter index to access element from first array:");

    let mut index = String::new();

    io::stdin()
    .read_line(&mut index)
    .expect("What are you doing..?");

    let index : usize = match index.trim().parse() {
        Ok(x) => x,
        Err(_) => return
    };

    println!("{}", a[index]);
}

fn add(x : f64, y : f64) -> f64 {
    if y < x {
        return 0.0;
    }

    let z = {
        let s = x + y;
        s * 2.0
    };

    z / 2.0
}

fn conditions() {
    let num = 3;

    if num < 5 {
        println!("{num} is lower than 5");
    } else {
        println!("{num} is not lower than 5");
    }

    let z = if num < 5 { num * 3 } else { num / 3 };
    println!("{z}");
}

fn loops() {
    let mut counter = 0;

    let res = loop {
        counter += 1;

        if counter > 10 {
            break -(counter - 1);
        }
    };

    println!("This is {res}");

    for elem in (1..=4).rev() {
        println!("{}", elem * 10);
    }
}

fn named_loops() {
    //just to remember this exists
    let mut count = 0;
    let res = 'test : loop {
        let mut z = 21;
        loop {
            if z == 5 {
                break 'test count * 5;
            }

            z -= 1;
            count += z;

            if count % z == 0 {
                continue;
            }
        };
    };

    println!("{res}");
}
