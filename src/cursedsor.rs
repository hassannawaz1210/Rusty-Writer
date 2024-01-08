// Implement a cursor struct for our linked list to map the linkedlist data position to the terminal position

use std::rc::Rc;
use std::{borrow::Borrow, cell::RefCell};

use crate::linkedlist::{LinkedList, Node, SIZE};
use crossterm::cursor::position;

//===============================================

pub struct TerminalCursor {
    x: usize,
    y: usize,
}

impl TerminalCursor {
    pub fn new() -> Self {
        TerminalCursor { x: 0, y: 0 }
    }
    pub fn update(&mut self) {
        let (x, y) = crossterm::cursor::position().unwrap();
        self.x = x as usize;
        self.y = y as usize;
    }
}

//================================================

pub enum Direction {
    Left,
    Right,
}

pub struct Cursedsor {
    current_node: usize, // Present Node in the Linked List
    index: usize,        // Present Index of the char array in the Node
    termCursor: TerminalCursor,
}

impl Cursedsor {
    pub fn new() -> Self {
        Cursedsor {
            current_node: 0,
            index: 0,
            termCursor: TerminalCursor::new(),
        }
    }

    pub fn getX(&self) -> usize {
        self.termCursor.x
    }

    pub fn getY(&self) -> usize {
        self.termCursor.y
    }

    pub fn get_node(&self) -> usize {
        self.current_node
    }

    pub fn update(&mut self) {
        self.termCursor.update();
        self.current_node = (self.termCursor.x / SIZE) + 1;
        self.index = (self.termCursor.x % SIZE);
    }
//  this whole func is useless 
    pub fn move_cursor(
        &mut self,
        direction: Direction,
        number_of_nodes: usize,
        last: Option<Rc<RefCell<Node>>>,
    ) -> bool {
        self.update();

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

            //  All this is useless XD

            Direction::Right => {
                if self.current_node < number_of_nodes {
                    //move cursor to the next node
                    if self.index < SIZE {
                        //cursor movement inside current node
                        self.index += 1;
                        return true;
                    } else {
                        self.current_node += 1;
                        self.index = 0;
                        return true;
                    }
                } else if self.current_node == number_of_nodes {
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
