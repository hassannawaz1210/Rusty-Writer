use std::{io::{stdout, Stdout, Write}, borrow::{Borrow, BorrowMut}, cell::RefCell, rc::Rc, sync::Arc};

use crossterm::{execute, terminal, event::{KeyEvent, KeyEventKind, self, Event, KeyCode}, cursor::{self, position}, style::{self, Print}, QueueableCommand, queue};

use crate::{linkedlist::{LinkedList, SIZE, Node}, cursedsor::Direction};
use crate::cursedsor::Cursedsor;
// This struct is used to get the terminal cursor position is also used for changing the indexes of vector
pub struct TerminalCursor {
    x: usize,
    y: usize,
}

impl TerminalCursor {
    pub fn new() -> Self {
        TerminalCursor {
            x: 0,
            y: 0,
        }
    }
    pub fn update(&mut self) {
        let (x, y) = crossterm::cursor::position().unwrap();
        self.x = x as usize;
        self.y = y as usize;
    }
}

pub struct TextEditor {
    pub rows: Vec<LinkedList>,
    terminal_cursor: TerminalCursor,
    cursor: Cursedsor,
    stdout: Stdout,
}
impl TextEditor {
    pub fn new() -> Self {
        let mut tx = TextEditor {
            rows: Vec::new(),
            cursor : Cursedsor::new(),
            terminal_cursor: TerminalCursor::new(),
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
        let mut ch_array = ['\0'; SIZE];
        loop {
            // Making a node on heap making the linked list point to it and then adding characters to the node.
            //hope this is idiomatic enough
            let node = Some(Rc::new(RefCell::new(Node::new(ch_array))));
            self.rows.get_mut(self.terminal_cursor.y).unwrap().new_node(node.clone());

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
                                
                                // let mut row = self.rows.get_mut(self.terminal_cursor.y).unwrap();
                                // let moveright = self.cursor.move_cursor(Direction::Right, row.get_number_of_nodes(), row.get_last(), self.terminal_cursor.x);
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
                            self.rows.get_mut(self.terminal_cursor.y).unwrap().new_node(node.clone());
                            return;
                        }
                        KeyCode::Enter => {
                            let row_list = self.rows.get_mut(self.terminal_cursor.y).unwrap();
                            
                            if let Some(ref node) = node {
                                let mut node_ref = RefCell::borrow_mut(&node);
                                node_ref.add_char('\n');
                            }
                            
                            self.rows.push(LinkedList::new());
                            self.terminal_cursor.update();
                            println!();
                            break;
                        }
                        KeyCode::Home => println!("Cursor position: {:?}\r", position()),
                        KeyCode::Up => execute!(self.stdout, cursor::MoveUp(1)).unwrap(),
                        KeyCode::Down => execute!(self.stdout, cursor::MoveDown(1)).unwrap(),
                        KeyCode::Left => {
                            execute!(self.stdout, cursor::MoveLeft(1)).unwrap();
                            let mut row = self.rows.get_mut(self.terminal_cursor.y).unwrap();
                            let moveleft = self.cursor.move_cursor(Direction::Left, row.get_number_of_nodes(), row.get_last(), self.terminal_cursor.x);
                        }
                        KeyCode::Right => {
                            let mut row = self.rows.get_mut(self.terminal_cursor.y).unwrap();
                            let moveright = self.cursor.move_cursor(Direction::Right, row.get_number_of_nodes(), row.get_last(), self.terminal_cursor.x);
                            if moveright {
                                execute!(self.stdout, cursor::MoveRight(1)).unwrap();
                            }
                            // self.cursor.print();
                        }
                        _ => (),
                    }
                }
                self.terminal_cursor.update(); 
            }

        }
    }
    

}
