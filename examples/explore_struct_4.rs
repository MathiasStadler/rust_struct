// FROM HERE
// https://doc.rust-lang.org/book/ch05-01-defining-structs.html

#[allow(dead_code)]
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut _user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    dbg!(&_user1);

    // change vakue inside struct
    _user1.email = String::from("anotheremail@example.com");

    dbg!(&_user1);

    // new instamce with value form _user1
    let _user2 = User {
        active: _user1.active,
        username: _user1.username,
        email: String::from("another@example.com"),
        sign_in_count: _user1.sign_in_count,
    };

    dbg!(&_user2);

    }

// cargo run --example explore_struct_4