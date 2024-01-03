use crossterm::{cursor, event::{self, Event, KeyCode}, execute};
use std::io::{stdout, Write};

fn main() {

    let mut stdout = stdout();

    println!("Started.");
    //clear the screen
    print!("{esc}c", esc = 27 as char);

    loop {
        if event::poll(std::time::Duration::from_millis(1000000000000000000)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char(c) => {
                        stdout.flush().unwrap();
                        print!("{}", c);
                    },
                    KeyCode::Backspace => {
                        //remove char
                        print!("{} {}", 8 as char, 8 as char);
                        stdout.flush().unwrap();
                        
                        //remove char and move cursor to the end of previous line
                        if cursor::position().unwrap().0 == 0 {
                            execute!(stdout, cursor::MoveUp(1)).unwrap();
                            let (_, y) = cursor::position().unwrap();
                            execute!(stdout, cursor::MoveRight(y)).unwrap();
                        }
                        stdout.flush().unwrap();
                    },
                    KeyCode::Enter => println!(),
                    KeyCode::Up => execute!(stdout, cursor::MoveUp(1)).unwrap(),
                    KeyCode::Down => execute!(stdout, cursor::MoveDown(1)).unwrap(),
                    KeyCode::Left => execute!(stdout, cursor::MoveLeft(1)).unwrap(),
                    KeyCode::Right => execute!(stdout, cursor::MoveRight(1)).unwrap(),
                    _ => (),
                }
            }
        }
    }
}