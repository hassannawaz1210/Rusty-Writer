pub use crossterm::{
    cursor,
    cursor::position,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
    Command,
};
use crossterm::{
    event::KeyEventKind,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use std::{
    cell::Ref,
    fs::{DirBuilder, File},
    io::{self, stdout, Cursor, Write},
    rc::Rc,
};
mod editor;
use editor::TextEditor;

mod cursedsor;
use cursedsor::Cursedsor;
use cursedsor::Direction;

mod linkedlist;
use linkedlist::LinkedList;
use linkedlist::SIZE;



// fn refreshWindow(out: &mut io::Stdout, list: &mut LinkedList) {
//     //save mouse position
//     let (x, y) = cursor::position().unwrap();
//     //clear the screen
//     execute!(out, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0));

//     //print the linked list
//     let mut current = list.root.clone();
//     while let Some(node) = current {
//         let node = node.borrow();
//         let data = node.data.clone();
//         for i in 0..SIZE {
//             print!("{}", data[i]);
//             out.flush().unwrap();
//         }
//         current = node.next.clone();
//     }
//     //restore mouse position
//     execute!(out, cursor::MoveTo(x, y));
// }




fn main() {
    let mut tx = TextEditor::new();

    TextEditor::setup(&mut tx);
    TextEditor::run(&mut tx);

    println!("{:#?}", tx.rows);
}
