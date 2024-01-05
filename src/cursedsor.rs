// Implement a cursor struct for our linked list to map the linkedlist data position to the terminal position

use std::{cell::RefCell, borrow::Borrow};
use std::rc::Rc;

use crate::linkedlist::{LinkedList, Node, SIZE};

pub enum Direction {
    Left,
    Right,
}

pub struct Cursedsor {
    linked_list: Option<Rc<RefCell<Node>>>,
    current_node: (usize, Option<Rc<RefCell<Node>>>), //usize is for current node "number"
    index: usize,
}

impl Cursedsor {
    pub fn new(root: Option<Rc<RefCell<Node>>>) -> Self {
        Cursedsor {
            linked_list: root.clone(),
            current_node: (0, root.clone()),
            index: 0,
        }
    }

    // pub fn increment_index(&mut self){
    //     if self.index >  {
    //         self.index = self.index + 1; 
    //     }
    // }


   pub fn move_cursor(&mut self, direction: Direction, ) -> bool {
        match direction {
            Direction::Left => {
                if self.index > 0 {//cursor movement inside current node
                    self.index -= 1;
                    return true;
                } else {//move cursor to the previous node
                    if self.current_node.0 == 0 { //cant move cursor to the left of the first node
                        return false;
                    }
                    let mut i = self.current_node.0 - 1; //run loop current_node-1 times to get to the previous node
                    self.current_node = (0, self.linked_list.clone());
                    while i > 0 {
                        if let Some(next_node) = &self.current_node.1.as_ref().unwrap().clone().borrow_mut().next {
                            // let b = next_node.clone();
                            self.current_node = (self.current_node.0 + 1, Some(Rc::clone(next_node)));
                        }
                        i -= 1;
                    }
                    self.index = SIZE - 1;
                    return true;
                }
            }
            Direction::Right => {
                if self.index < SIZE - 1 { //cursor movement inside current node
                    self.index += 1;
                    return true;
                } else { //move cursor to the next node
                    if let Some(node) = &self.current_node.1.as_ref().unwrap().clone().borrow_mut().next {
                        self.current_node = (self.current_node.0 + 1, Some(Rc::clone(node)));
                        self.index = 0;
                        return true;
                    }
                }
            }
        }
        false
    }

   pub fn getNextChar(&self) -> char { //return char that is next to current cursor position
        let sh: Option<Rc<RefCell<Node>>> = self.current_node.1.clone();
        if let Some(node) = &sh {
            let bruh = RefCell::borrow_mut(&node);
            bruh.data[self.index]
        } else {return '\n';}
    }

    pub fn print(&self){
        print!("Node: {:#?}",self.current_node);
        print!("Index: {}", self.index);
    }

}