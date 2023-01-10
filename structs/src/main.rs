#[derive(Debug)]
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u128
}

struct Point(i32, i32, i32);
#[allow(dead_code)]
struct Test;

#[derive(Debug)]
struct Rectange {
    length: i32,
    width: i32
}

impl Rectange {
    fn is_square(&self) -> bool {
        self.width == self.length
    }

    fn build_square(size : i32) -> Self {
        Self {
            width: size,
            length: size
        }
    }

    fn take_ownership(self) -> Self {
        Self {
            width: self.width,
            length: self.length
        }
    }
}

impl User {
    fn build(email : String, username : String) -> Self {
        Self {
            email,
            username,
            active: true,
            sign_in_count: 1
        }
    }
}

impl User {
    fn is_active(&self) -> bool {
        self.active
    }
}

fn main() {
    let user1 = User::build("name@abc".to_string(), "test".to_string());
    let user2 = User {
        email: "new@test".to_string(),
        ..user1
    };

    println!("{:#?}, {}", user2, user2.is_active());

    let origin = Point(0, 0, 0);
    println!("{}", origin.0);

    let r1 = Rectange::build_square(5);
    let r2 = r1.take_ownership();

    println!("{:#?}, {}", r2, r2.is_square());
}
