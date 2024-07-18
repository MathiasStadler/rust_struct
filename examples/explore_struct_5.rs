#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
    dbg!(subject);
}

// cargo run --example explore_struct_5