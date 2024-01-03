use crossterm::{event::KeyEventKind, terminal::{enable_raw_mode, disable_raw_mode}};
pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
    Command,
};

use std::io::{stdout, Write};


pub fn read_key() -> crossterm::Result<KeyEvent> {
    loop {
        if let Ok(Event::Key(key_event)) = event::read() {
            if key_event.kind == KeyEventKind::Press {
                return Ok(key_event);
            }
        }
    }
}

fn main()  -> crossterm::Result<()>{

    //TODO: what is the use of ? 

    let mut stdout = stdout();

    execute!(stdout, terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?; // read somewhere does not echo the user input in raw mode

    execute!(
        stdout,
        style::ResetColor,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
        cursor::SetCursorStyle::BlinkingUnderScore
    )?; // Macro within crossterm for setting up the terminal



    loop {
        // if event::poll(std::time::Duration::from_secs(10)).unwrap() { // For some reason this does nothing to the code 
            if let Ok(key_event) = read_key() {
                match key_event.code {
                    KeyCode::Char(c) => {
                        print!("{}", c);
                        stdout.flush().unwrap();
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
                    KeyCode::Esc => {
                        break;
                    },
                    KeyCode::Enter => println!(),
                    KeyCode::Up => execute!(stdout, cursor::MoveUp(1)).unwrap(),
                    KeyCode::Down => execute!(stdout, cursor::MoveDown(1)).unwrap(),
                    KeyCode::Left => execute!(stdout, cursor::MoveLeft(1)).unwrap(),
                    KeyCode::Right => execute!(stdout, cursor::MoveRight(1)).unwrap(),
                    _ => (),
                }
            }
        // }
    }
    execute!(stdout, terminal::LeaveAlternateScreen)?; // Leave alternate screen
    terminal::disable_raw_mode()?;
    Ok(())
}