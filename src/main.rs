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
    fs::File,
    io::{self, stdout, Write, Cursor},
    rc::Rc,
};

mod cursedsor;
use cursedsor::Cursedsor;

mod linkedlist;
use linkedlist::LinkedList;
use linkedlist::SIZE;

pub fn Read_Input(out: &mut io::Stdout, c: char, char_array: &mut [char; SIZE], index: usize) {
    queue!(out, Print(c));
    out.flush().unwrap();
    char_array[index] = c;
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

fn setup(out: &mut io::Stdout) -> crossterm::Result<()> {
    execute!(out, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?; // read somewhere does not echo the user input in raw mode
    execute!(
        out,
        style::ResetColor,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
        cursor::SetCursorStyle::BlinkingUnderScore
    ) // Macro within crossterm for setting up the terminal
}

fn refreshWindow(out: &mut io::Stdout, list: &mut LinkedList) {
    //save mouse position
    let (x, y) = cursor::position().unwrap();
    //clear the screen
    execute!(out, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0));

    //print the linked list
    let mut current = list.root.clone();
    while let Some(node) = current {
        let node = node.borrow();
        let data = node.data.clone();
        for i in 0..SIZE {
            print!("{}", data[i]);
            out.flush().unwrap();
        }
        current = node.next.clone();
    }
    //restore mouse position
    execute!(out, cursor::MoveTo(x, y));
}

fn run(out: &mut io::Stdout, list: &mut LinkedList) {
    loop {
        let mut ch_array = ['\0'; SIZE];
        let mut i = 0;
        while i < SIZE {
            if let Ok(key_event) = read_key() {
                match key_event.code {
                    KeyCode::Char(c) if key_event.modifiers == event::KeyModifiers::CONTROL && c == 's' => {
                        println!("File saved.");
                    },
                    KeyCode::Char(c) => {
                        // if character read print on screen and place it into the array
                        Read_Input(out, c, &mut ch_array, i);
                        //refreshWindow(out, list);
                        i+=1
                    }
                    KeyCode::Backspace => {
                        //remove char
                        print!("{} {}", 8 as char, 8 as char);
                        out.flush().unwrap();

                        //remove char and move cursor to the end of previous line
                        if cursor::position().unwrap().0 == 0 {//.0 is x coordinate
                            execute!(out, cursor::MoveUp(1)).unwrap();
                            let (_, y) = cursor::position().unwrap();
                            execute!(out, cursor::MoveRight(y)).unwrap();
                        }
                        out.flush().unwrap();
                    }
                    KeyCode::Esc => {
                        end(out);
                        // inserting the array into the linked list 
                        let mut char_arr_ref: Rc<[char; SIZE]> = Rc::new(ch_array);
                        list.insert(char_arr_ref);
                        return;
                    }
                    KeyCode::Enter => {
                        Read_Input(out, '\n', &mut ch_array, i);
                        i+=1;
                        //println!();
                    }
                    KeyCode::Home => println!("Cursor position: {:?}\r", position()),
                    KeyCode::Up => execute!(out, cursor::MoveUp(1)).unwrap(),           
                    KeyCode::Down => execute!(out, cursor::MoveDown(1)).unwrap(),                  
                    KeyCode::Left => execute!(out, cursor::MoveLeft(1)).unwrap(),
                    KeyCode::Right => execute!(out, cursor::MoveRight(1)).unwrap(),
                    _ => (),
                }
            }
        }
        // inserting filled array into the linked list 
        let mut char_arr_ref: Rc<[char; SIZE]> = Rc::new(ch_array);
        list.insert(char_arr_ref);
    }
}

fn end(out: &mut io::Stdout) {
    execute!(out, terminal::LeaveAlternateScreen); // Leave alternate screen
    terminal::disable_raw_mode();
}

fn main() {
    let mut list = LinkedList::new();
    let mut out = stdout();
    let mut file: Result<File, io::Error> = File::create("out.txt");
    let cursor =  Cursedsor::new(list.root.clone());

    setup(&mut out);
    run(&mut out, &mut list);
    list.write_to_file(&mut file.unwrap());
    println!("{:#?}", list);
}
