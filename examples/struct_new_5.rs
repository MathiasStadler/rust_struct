// FROM HERE
// https://www.makeuseof.com/rust-structs-guide-work-with/

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    is_student: bool,
}

fn main() {
    let _person = Person {
        name: String::from("John"),
        age: 27,
        is_student: true,
    };

    println!("person {:#?}", _person);
}
