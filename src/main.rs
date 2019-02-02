use std::io::{stdin, stdout, Write};
use std::sync::mpsc;
use std::thread;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod games;

fn main() {
    let mut game = games::select();

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
                _ => None,
            }
            .and_then(|m| tx.send(m).ok());
        }
    });

    loop {
        let act = rx.try_recv();

        if let Ok(games::Action::Quit) = act {
            println!("Good bye!");
            break;
        }

        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::CurrentLine
        )
        .unwrap();

        game.update(act.ok());

        stdout.flush().unwrap();
    }

    // Show the cursor again before we exit.
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
