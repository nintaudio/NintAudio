use std::time::Duration;
use std::thread;

use rust_embed::RustEmbed;
use gilrs::{Gilrs, Button, EventType, Axis};

use crate::vessel::Vessel;

mod games;
mod vessel;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

fn main() {
    let device = rodio::default_output_device().unwrap();
    let mut gilrs = Gilrs::new().unwrap();
    let mut game = games::select(&device);

    let mut backend = vessel::piston::init();

    let mut direction = 0.;

    loop {
        let act = backend.refresh()
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
            backend.clear();
            println!("Good bye!");
            break;
        }

        if let Some(score) = game.update(act, &device) {
            backend.clear();
            println!("You made {} point(s)", score);
            break;
        }

        thread::sleep(Duration::from_millis(20));
    }
}
