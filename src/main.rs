#![cfg_attr(test, deny(bad_style,
       const_err,
       dead_code,
       improper_ctypes,
       legacy_directory_ownership,
       non_shorthand_field_patterns,
       no_mangle_generic_items,
       overflowing_literals,
       path_statements ,
       patterns_in_fns_without_body,
       plugin_as_library,
       private_in_public,
       safe_extern_statics,
       unconditional_recursion,
       unions_with_drop_fields,
       unused,
       unused_allocation,
       unused_comparisons,
       unused_parens,
       while_true))]

// Allowed by default
#![cfg_attr(test, deny(missing_docs,
       trivial_numeric_casts,
       unused_extern_crates,
       unused_import_braces,
       unused_qualifications))]

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

    games::once(&device, "start_sound.ogg", 0., 0.);

    thread::spawn(move || {
        let mut gilrs = Gilrs::new().unwrap();
        let mut direction = 0.;

        loop {
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
                games::once(&device, "end_sound.ogg", 0., 0.);
                return;
            }

            if let Some(score) = game.update(act, &device) {
                println!("You made {} point(s)", score);
                games::once(&device, "end_sound.ogg", 0., 0.);
                return;
            }

            thread::sleep(Duration::from_millis(20));
        }
    });

    if cfg!(feature = "piston") {
        vessel::piston::init(&tx);
    } else {
        vessel::term::init(&tx);
    }

    // Show the cursor again before we exit.
    print!("{}", termion::cursor::Show);
}
