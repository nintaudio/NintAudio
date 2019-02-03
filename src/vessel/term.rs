use std::io::stdin;
use termion::event::Key;
use termion::input::TermRead;

use super::super::games;

pub fn init(tx: &std::sync::mpsc::Sender<games::Action>) {
    let keys = stdin().keys(); // stdin keys

    for c in keys {
        let act = match c.unwrap() {
            Key::Char('q') | Key::Ctrl('c') => Some(games::Action::Quit),
            Key::Left | Key::Char('a') | Key::Char('4') => Some(games::Action::Left),
            Key::Right | Key::Char('d') | Key::Char('6') => Some(games::Action::Right),
            Key::Char('\n') | Key::Char(' ') | Key::Char('5') => Some(games::Action::Fire),
            Key::Up | Key::Char('w') | Key::Char('8') => Some(games::Action::Up),
            _ => None,
        };

        if let Some(m) = act {
            if tx.send(m).is_err() {
                return;
            }
        };
    }
}
