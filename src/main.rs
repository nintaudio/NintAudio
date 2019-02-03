use std::io::{stdin, stdout, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use gilrs::{Gilrs, Button, EventType, Axis};

mod games;

fn main() {
    let device = rodio::default_output_device().unwrap();
    let mut gilrs = Gilrs::new().unwrap();
    let mut game = games::select(&device);

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

    let mut direction = 0.;

    loop {


        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::CurrentLine
        )
        .unwrap();

        let act = rx.try_recv().ok()
            .or_else(|| gilrs.next_event()
                .and_then(|e| {
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
            )
            .or_else(|| gilrs.gamepads()
                .next()
                .and_then(|(_id, gamepad)| {
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
            )
            .or_else(|| gilrs.gamepads()
                .next()
                .and_then(|(_id, gamepad)| {
                    if gamepad.is_pressed(Button::RightTrigger2) {
                        Some(games::Action::Right)
                    } else if gamepad.is_pressed(Button::LeftTrigger2) {
                        Some(games::Action::Left)
                    } else {
                        None
                    }
                })
            );

        if let Some(games::Action::Quit) = act {
            println!("Good bye!");
            break;
        }

        if let Some(score) = game.update(act, &device) {
            // Show the cursor again before we exit.
            write!(stdout, "{}", termion::cursor::Show).unwrap();
            println!("You made {} point(s)", score);
            break;
        }

        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(20));
    }
}
