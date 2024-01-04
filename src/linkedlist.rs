use std::char;
use std::fs::File;
use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::{BorrowMut, Borrow};
use std::io::{self, Write};

pub const SIZE: usize = 10;

#[derive(Debug)]
pub struct Node{
    pub data: Rc<[char; SIZE]>,
    pub next: Option<Rc<RefCell<Node>>>,
    pub current_index: usize,
}

impl Node{

    fn new(ch:Rc<[char; SIZE]>  ) -> Self{
       Node{
            data: Rc::clone(&ch),
            next: None,
            current_index : 0,
        }
    }

    // fn add_char(&mut self ,ch:char) {
    //     if !self.is_full(){
    //         self.data[self.current_index] = ch; 
    //         self.current_index += 1;
    //     }
    // }

    fn is_full(&self) -> bool {
        if self.current_index >= SIZE {
            true
        } else {
            false
        }
    }
    
}


#[derive(Debug)]
pub struct LinkedList{
    root: Option<Rc<RefCell<Node>>>,
    last: Option<Rc<RefCell<Node>>>,
}

impl LinkedList{
    pub fn new () -> Self{
        LinkedList{root: None, last: None}
    }

    pub fn insert(&mut self, data: Rc<[char; SIZE]>) {
        let boxed_node = Some(Rc::new(RefCell::new(Node::new(data))));
        // if the root and last are none
        if self.root.is_none() {
            self.root = boxed_node.clone();
            self.last = boxed_node.clone();
        }
        // Insert another node 
        else {
            if let Some(last) = &self.last { 
                let mut node_ref = RefCell::borrow_mut(&last); 
                (*node_ref).next = boxed_node.clone();
            }
            self.last = boxed_node.clone(); 
        }
    }



    pub fn write_to_file(&self, f: &mut File) {
        let mut current = self.root.clone();
        while let Some(node) = current {
            let node_ref   = RefCell::borrow_mut(&node);
            let data_string: String = node_ref.data.iter().filter(|&&c| c != '\0').collect();
            print!("{}", data_string);
            if let Err(err) = f.write(data_string.as_bytes()) {
                panic!("Failed to write to file: {}", err);
            }
            current = node_ref.next.clone();
        } 
    }


}