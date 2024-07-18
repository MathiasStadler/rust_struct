// FROM HERE
// https://doc-rust--lang-org.translate.goog/book/ch05-02-example-structs.html?_x_tr_sl=auto&_x_tr_tl=en&_x_tr_hl=de
// 5.2

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
    println!("{:#?}",&rect1);
}

// cargo run --example area_5