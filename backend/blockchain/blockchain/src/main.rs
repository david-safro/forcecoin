mod block;
mod blockchain;
mod node;

use node::Node;
use std::thread;

fn main() {
    let node1 = Node::new();
    thread::spawn(move || {
        node1.start("127.0.0.1:8000");
    });

    let node2 = Node::new();
    thread::spawn(move || {
        node2.start("127.0.0.1:8001");
    });

    loop {}
}
