use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use rust_embed::RustEmbed;
use gilrs::{Axis, Button, EventType, Gilrs};

mod games;
mod vessel;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

fn main() {
    let device = rodio::default_output_device().unwrap();
    let mut game = games::select(&device);

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut gilrs = Gilrs::new().unwrap();
        let mut direction = 0.;

        loop {
            vessel::piston::refresh();
            let act = rx.try_recv().ok()
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
                vessel::piston::clear();
                println!("Good bye!");
                std::process::exit(0);
            }

            if let Some(score) = game.update(act, &device) {
                vessel::piston::clear();
                println!("You made {} point(s)", score);
                std::process::exit(0);
            }

            thread::sleep(Duration::from_millis(20));
        }
    });

    vessel::piston::init(&tx);
}
