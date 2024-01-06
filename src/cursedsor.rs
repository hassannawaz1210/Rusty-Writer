// Implement a cursor struct for our linked list to map the linkedlist data position to the terminal position

use std::rc::Rc;
use std::{borrow::Borrow, cell::RefCell};

use crate::linkedlist::{LinkedList, Node, SIZE};

pub enum Direction {
    Left,
    Right,
}

pub struct Cursedsor {
    current_node: usize, // Present Node in the Linked List
    index: usize,        // Present Index of the char array in the Node
}

impl Cursedsor {
    pub fn new() -> Self {
        Cursedsor {
            current_node: 0,
            index: 0,
        }
    }

    pub fn update(&mut self, term_cursor: usize) {
        self.current_node = (term_cursor / SIZE) + 1;
        self.index = (term_cursor % SIZE);
    }

    pub fn move_cursor(
        &mut self,
        direction: Direction,
        number_of_nodes: usize,
        last: Option<Rc<RefCell<Node>>>, 
        term_cursor: usize,
    ) -> bool {
        self.update(term_cursor);
        match direction {
            Direction::Left => {
                if self.index > 0 {
                    //cursor movement inside current node
                    self.index -= 1;
                    return true;
                } else {
                    //move cursor to the previous node
                    if self.current_node != 0 {
                        //cant move cursor to the left of the first node
                        self.current_node -= 1;
                        self.index = SIZE - 1;
                        return true;
                    }
                }
            }
            Direction::Right => {

                if self.current_node < number_of_nodes {
                    //move cursor to the next node
                    if self.index < SIZE - 1 {
                        //cursor movement inside current node
                        self.index += 1;
                        return true;
                    }
                    else{
                        self.current_node += 1;
                        self.index = 0;
                        return true;
                    }
                }
                else if self.current_node == number_of_nodes {
                    if let Some(last) = last {
                        if self.index < last.borrow_mut().current_index {
                            self.index += 1;
                            return true;
                        }
                    }
                    return false;
                }

                }
            }
            false
        }
        pub fn print(&self) {
            print!("Node: {}", self.current_node);
            print!("Index: {}", self.index);
        }
}
