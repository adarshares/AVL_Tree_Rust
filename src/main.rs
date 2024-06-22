#![allow(warnings)]

struct Node {
    value: i32,
    height: u32,
    left_child: Option<Box<Node>>,
    right_child: Option<Box<Node>>,
}

impl Node {
    fn new(new_val:i32) -> Option<Box<Node>> {
        Some(Box::new(Node{
            value:new_val,
            height: 0,
            left_child: None,
            right_child: None,
        }))
    }

    fn rotate_right(){}
    fn rotate_left(){}

    


}

fn main() {
    println!("Hello, world!");
}
