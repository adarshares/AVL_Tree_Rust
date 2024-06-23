#![allow(warnings)]

use std::{collections::btree_map::Values, ops::Deref};

#[derive(Debug)]
struct Node {
    value: i32,
    height: i32,
    left_child: Option<Box<Node>>,
    right_child: Option<Box<Node>>,
}

impl Node {
    fn new(new_val:i32) -> Box<Node> {
        Box::new(Node{
            value:new_val,
            height: 1,
            left_child: None,
            right_child: None,
        })
    }

    fn get_height(root: &Option<Box<Node>>) -> i32 {
        match root {
            Some(root_node) => (*root_node).height,
            None => 0,
        }
    }
    
    fn max_height(left_child: &Option<Box<Node> >,right_child: &Option<Box<Node>>) -> i32 {
        let left_child_height = Self::get_height(left_child);
        let right_child_height = Self::get_height(right_child);
        if left_child_height > right_child_height {
            return left_child_height;
        }
        return right_child_height;
    }

    fn update_node_height(mut root: Option<Box<Node>>) -> Box<Node> {
        match root {
            Some(mut root_node) => {
                root_node.height = 1+Self::max_height(&((*root_node).right_child), &((*root_node).left_child));
                return root_node;
            },
            None => {panic!("cannote update height of None");}
        }
    }

    fn rotate_left(mut root: Option<Box<Node>>) -> Option<Box<Node>>{
        match root {
            Some(mut root_node) => {
                match (*root_node).right_child {
                    Some(mut root_right_child) => {
                        match (*root_right_child).left_child {
                            Some(root_right_left_child)=>{
                                (*root_node).right_child = Some(root_right_left_child);
                                root_node = Self::update_node_height(Some(root_node));

                                (*root_right_child).left_child = Some(root_node);
                                root_right_child = Self::update_node_height(Some(root_right_child));
                                return Some(root_right_child);
                            }
                            None => {
                                //root right left child does not exist
                                (*root_node).right_child = None;
                                root_node = Self::update_node_height(Some(root_node));
                                
                                (*root_right_child).left_child = Some(root_node);
                                root_right_child = Self::update_node_height(Some(root_right_child));
                                return Some(root_right_child);
                            }
                            
                        }
                    },
                    None => panic!("no right child present!!!"),
                }
            },
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
                                root_node = Self::update_node_height(Some(root_node));

                                (*root_left_child).right_child = Some(root_node);
                                root_left_child = Self::update_node_height(Some(root_left_child));
                                return Some(root_left_child);
                            }
                            None => {
                                (*root_node).left_child = None;
                                root_node = Self::update_node_height(Some(root_node));

                                (*root_left_child).right_child = Some(root_node);
                                root_left_child = Self::update_node_height(Some(root_left_child));
                                return Some(root_left_child);
                            }
                        }
                    }
                    None => panic!("no left child present!!!"),
                }
            },
            None => {return None;},
        }
    }

    fn balance_node(mut root_node:Box<Node>,value:i32) -> Option<Box<Node>> {
        let balance_factor = Self::get_height(&(*root_node).left_child) - Self::get_height(&(*root_node).right_child);
        //println!("{}",balance_factor);

        if balance_factor > 1 {
            match &(*root_node).left_child { //fucking insane error //understand partial borrowing
                Some( root_left_child) => {
                    if value < (*root_left_child).value {
                        return Self::rotate_right(Some(root_node));
                    }
                    else if value > (*root_left_child).value {
                        (*root_node).left_child = Self::rotate_left((*root_node).left_child);
                        return Self::rotate_right(Some(root_node));
                    } else{
                        return Some(root_node);
                    }
                }
                None => {return Some(root_node);}
            }
        } else if balance_factor < -1 {
            match &(*root_node).right_child { //fucking insane error //understand partial borrowing
                Some( root_right_child) => {
                    if value > (*root_right_child).value {
                        return Self::rotate_left(Some(root_node));
                    }
                    else if value < (*root_right_child).value {
                        (*root_node).right_child = Self::rotate_right((*root_node).right_child);
                        return Self::rotate_left(Some(root_node));
                    } else{
                        return Some(root_node);
                    }
                }
                None => {return Some(root_node);}
            }
        } else {
            return Some(root_node);
        }
    }

    fn insertself(&mut self,value:i32){

        //let x = self;

        if self.value < value { 
            //go right
        } else if self.value > value {
            //go left
            match &mut self.left_child {
                Some(left_child) => {
                    left_child.insertself(value);
                }
                None => {
                    self.left_child = Some(Node::new(value));
                }
            }
        } else {
            //leave
        }

    }

    fn insert(mut root:Option<Box<Node>>, value:i32) -> Option<Box<Node>> {
        match root {
            Some(mut root_node)=> {
                if((*root_node).value < value){
                    match (*root_node).right_child {
                        Some(root_right_child) => {
                            (*root_node).right_child = Self::insert(Some(root_right_child),value);
                            root_node = Self::update_node_height(Some(root_node));

                            return Self::balance_node(root_node, value);
                        }
                        None => {
                            (*root_node).right_child = Some(Node::new(value));
                            root_node = Self::update_node_height(Some(root_node));
                            return Self::balance_node(root_node, value);
                            //return Some(root_node);
                        }
                    }
                }
                else {
                    match (*root_node).left_child {
                        Some(root_left_child) => {
                            (*root_node).left_child = Self::insert(Some(root_left_child),value);
                            root_node = Self::update_node_height(Some(root_node));

                            return Self::balance_node(root_node, value);
                        }
                        None => {
                            (*root_node).left_child = Some(Node::new(value));
                            root_node = Self::update_node_height(Some(root_node));
                            return Self::balance_node(root_node, value);
                            //return Some(root_node);
                        }
                    }
                }
            }
            None => {return Some(Node::new(value));}
        }
    }
    
    fn print_sorted(root:Option<Box<Node>>) {
        match root {
            Some(root_node) => {
                Self::print_sorted(root_node.left_child);
                print!("{} ",root_node.value);
                Self::print_sorted(root_node.right_child);
            },
            None => {return;}
        }
    }


}

struct Set {
    node:Option<Box<Node>>,
}

impl Set {
    fn new(new_val:Option<i32>) -> Set {
        match new_val {
            Some(value) => {
                return Set {
                    node:Some(Node::new(value)),
                };
            }
            None => {
                return Set{
                    node:None,
                };
            }
        }
    }
    fn insert(&mut self,value:i32) {
        match &mut self.node {
            Some(root) => {
                if root.value > value {
                    //go left
                    match &mut root.left_child {
                        Some(root_left_child) => {
                            
                        }
                        None => {
                            root.left_child = Self::new(Some(value));
                        }
                    }
                } else if root.value < value {
                    //go right
                } else {
                    //leave case
                }
            }
            None => {
                self.node = Some(Node::new(Some(value)));
            }
        }
    }
}

struct Map (Option<Box<Node>>);

impl Deref for Map {
    type Target = Option<Box<Node>>;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl Map {

    fn new(new_val:Option<i32>) -> Option<Box<Node>> {
        match new_val {
            Some(value) => {
                return Some(Node::new(value));
            }
            None => {return None;}
        }
    }
    fn insert(&mut self,value:i32) {
        match *self {
            Some(root) => {
                if root.value > value {
                    //go left
                    match &mut root.left_child {
                        Some(root_left_child) => {
                            
                        }
                        None => {
                            root.left_child = Self::new(Some(value));
                        }
                    }
                } else if root.value < value {
                    //go right
                } else {
                    //leave case
                }
            }
            None => {
                self.node = Self::new(Some(value));
            }
        }
    }
}

fn main() {
    let mut root = Node::new(9);
    // root = Node::insert(root,8);
    // root = Node::insert(root,7);
    // root = Node::insert(root,6);
    // root = Node::insert(root,5);
    // root = Node::insert(root,4);
    // root = Node::insert(root,3);
    // Node::print_sorted(root);
    //println!("{:?}",root);
    
}
