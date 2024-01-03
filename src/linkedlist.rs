use std::fs::File;
use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::{BorrowMut, Borrow};
use std::io::{self, Write};


const SIZE: usize = 10;

#[derive(Debug)]
pub struct Node{
    pub data: [char; SIZE],
    pub next: Option<Rc<RefCell<Node>>>,
    pub current_index: usize,
}

impl Node{
    fn new(ch: char) -> Self{
        let mut node = Node{
            data: Default::default(),
            next: None,
            current_index : 0,
        };
        node.add_char(ch);
        return node;
    }

    fn add_char(&mut self ,ch:char) {
        self.data[self.current_index] = ch; 
        self.current_index += 1;
    }

    fn remove_char(&mut self){
        
    }

    fn is_full(&self) -> bool {
        if self.current_index >= SIZE {
            true
        } else {
            false
        }
    }
    
}


impl Write for Node {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for &byte in buf {
            let ch = byte as char;
            self.data[self.current_index] = ch; 
            self.current_index += 1;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
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

    pub fn insert(&mut self, data: char) {
        if self.root.is_none() {
            
            let mut node = Node::new(data);
            let mut boxed_node = Some(Rc::new(RefCell::new(node))); // Wrapping the Node in Option then Rc and Refcells
            
            self.root = boxed_node.clone();
            self.last = boxed_node.clone();
        } 

        else if !self.last.clone().unwrap().as_ref().borrow().is_full() {
            self.last.clone().unwrap().as_ref().borrow_mut().write(&(data as u8).to_be_bytes());
        }

        else {
            
            let mut node = Node::new(data); // Simple Node 
            let mut boxed_node = Some(Rc::new(RefCell::new(node))); 
            if let Some(last) = &self.last { // Unwrapping self.last if it has some value 
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
            if let Err(err) = f.write_all(data_string.as_bytes()) {
                panic!("Failed to write to file: {}", err);
            }
            current = node_ref.next.clone();
        } 
    }


}


impl Write for LinkedList {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for &byte in buf {
            let ch = byte as char;
            self.insert(ch);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        // Since there's no buffering in the LinkedList, we don't need to do anything here
        Ok(())
    }

}
