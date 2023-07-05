#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut user1 = User {
        email: String::from("mingleeng@gmail.com"),
        username: String::from("Redcon1"),
        sign_in_count: 1,
        active: true,
    };

    let name = user1.username;
    user1.username = String::from("Universal_Lee");

    let user2 = build_user(String::from("ngmingl@gmail.com"), String::from("ngmingl"));

    let user3 = User {
        email: String::from("thing1@seuss.com"),
        username: String::from("ThingOne"),
        ..user2
    };

    println!(
        "name: {},\nuser3 (\n  username: {},\n  email: {},\n  sign in count: {},\n  active: {}\n)",
        name, user3.username, user3.email, user3.sign_in_count, user3.active
    );
    println!("{:?}", user2);

    let width1: u32 = 30;
    let height1: u32 = 50;
    let rect = Rectangle {
        width: width1,
        height: height1,
    };

    println!("The area of the rectangle is {} square pixels", area(&rect));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}
