use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::cell::RefMut;
use std::char;
use std::fs::File;
use std::io::{self, Write};
use std::rc::Rc;

pub const SIZE: usize = 10;

#[derive(Debug)]
pub struct Node {
    pub data: [char; SIZE],
    pub next: Option<Rc<RefCell<Node>>>,
    pub current_index: usize,
    pub num: usize,
}

impl Node {
    pub fn new(ch: [char; SIZE], n: usize) -> Self {
        Node {
            data: ['\0'; SIZE],
            next: None,
            current_index: 0,
            num: n,
        }
    }

    pub fn add_char(&mut self ,ch:char) {
        if !self.is_full(){
            self.data[self.current_index] = ch;
            self.current_index += 1;
        }
    }

    pub fn is_full(&self) -> bool {
        if self.current_index == SIZE {
            return true;
            
        }
        false
    }
}

#[derive(Debug)]
pub struct LinkedList {
    pub root: Option<Rc<RefCell<Node>>>,
    last: Option<Rc<RefCell<Node>>>,
    number_of_nodes: usize,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList {
            root: None,
            last: None,
            number_of_nodes: 0,
        }
    }

    pub fn insert(&mut self, data: [char; SIZE]) {
        let boxed_node = Some(Rc::new(RefCell::new(Node::new(data, 0))));
        // if the root and last are none
        if self.root.is_none() {
            self.root = boxed_node.clone();
            self.last = boxed_node.clone();
            self.number_of_nodes += 1;
        }
        // Insert another node
        else {
            if let Some(last) = &self.last {
                let mut node_ref = RefCell::borrow_mut(&last);
                (*node_ref).next = boxed_node.clone();
            }
            self.last = boxed_node.clone();
            self.number_of_nodes += 1;
        }
    }

    pub fn new_node(&mut self, n :Option<Rc<RefCell<Node>>>) {
        // if the root and last are none
        if self.root.is_none() {
            self.root = n.clone();
            self.last = n.clone();
            self.number_of_nodes += 1;
        }
        // Insert another node
        else {
            if let Some(last) = &self.last {
                let mut node_ref = RefCell::borrow_mut(&last);
                (*node_ref).next = n.clone();
            }
            self.last = n.clone();
            self.number_of_nodes += 1;
        }
    }

    pub fn update_node_numbers(&mut self, current_node: usize, other_node: usize) {
        let mut current = self.root.clone();
        //find the node with node with num = other_node, then replace it with current_node
        //use loop loop
        loop {
            if let Some(node) = current {
                let mut node_ref = RefCell::borrow_mut(&node);
                if node_ref.num == other_node {
                    node_ref.num = current_node;
                    break;
                }
                current = node_ref.next.clone();
            }
        }
        
    }


    pub fn get_last(&mut self) -> Option<Rc<RefCell<Node>>> {
        self.last.clone()
    }

    pub fn get_row_len(&mut self) -> usize {
       //count the number of characters in linkedlist
       //fukin copepilot 
         let mut current = self.root.clone();
            let mut count = 1;
            count *= self.number_of_nodes  - 1;
            count *= SIZE;
            if let Some(node) = &self.last {
                let node_ref = RefCell::borrow_mut(&node);
                count += node_ref.current_index;
            }
            count
    }


    pub fn write_to_file(&self, f: &mut File) {
        let mut current = self.root.clone();
        while let Some(node) = current {
            let node_ref = RefCell::borrow_mut(&node);
            let data_string: String = node_ref.data.iter().filter(|&&c| c != '\0').collect();
            print!("{}", data_string);
            if let Err(err) = f.write(data_string.as_bytes()) {
                panic!("Failed to write to file: {}", err);
            }
            current = node_ref.next.clone();
        }
    }

    pub fn write_to_file_by_num(&self, f: &mut File) {
      //write nodes to file according to their numbers in ascending order
        let mut count = 0;
        while count < self.number_of_nodes {
            let mut current = self.root.clone();
            while let Some(node) = current {
                let node_ref = RefCell::borrow_mut(&node);
                if node_ref.num == count {
                    let data_string: String = node_ref.data.iter().filter(|&&c| c != '\0').collect();
                    print!("{}", data_string);
                    if let Err(err) = f.write(data_string.as_bytes()) {
                        panic!("Failed to write to file: {}", err);
                    }
                    break;
                }
                current = node_ref.next.clone();
            }
            count += 1;
        }
    }

    pub fn get_number_of_nodes(&self) -> usize {
        self.number_of_nodes
    }
}
