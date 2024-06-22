#![allow(warnings)]

struct Node {
    value: i32,
    height: i32,
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

    fn rotate_left(mut root: Option<Box<Node>>) -> Option<Box<Node>>{
        match root {
            Some(mut root_node) => {
                match ((*root_node).right_child) {
                    Some(mut root_right_child) => {
                        match ((*root_right_child).left_child) {
                            Some(root_right_left_child)=>{
                                (*root_node).right_child = Some(root_right_left_child);
                                (*root_right_child).left_child = Some(root_node);
                                return Some(root_right_child);
                            }

                            None => {
                                //root right left child does not exist
                                (*root_node).right_child = None;
                                (*root_right_child).left_child = Some(root_node);
                                return Some(root_right_child);
                            }
                            
                        }
                    },
                    None => panic!("no right child present!!!"),
                }
            }
            None => {return None;},
        }
    }
    fn rotate_right(mut root: Option<Box<Node>>) -> Option<Box<Node>>{
        match root {
            Some (mut root_node) => {
                match (*root_node).left_child {
                    Some(mut root_left_child) => {
                        match (*root_left_child).right_child {
                            Some(root_left_right_child) => {
                                (*root_node).left_child = Some(root_left_right_child);
                                (*root_left_child).right_child = Some(root_node);
                                return Some(root_left_child);
                            }
                            None => {
                                (*root_node).left_child = None;
                                (*root_left_child).right_child = Some(root_node);
                                return Some(root_left_child);
                            }
                        }
                    }
                    None => panic!("no left child present!!!"),
                }
                return None;
            },
            None => {return None;},
        }
    }

    


}

fn main() {
    println!("Hello, world!");
}
