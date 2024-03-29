use std::{io::{stdout, Stdout, Write}, borrow::{Borrow, BorrowMut}, cell::RefCell, rc::Rc, sync::Arc};

use crossterm::{execute, terminal, event::{KeyEvent, KeyEventKind, self, Event, KeyCode}, cursor::{self, position}, style::{self, Print}, QueueableCommand, queue};

use crate::{linkedlist::{LinkedList, SIZE, Node}, cursedsor::Direction};
use crate::cursedsor::Cursedsor;
// This struct is used to get the terminal cursor position is also used for changing the indexes of vector


pub struct TextEditor {
    pub rows: Vec<LinkedList>,
    cursor: Cursedsor,
    stdout: Stdout,
}
impl TextEditor {
    pub fn new() -> Self {
        let mut tx = TextEditor {
            rows: Vec::new(),
            cursor : Cursedsor::new(),
            stdout: stdout(),
        };
        tx.rows.push(LinkedList::new()); // inserting the first row 
        return tx;
    }
    
    pub fn end(&mut self) {
        execute!(self.stdout, terminal::LeaveAlternateScreen); // Leave alternate screen
        terminal::disable_raw_mode();
    }

    pub fn read_key() -> crossterm::Result<KeyEvent> {
        loop {
            if let Ok(Event::Key(key_event)) = event::read() {
                if key_event.kind == KeyEventKind::Press {
                    return Ok(key_event);
                }
            }
        }
    }
    
    pub fn Read_Input(&mut self , c: char) {
        queue!(self.stdout, Print(c));
        self.stdout.flush().unwrap();
    }

    
    pub fn setup(&mut self) {
        execute!(self.stdout, terminal::EnterAlternateScreen);
        execute!(
            self.stdout,
            style::ResetColor,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0),
            cursor::SetCursorStyle::BlinkingUnderScore
        ); // Macro within crossterm for setting up the terminal
        let _ = terminal::enable_raw_mode(); // read somewhere does not echo the user input in raw mode
    }


    pub fn run(&mut self) {    
        let mut node_numebr = 0;
        let ch_array = ['\0'; SIZE];
        loop {
            // Making a node on heap making the linked list point to it and then adding characters to the node.
            //hope this is idiomatic enough
            node_numebr += 1;
            let node = Some(Rc::new(RefCell::new(Node::new(ch_array, node_numebr))));
            self.rows.get_mut(self.cursor.getY()).unwrap().new_node(node.clone());

            let mut i = 0;

            while i < SIZE {

                if let Ok(key_event) = TextEditor::read_key() {

                    let current_node = self.cursor.getX()/SIZE;
                    let last_node = self.rows.get_mut(self.cursor.getY()).unwrap().get_number_of_nodes();

                    match key_event.code {
                        KeyCode::Char(c)
                            if key_event.modifiers == event::KeyModifiers::CONTROL && c == 's' => {
                                // save file
                                let mut file = std::fs::File::create("test.txt").unwrap();
                                for row in self.rows.iter() {
                                    row.write_to_file_by_num(file.borrow_mut());
                                    file.write_all(b"\n").unwrap();
                                };
                                print!("File saved.");
                        }

                        KeyCode::Char(c) => {
                            // if character read print on screen and place it into the array
                            if let Some(ref node) = node {
                                let mut node_ref = RefCell::borrow_mut(&node);
                                self.Read_Input(c);
                                node_ref.add_char(c);

                                //if the cursor is present at the last node
                                //TODO: also add the check that cursor is at the last INDEX
                               
                                if current_node == last_node {
                                    node_ref.add_char(c);
                                }

                                //if cursor is not at the last node: 2 Cases
                                //1. cursor is at the end of any node
                                //2. cursor is in the middle of any node
                                i += 1;
                            }
                        }

                        KeyCode::Backspace => {
                            //remove char
                            print!("{} {}", 8 as char, 8 as char);
                            self.stdout.flush().unwrap();
    
                            //remove char and move cursor to the end of previous line
                            if cursor::position().unwrap().0 == 0 {
                                //.0 is x coordinate
                                execute!(self.stdout, cursor::MoveUp(1)).unwrap();
                                let (_, y) = cursor::position().unwrap();
                                execute!(self.stdout, cursor::MoveRight(y)).unwrap();
                            }
                            self.stdout.flush().unwrap();
                        }

                        KeyCode::Esc => {
                            self.end();
                            return;
                        }

                        KeyCode::Enter => {
                            // if let Some(ref node) = node {
                            //     let mut node_ref = RefCell::borrow_mut(&node);
                            // }
                            
                            self.rows.push(LinkedList::new());
                            println!();
                            self.cursor.update();
                            break;
                        }
                        KeyCode::Home => println!("Cursor position: {:?}\r", position()),
                        KeyCode::Up =>  {
                           
                            if self.cursor.getY() > 0   {
                                let current_row_len = self.rows.get_mut(self.cursor.getY()).unwrap().get_row_len();
                                let upper_row_len = self.rows.get_mut(self.cursor.getY() - 1).unwrap().get_row_len();
                                
                                if current_row_len <= upper_row_len || self.cursor.getX() <= upper_row_len {
                                    execute!(self.stdout, cursor::MoveUp(1)).unwrap();
                                } else {
                                    execute!(self.stdout, cursor::MoveTo(upper_row_len as u16, (self.cursor.getY() - 1) as u16)).unwrap();
                                }
                            }
                        }

                        KeyCode::Down => {
                            
                            if self.rows.len() - 1 > self.cursor.getY() {
                                let current_row_len = self.rows.get_mut(self.cursor.getY()).unwrap().get_row_len();
                                let lower_row_len = self.rows.get_mut(self.cursor.getY() + 1).unwrap().get_row_len();

                                if current_row_len <= lower_row_len || self.cursor.getX() <= lower_row_len  {
                                    execute!(self.stdout, cursor::MoveDown(1)).unwrap();
                                } else  {
                                    execute!(self.stdout, cursor::MoveTo(lower_row_len as u16, (self.cursor.getY() + 1) as u16)).unwrap();
                                }
                            }
                            
                        }

                        KeyCode::Left => {
                            // Hassan u were right there is an easier way XD
                            if self.cursor.getX() > 0 {
                                execute!(self.stdout, cursor::MoveLeft(1)).unwrap();
                            } else if self.cursor.getY() > 0 { // if the cursor is at the start of the line move upwards and to the end of the line
                                let upper_row_len = self.rows.get_mut(self.cursor.getY() - 1).unwrap().get_row_len();
                                execute!(self.stdout, cursor::MoveTo(upper_row_len as u16, (self.cursor.getY() - 1) as u16)).unwrap();
                            }

                             //updating the node numbers
                                if let Some(ref node) = node {
                                let mut node_ref = RefCell::borrow_mut(&node);

                                if current_node != last_node{
                                    let other_node = node_ref.num;
                                    node_ref.num = current_node;

                                    let row = self.rows.get_mut(self.cursor.getY()).unwrap();
                                    row.update_node_numbers(current_node, other_node); //replace current with other

                                }
                            }
                                
                            }

                        KeyCode::Right => {
                            let current_row_len = self.rows.get_mut(self.cursor.getY()).unwrap().get_row_len();

                            if current_row_len > self.cursor.getX() {
                                execute!(self.stdout, cursor::MoveRight(1)).unwrap();
                            }


                             //updating the node numbers
                                if let Some(ref node) = node {
                                let mut node_ref = RefCell::borrow_mut(&node);

                                if current_node != last_node{
                                    let other_node = node_ref.num;
                                    node_ref.num = current_node;

                                    let row = self.rows.get_mut(self.cursor.getY()).unwrap();
                                    row.update_node_numbers(current_node, other_node); //replace current with other

                                }
                            }

                        }
                        _ => (),
                    }
                }
                self.cursor.update(); 
            }

        }
    }
    

}
