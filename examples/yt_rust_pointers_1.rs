// FROM HERE
// https://www.youtube.com/watch?v=KYJ95TxEC18

struct Node {
    val: i32,
    points_to: Option<Box<Node>>,
}

fn main() {
    let n3 = Box::new(Node {
        val: 3,
        points_to: None,
    });
    let n2 = Node {
        val: 2,
        points_to: Some(n2),
    };
    let n1 = Node {
        val: 1,
        points_to: Some(n1),
    };
}


