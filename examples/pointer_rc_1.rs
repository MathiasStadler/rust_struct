// FROM HERE
// https://www.youtube.com/watch?v=mNHdD69iLzA

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

struct Owner {
    name: String,
    tools: RefCell<Vec<Weak<Tool>>>,
}

struct Tool {
    owner: Rc<Owner>,
}

pub fn main() {
    let brad = Rc::from(Owner {
        name: "Brad".to_string(),
        tools: RefCell::new(vec![]),
    });

    // ?? Has the clone statement here the same function
    // german => pliers ->Zange
    let pliers = Rc::from(Tool {
        owner: Rc::clone(&brad),
    });
    // german => wrench -> Schlüessel
    let wrench = Rc::from(Tool {
        owner: brad.clone(),
    });

    brad.tools.borrow_mut().push(Rc::downgrade(&pliers));
    brad.tools.borrow_mut().push(Rc::downgrade(&wrench));

    println!("Pliers owner: {} ", pliers.owner.name);
    println!(
        "Brad's pliers owner: {:?}",
        brad.tools.borrow()[0].upgrade().unwrap().owner.name
    );
}
 // https://www.makeuseof.com/rust-structs-guide-work-with/
 //https://www.linkedin.com/pulse/constructors-static-function-rust-amit-nadiger
 // https://github.com/danbugs/
 //https://www.youtube.com/watch?v=mNHdD69iLzA
 // https://www.youtube.com/watch?v=KYJ95TxEC18