use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{stdout, stdin, Write};
use std::sync::mpsc;
use std::thread;
use super::super::games;

pub fn init () -> std::sync::mpsc::Receiver<games::Action> {
    let keys = stdin().keys(); // stdin keys
    let mut stdout = stdout().into_raw_mode().unwrap(); // stdout to raw mode.

    write!(
        stdout,
        "{}{}q to exit{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for c in keys {
            match c.unwrap() {
                Key::Char('q') | Key::Ctrl('c') => Some(games::Action::Quit),
                Key::Left | Key::Char('a') | Key::Char('4') => Some(games::Action::Left),
                Key::Right | Key::Char('d') | Key::Char('6') => Some(games::Action::Right),
                Key::Char('\n') | Key::Char(' ') | Key::Char('5') => Some(games::Action::Fire),
                Key::Up | Key::Char('w') | Key::Char('8') => Some(games::Action::Up),
                _ => None,
            }
            .and_then(|m| tx.send(m).ok());
        }
    });
    rx
}

pub fn clear () {
    write!(
                stdout(),
                "{}{}",
                termion::cursor::Goto(1, 1),
                termion::clear::CurrentLine
            )
            .unwrap();
}

// Show the cursor again before we exit.
pub fn cursor () {
    write!(stdout(), "{}", termion::cursor::Show).unwrap();
}
