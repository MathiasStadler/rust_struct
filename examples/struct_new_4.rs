// defining methods inside struct

#[derive(Debug)]
struct Rectangle {
    #[allow(dead_code)]
    width: u32,
    #[allow(dead_code)]
    height: u32,
}


impl Rectangle {
    

    fn new(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
   

    // https://rust-unofficial.github.io/patterns/idioms/ctor.html
    let _new = Rectangle::new(5);
    println!("new {:?}", _new);
}

// cargo run --example area_7

//HERE futures
// Listing 5-14: Using the as-yet-unwritten can_hold method
// https://doc-rust--lang-org.translate.goog/book/ch05-03-method-syntax.html?_x_tr_sl=auto&_x_tr_tl=en&_x_tr_hl=de


// https://www.makeuseof.com/rust-structs-guide-work-with/