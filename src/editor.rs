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
        let ch_array = ['\0'; SIZE];
        loop {
            // Making a node on heap making the linked list point to it and then adding characters to the node.
            //hope this is idiomatic enough
            let node = Some(Rc::new(RefCell::new(Node::new(ch_array))));
            self.rows.get_mut(self.cursor.getY()).unwrap().new_node(node.clone());

            let mut i = 0;

            while i < SIZE {

                if let Ok(key_event) = TextEditor::read_key() {
                    match key_event.code {
                        KeyCode::Char(c)
                        if key_event.modifiers == event::KeyModifiers::CONTROL && c == 's' =>{
                            println!("File saved.");
                        }

                        KeyCode::Char(c) => {
                            // if character read print on screen and place it into the array
                            if let Some(ref node) = node {
                                let mut node_ref = RefCell::borrow_mut(&node);
                                self.Read_Input(c);
                                node_ref.add_char(c);
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
                            if let Some(ref node) = node {
                                let mut node_ref = RefCell::borrow_mut(&node);
                                node_ref.add_char('\n');
                            }
                            
                            self.rows.push(LinkedList::new());
                            println!();
                            self.cursor.update();
                            break;
                        }
                        KeyCode::Home => println!("Cursor position: {:?}\r", position()),
                        KeyCode::Up => 
                        {
                            self.cursor.update();
                            if 0 < self.cursor.getY() {
                                let current_row_len = self.rows.get_mut(self.cursor.getY()).unwrap().get_row_len();
                                let upper_row_len = self.rows.get_mut(self.cursor.getY() - 1).unwrap().get_row_len();
                                
                                if current_row_len <= upper_row_len || self.cursor.getX() <= upper_row_len {
                                    execute!(self.stdout, cursor::MoveUp(1)).unwrap();
                                } else {
                                    execute!(self.stdout, cursor::MoveTo(upper_row_len as u16, (self.cursor.getY() - 1) as u16)).unwrap();
                                }
                            }
                        }

                        KeyCode::Down => 
                        {
                            self.cursor.update();
                            let current_row_len = self.rows.get_mut(self.cursor.getY()).unwrap().get_row_len();
                            let lower_row_len = self.rows.get_mut(self.cursor.getY() + 1).unwrap().get_row_len();

                            if current_row_len <= lower_row_len || self.cursor.getX() <= lower_row_len  {
                                execute!(self.stdout, cursor::MoveDown(1)).unwrap();
                            } else  {
                                execute!(self.stdout, cursor::MoveTo(lower_row_len as u16, (self.cursor.getY() + 1) as u16)).unwrap();
                            }
                        }

                        KeyCode::Left => {
                            let row = self.rows.get_mut(self.cursor.getY()).unwrap();
                            let moveleft = self.cursor.move_cursor(Direction::Left, row.get_number_of_nodes(), row.get_last());
                            if moveleft {
                                execute!(self.stdout, cursor::MoveLeft(1)).unwrap();
                            }
                        }

                        KeyCode::Right => {
                            let row = self.rows.get_mut(self.cursor.getY()).unwrap();
                            let moveright = self.cursor.move_cursor(Direction::Right, row.get_number_of_nodes(), row.get_last());
                            if moveright {
                                execute!(self.stdout, cursor::MoveRight(1)).unwrap();
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
