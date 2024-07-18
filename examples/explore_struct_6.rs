// lifetime
#[derive(Debug)]
struct User<'a>  {
    active: bool,
    username: &'a str,
    email:&'a str,
    sign_in_count: u64,
}

// store refeuse of lifetimes, a Rust feature that we’ll discuss in Chapter 10.
fn main() {
    let _user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
    dbg!(_user1);
}

// cargo run --example explore_struct_6