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

    brad.tools.barrow_mut().push(Rc::downgrade(pliers));
    brad.tools.barrow_mut().push(Rc::downgrade(wrench));

    println!("Pliers owner: {} ", pliers.owner.name);
    println!(
        "Brad's pliers owner: {}",
        brad.tools.barrow_mut()[0].upgrade()
    );
}
