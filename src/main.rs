#![allow(warnings)]

use core::num;
use std::{fs::File, io::{BufRead, BufReader}, time::Instant};

#[derive(Debug)]
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
            height: 1,
            left_child: None,
            right_child: None,
        }))
    }

    fn get_height(root: &Option<Box<Node>>) -> i32 {
        match root {
            Some(root_node) => (*root_node).height,
            None => 0,
        }
    }
    
    fn max_height(left_child:& Option<Box<Node> >,right_child: &Option<Box<Node>>) -> i32 {
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
                            (*root_node).right_child = Node::new(value);
                            root_node = Self::update_node_height(Some(root_node));
                            return Self::balance_node(root_node, value);
                            //return Some(root_node);
                        }
                    }
                } else if ((*root_node).value > value){
                    match (*root_node).left_child {
                        Some(root_left_child) => {
                            (*root_node).left_child = Self::insert(Some(root_left_child),value);
                            root_node = Self::update_node_height(Some(root_node));

                            return Self::balance_node(root_node, value);
                        }
                        None => {
                            (*root_node).left_child = Node::new(value);
                            root_node = Self::update_node_height(Some(root_node));
                            return Self::balance_node(root_node, value);
                            //return Some(root_node);
                        }
                    }
                }
                else{
                    return Some(root_node);
                }
            }
            None => {panic!("root is NULL");}
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

#[derive(Debug)]
struct Set {
    root: Option<Box<Node>>,
}

impl Set {
    fn new(new_value:Option<i32>) -> Set {
        match new_value {
            Some(value) => {
                return Set {
                    root: Node::new(value)
                }
            }
            None => {
                return Set{
                    root: None,
                }
            }
        }
    }

    fn insert(&mut self, new_value:i32) {
        match self.root.take() {
            Some(root_node) => {
                self.root = Node::insert(Some(root_node), new_value);
            }
            None => {
                self.root = Node::new(new_value);
            }
        }
    }
    fn pnt(&mut self){
        Node::print_sorted(self.root.take());
    }
}

fn main() {
    // Open the file
    let file = File::open("random_numbers.txt").unwrap();
    let reader = BufReader::new(file);
    
    // Vector to store the integers
    let mut numbers:Vec<i32> = Vec::new();
    
    // Read each line and parse it as an integer
    for line in reader.lines() {
        let line = line.unwrap();
        if let Ok(number) = line.trim().parse::<i32>() {
            numbers.push(number);
        }
    }
    let x = numbers.len();
    
    let mut s = Set::new(None);
    
    let start = Instant::now();
    
    for number in numbers {
        s.insert(number);
    }

    // Stop measuring time
    let duration = start.elapsed();
    println!("Time taken to read and parse the file: {:?}", duration);
    
    // Output the vector size to verify the result
    println!("Number of integers read: {}", x);
    
    // let mut root = Node::new(9);
    // root = Node::insert(root,8);
    // root = Node::insert(root,7);
    // root = Node::insert(root,6);
    // root = Node::insert(root,5);
    // Node::print_sorted(root);
}