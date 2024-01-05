// Implement a cursor struct for our linked list to map the linkedlist data position to the terminal position

use std::{cell::RefCell, borrow::Borrow};
use std::rc::Rc;

use crate::linkedlist::{LinkedList, Node, SIZE};

pub enum Direction {
    Left,
    Right,
}

pub struct Cursedsor {
    current_node: usize,
    index: usize,
}

impl Cursedsor {
    pub fn new(root: Option<Rc<RefCell<Node>>>) -> Self {
        Cursedsor {
            current_node: 0,
            index: 0,
        }
    }

   pub fn move_cursor(&mut self, direction: Direction, number_of_nodes: usize) -> bool {
        match direction {
            Direction::Left => {
                if self.index > 0 {//cursor movement inside current node
                    self.index -= 1;
                    return true;
                } else {//move cursor to the previous node
                    if self.current_node != 0 { //cant move cursor to the left of the first node
                        self.current_node -= 1;
                        self.index = SIZE - 1;
                        return true;
                    }
            }
        }
            Direction::Right => {
                if self.index < SIZE - 1 { //cursor movement inside current node
                    self.index += 1;
                    return true;
                } else { //move cursor to the next node
                    if(self.current_node < number_of_nodes) {
                        self.current_node += 1;
                        self.index = 0;
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn print(&self){
        print!("Node: {}",self.current_node);
        print!("Index: {}", self.index);
    }

}