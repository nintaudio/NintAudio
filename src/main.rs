use std::io::{stdout, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use gilrs::{Axis, Button, EventType, Gilrs};
use rust_embed::RustEmbed;
use termion::raw::IntoRawMode;

mod games;
mod vessel;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

fn main() {
    let device = rodio::default_output_device().unwrap();
    let mut game = games::select(&device);

    let (tx, rx) = mpsc::channel();

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

    thread::spawn(move || {
        let mut gilrs = Gilrs::new().unwrap();
        let mut direction = 0.;

        loop {
            if cfg!(piston) {
                vessel::piston::refresh();
            } else {
                vessel::term::refresh();
            }

            write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(1, 1),
                termion::clear::CurrentLine
            )
            .unwrap();
            stdout.flush().unwrap();

            let act = rx
                .try_recv()
                .ok()
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
                });

            if let Some(games::Action::Quit) = act {
                println!("Good bye!");
                std::process::exit(0);
            }

            if let Some(score) = game.update(act, &device) {
                println!("You made {} point(s)", score);
                std::process::exit(0);
            }

            thread::sleep(Duration::from_millis(20));
        }
    });

    if cfg!(piston) {
        vessel::piston::init(&tx);
    } else {
        vessel::term::init(&tx);
    }

    // Show the cursor again before we exit.
    print!("{}", termion::cursor::Show);
}
