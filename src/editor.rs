use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Ref, RefCell},
    io::{stdout, Stdout, Write},
    ops::IndexMut,
    rc::Rc,
    sync::Arc,
};

use crossterm::{
    cursor::{self, position},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute, queue,
    style::{self, Print},
    terminal, QueueableCommand,
};

use crate::cursedsor::Cursedsor;
use crate::{
    cursedsor::Direction,
    linkedlist::{LinkedList, Node, SIZE},
};
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
            cursor: Cursedsor::new(),
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

    pub fn Read_Input(&mut self, c: char) {
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

    pub fn middle_insertion(&mut self) -> Option<Rc<RefCell<Node>>> {
        let ch_array = ['\0'; SIZE];
        let node = Some(Rc::new(RefCell::new(Node::new(ch_array))));
        self.cursor.update();
        self.rows
            .get_mut(self.cursor.getY())
            .unwrap()
            .insert_after(node.clone(), self.cursor.get_node() - 2);
        return node.clone();
    }

    pub fn last_insertion(&mut self) -> Option<Rc<RefCell<Node>>> {
        let ch_array = ['\0'; SIZE];
        let node = Some(Rc::new(RefCell::new(Node::new(ch_array))));
        self.cursor.update();
        self.rows
            .get_mut(self.cursor.getY())
            .unwrap()
            .new_node(node.clone());
        return node.clone();
    }

    pub fn write_row(){
        
    }

    pub fn run(&mut self) {
        loop {
            // Making a node on heap making the linked list point to it and then adding characters to the node.
            //hope this is idiomatic enough
            let mut current_row_len = self.rows.get_mut(self.cursor.getY()).unwrap().get_row_len();
            let mut node = None;
            if current_row_len == self.cursor.getX(){
                node = self.last_insertion();
            } else {
                node = self.middle_insertion();
            }

            let mut i = 0;

            while i < SIZE {
                if let Ok(key_event) = TextEditor::read_key() {
                    match key_event.code {
                        KeyCode::Char(c)
                            if key_event.modifiers == event::KeyModifiers::CONTROL && c == 's' =>
                        {
                            // save file
                            let mut file = std::fs::File::create("test.txt").unwrap();
                            for row in self.rows.iter() {
                                row.write_to_file(file.borrow_mut());
                                file.write_all(b"\n").unwrap();
                            }
                            println!("File saved.");
                        }

                        KeyCode::Char(c) => {
                            // if character read print on screen and place it into the array
                            let mut current_row_len = self.rows.get_mut(self.cursor.getY()).unwrap().get_row_len();
                            if current_row_len > self.cursor.getX(){
                                node = self.middle_insertion();
                                i = 0;
                                if let Some(ref node) = node {
                                    let mut node_ref = RefCell::borrow_mut(&node);
                                    self.Read_Input(c);
                                    node_ref.add_char(c);
                                    i += 1;
                                }
                            }
                            else{
                                if let Some(ref node) = node {
                                    let mut node_ref = RefCell::borrow_mut(&node);
                                    self.Read_Input(c);
                                    node_ref.add_char(c);
                                    i += 1;
                                }
                            }

                            // write_row();
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
                        KeyCode::Up => {
                            if self.cursor.getY() > 0 {
                                let current_row_len =
                                    self.rows.get_mut(self.cursor.getY()).unwrap().get_row_len();
                                let upper_row_len = self
                                    .rows
                                    .get_mut(self.cursor.getY() - 1)
                                    .unwrap()
                                    .get_row_len();

                                if current_row_len <= upper_row_len
                                    || self.cursor.getX() <= upper_row_len
                                {
                                    execute!(self.stdout, cursor::MoveUp(1)).unwrap();
                                } else {
                                    execute!(
                                        self.stdout,
                                        cursor::MoveTo(
                                            upper_row_len as u16,
                                            (self.cursor.getY() - 1) as u16
                                        )
                                    )
                                    .unwrap();
                                }
                            }
                        }

                        KeyCode::Down => {
                            if self.rows.len() - 1 > self.cursor.getY() {
                                let current_row_len =
                                    self.rows.get_mut(self.cursor.getY()).unwrap().get_row_len();
                                let lower_row_len = self
                                    .rows
                                    .get_mut(self.cursor.getY() + 1)
                                    .unwrap()
                                    .get_row_len();

                                if current_row_len <= lower_row_len
                                    || self.cursor.getX() <= lower_row_len
                                {
                                    execute!(self.stdout, cursor::MoveDown(1)).unwrap();
                                } else {
                                    execute!(
                                        self.stdout,
                                        cursor::MoveTo(
                                            lower_row_len as u16,
                                            (self.cursor.getY() + 1) as u16
                                        )
                                    )
                                    .unwrap();
                                }
                            }
                        }

                        KeyCode::Left => {
                            // Hassan u were right there is an easier way XD
                            if self.cursor.getX() > 0 {
                                execute!(self.stdout, cursor::MoveLeft(1)).unwrap();
                            } else if self.cursor.getY() > 0 {
                                // if the cursor is at the start of the line move upwards and to the end of the line
                                let upper_row_len = self
                                    .rows
                                    .get_mut(self.cursor.getY() - 1)
                                    .unwrap()
                                    .get_row_len();
                                execute!(
                                    self.stdout,
                                    cursor::MoveTo(
                                        upper_row_len as u16,
                                        (self.cursor.getY() - 1) as u16
                                    )
                                )
                                .unwrap();
                            }
                        }

                        KeyCode::Right => {
                            let current_row_len =
                                self.rows.get_mut(self.cursor.getY()).unwrap().get_row_len();

                            if current_row_len > self.cursor.getX() {
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

    pub fn write_to_row() {}

    pub fn key_bindings(
        &mut self,
        node: Option<Rc<RefCell<Node>>>,
        i: &mut usize,
        key_event: KeyEvent,
    ) -> usize {
        match key_event.code {
            KeyCode::Char(c) if key_event.modifiers == event::KeyModifiers::CONTROL && c == 's' => {
                // save file
                let mut file = std::fs::File::create("test.txt").unwrap();
                for row in self.rows.iter() {
                    row.write_to_file(file.borrow_mut());
                    file.write_all(b"\n").unwrap();
                }
                // println!("File saved.");
            }

            KeyCode::Char(c) => {
                // if character read print on screen and place it into the array
                if let Some(o) = node {
                    let mut node_ref = RefCell::borrow_mut(&o);
                    self.Read_Input(c);
                    node_ref.add_char(c);
                    *i += 1;
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

            KeyCode::Up => {
                if self.cursor.getY() > 0 {
                    let current_row_len =
                        self.rows.get_mut(self.cursor.getY()).unwrap().get_row_len();
                    let upper_row_len = self
                        .rows
                        .get_mut(self.cursor.getY() - 1)
                        .unwrap()
                        .get_row_len();

                    if current_row_len <= upper_row_len || self.cursor.getX() <= upper_row_len {
                        execute!(self.stdout, cursor::MoveUp(1)).unwrap();
                    } else {
                        execute!(
                            self.stdout,
                            cursor::MoveTo(upper_row_len as u16, (self.cursor.getY() - 1) as u16)
                        )
                        .unwrap();
                    }
                }
            }

            KeyCode::Down => {
                if self.rows.len() - 1 > self.cursor.getY() {
                    let current_row_len =
                        self.rows.get_mut(self.cursor.getY()).unwrap().get_row_len();
                    let lower_row_len = self
                        .rows
                        .get_mut(self.cursor.getY() + 1)
                        .unwrap()
                        .get_row_len();

                    if current_row_len <= lower_row_len || self.cursor.getX() <= lower_row_len {
                        execute!(self.stdout, cursor::MoveDown(1)).unwrap();
                    } else {
                        execute!(
                            self.stdout,
                            cursor::MoveTo(lower_row_len as u16, (self.cursor.getY() + 1) as u16)
                        )
                        .unwrap();
                    }
                }
            }

            KeyCode::Left => {
                // Hassan u were right there is an easier way XD
                if self.cursor.getX() > 0 {
                    execute!(self.stdout, cursor::MoveLeft(1)).unwrap();
                } else if self.cursor.getY() > 0 {
                    // if the cursor is at the start of the line move upwards and to the end of the line
                    let upper_row_len = self
                        .rows
                        .get_mut(self.cursor.getY() - 1)
                        .unwrap()
                        .get_row_len();
                    execute!(
                        self.stdout,
                        cursor::MoveTo(upper_row_len as u16, (self.cursor.getY() - 1) as u16)
                    )
                    .unwrap();
                }
            }

            KeyCode::Right => {
                let current_row_len = self.rows.get_mut(self.cursor.getY()).unwrap().get_row_len();

                if current_row_len > self.cursor.getX() {
                    execute!(self.stdout, cursor::MoveRight(1)).unwrap();
                }
            }
            _ => (),
        }
        0
    }
}

// pub fn run(&mut self) {
//     loop {
//         // Making a node on heap making the linked list point to it and then adding characters to the node.
//         //hope this is idiomatic enough
//         let mut current_row_len = self.rows.get_mut(self.cursor.getY()).unwrap().get_row_len();
//         let mut node = None;
//         if current_row_len == self.cursor.getX(){
//             node = self.last_insertion();
//         } else {
//             node = self.middle_insertion();
//         }

//         let mut i: usize = 0;
//         while i < SIZE {
//             current_row_len = self.rows.get_mut(self.cursor.getY()).unwrap().get_row_len();
//             if let Ok(key_event) = TextEditor::read_key() {
//                 if key_event.code == KeyCode::Esc {
//                     self.end();
//                     return;
//                 }
//                 else if key_event.code == KeyCode::Enter {
//                     self.rows.push(LinkedList::new());
//                     println!();
//                     self.cursor.update();
//                     break;
//                 }
//                 self.key_bindings(node.clone(), &mut i, key_event);
//             }
//             self.cursor.update();
//             if self.cursor.getX() < current_row_len{
//                 node = self.middle_insertion();
//             }
//             i = 0;
//         }
//         self.cursor.update();
//     }
// }
