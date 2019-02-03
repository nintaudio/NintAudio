use gilrs::{Axis, Button, EventType, Gilrs};
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use super::super::games;

pub fn init(tx: std::sync::mpsc::Sender<games::Action>) {
    let mut gilrs = Gilrs::new().unwrap();
    let keys = stdin().keys(); // stdin keys
    let mut stdout = stdout().into_raw_mode().unwrap(); // stdout to raw mode.
    let mut direction = 0.;

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
        .or_else(|| {
            gilrs.next_event().and_then(|e| {
                if let EventType::ButtonPressed(Button::South, _) = e.event {
                    Some(games::Action::Fire)
                } else if let EventType::ButtonPressed(Button::LeftTrigger2, _) = e.event {
                    Some(games::Action::Left)
                } else if let EventType::ButtonPressed(Button::RightTrigger2, _) = e.event {
                    Some(games::Action::Right)
                } else {
                    None
                }
            })
        })
        .or_else(|| {
            gilrs.gamepads().next().and_then(|(_id, gamepad)| {
                direction += (gamepad.value(Axis::LeftStickX) * 10.).round() / 10.;
                if direction >= 1. {
                    direction -= 1.;
                    Some(games::Action::Right)
                } else if direction <= -1. {
                    direction += 1.;
                    Some(games::Action::Left)
                } else {
                    None
                }
            })
        })
        .and_then(|m| Some(tx.send(m).unwrap()));
    }
}

pub fn refresh() {
    write!(
        stdout(),
        "{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::CurrentLine
    )
    .unwrap();
    stdout().flush().unwrap();
}

// Show the cursor again before we exit.
pub fn clear() {
    write!(stdout(), "{}", termion::cursor::Show).unwrap();
}
