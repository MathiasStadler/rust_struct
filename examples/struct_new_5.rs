// FROM HERE
// https://www.makeuseof.com/rust-structs-guide-work-with/

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    is_student: bool,
    
}

impl Person {

   

    // Define a constructor function `new` that takes `name`, `age` and
    // `is_student` parameters 
    fn new(name: String, age: u8, is_student: bool) -> Self {
        // Create a new instance of `Person` struct and initialize its fields
        // with the provided values 
        Self {
            name,        
            age,        
            is_student,
        }
    }
}

fn main() {
    let _person = Person {
        name: String::from("John"),
        age: 27,
        is_student: true,
    };

    println!("person {:#?}", _person);

    let _person_new = Person::new(String::from("John"), 27, true);

    println!("person {:#?}", _person_new);
}
