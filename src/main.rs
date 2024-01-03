use crossterm::{event::KeyEventKind, terminal::{enable_raw_mode, disable_raw_mode}, style::Print};
pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
    Command,
};

use std::{io::{self,stdout, Write}, fs::File };
mod linkedlist;
use linkedlist::LinkedList;


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

fn run(out: &mut io::Stdout, list: &mut LinkedList){
    loop {
        // if event::poll(std::time::Duration::from_secs(10)).unwrap() { // For some reason this does nothing to the code 
            if let Ok(key_event) = read_key() {
                match key_event.code {
                    KeyCode::Char(c) => {
                        queue!(out, Print(c));
                        out.flush().unwrap();
                        list.write(&[c as u8]);
                    },
                    KeyCode::Backspace => {
                        //remove char
                        print!("{} {}", 8 as char, 8 as char);
                        out.flush().unwrap();
                        
                        //remove char and move cursor to the end of previous line
                        if cursor::position().unwrap().0 == 0 {
                            execute!(out, cursor::MoveUp(1)).unwrap();
                            let (_, y) = cursor::position().unwrap();
                            execute!(out, cursor::MoveRight(y)).unwrap();
                        }
                        out.flush().unwrap();
                    },
                    KeyCode::Esc => {
                        end(out);
                        break;
                    },
                    KeyCode::Enter => {
                        println!();
                        list.insert('\n');

                    },
                    KeyCode::Up => execute!(out, cursor::MoveUp(1)).unwrap(),
                    KeyCode::Down => execute!(out, cursor::MoveDown(1)).unwrap(),
                    KeyCode::Left => execute!(out, cursor::MoveLeft(1)).unwrap(),
                    KeyCode::Right => execute!(out, cursor::MoveRight(1)).unwrap(),
                    _ => (),
                }
            }
        // }
    }
}

fn end(out: &mut io::Stdout){
    execute!(out, terminal::LeaveAlternateScreen); // Leave alternate screen
    terminal::disable_raw_mode();
}

fn main(){ 
    //TODO: what is the use of ?    
    let mut list = LinkedList::new();
    let mut out = stdout();
    let mut file = File::create("out.txt");

    
    setup(&mut out);
    run(&mut out,&mut list );
    list.write_to_file(&mut file.unwrap());
    println!("{:#?}", list);

}