use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use super::super::games;

pub fn init(tx: &std::sync::mpsc::Sender<games::Action>) {
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

    for c in keys {
        match c.unwrap() {
            Key::Char('q') | Key::Ctrl('c') => Some(games::Action::Quit),
            Key::Left | Key::Char('a') | Key::Char('4') => Some(games::Action::Left),
            Key::Right | Key::Char('d') | Key::Char('6') => Some(games::Action::Right),
            Key::Char('\n') | Key::Char(' ') | Key::Char('5') => Some(games::Action::Fire),
            Key::Up | Key::Char('w') | Key::Char('8') => Some(games::Action::Up),
            _ => None,
        }
        .and_then(|m| Some(tx.send(m).unwrap()));
    }
}

pub fn refresh() {
    print!(
        "{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::CurrentLine
    );
    stdout().flush().unwrap();
}

// Show the cursor again before we exit.
pub fn clear() {
    print!("{}", termion::cursor::Show);
}
