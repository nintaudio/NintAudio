const INITIAL_SIZE: (u32, u32) = (200, 200);

use std::rc::Rc;
use std::cell::RefCell;

use ai_behavior::{Action, Sequence};
use image::png;
use image::{ImageBuffer, ImageDecoder};
use piston_window::{OpenGL, PistonWindow, PressEvent, ResizeEvent, Size, Texture, TextureSettings, WindowSettings, Window, Input};
use sprite::*;
use uuid::Uuid;

use super::super::{games, Assets};

pub struct WindowBackend {
    window: RefCell<PistonWindow>,
    scene: RefCell<Scene<Texture<gfx_device_gl::Resources>>>,
    id: Uuid,
}

pub fn init() -> WindowBackend {
    let mut window: PistonWindow = WindowSettings::new("nintaudio", INITIAL_SIZE)
        .exit_on_esc(true)
        .opengl(OpenGL::V3_2)
        .build()
        .unwrap();

    let mut scene = Scene::new();
    let logo = Assets::get("logo.png").unwrap();
    let codec = png::PNGDecoder::new(&*logo).unwrap();
    let (w, h) = codec.dimensions();
    let buf = codec.read_image().unwrap();
    let img = ImageBuffer::from_raw(w as u32, h as u32, buf).unwrap();

    let tex =
        Rc::new(Texture::from_image(&mut window.factory, &img, &TextureSettings::new()).unwrap());
    let mut sprite = Sprite::from_texture(tex.clone());
    let Size { width, height } = window.size();
    sprite.set_position(f64::from(width) / 2., -f64::from(h as u32));
    sprite.set_scale(0.5, 0.5);

    let id = scene.add_child(sprite);

    // Run a sequence of animations.
    let seq = Sequence(vec![
        Action(Ease(
            EaseFunction::BounceOut,
            Box::new(MoveTo(0.1, f64::from(width) / 2., f64::from(height) / 2.)),
        )),
        Action(Ease(
            EaseFunction::CubicOut,
            Box::new(ScaleTo(0.1, 1., 1.)),
        )),
    ]);
    scene.run(id, &seq);

    // This animation and the one above can run in parallel.
    let rotate = Action(Ease(
        EaseFunction::ExponentialInOut,
        Box::new(RotateTo(0.2, 360.0)),
    ));
    scene.run(id, &rotate);

    WindowBackend {
        window: RefCell::new(window),
        scene: RefCell::new(scene),
        id,
    }
}

impl super::Vessel for WindowBackend {
    fn refresh(&mut self) -> Option<games::Action> {
        let mut scene = self.scene.borrow_mut();
        let mut window = self.window.borrow_mut();

        match window.next() {
            None => Some(games::Action::Quit),
            Some(e) => {
                scene.event(&e);

                window.draw_2d(&e, |c, g| {
                    piston_window::clear([10.0, 10.0, 1.0, 1.0], g);
                    scene.draw(c.transform, g);
                });

                if let Some(key) = e.press_args() {
                    println!("{:?}", key);
                }

                if let Some([w, h]) = e.resize_args() {
                    // This animation and the one above can run in parallel.
                    let recenter = Action(Ease(
                        EaseFunction::ExponentialInOut,
                        Box::new(MoveTo(1., f64::from(w) / 2., f64::from(h) / 2.)),
                    ));
                    scene.run(self.id, &recenter);
                }

                let i: Option<Input> = e.into();

                if let Some(event) = i {
                    println!("{:?}", event);
                }
                Some(games::Action::Left)
            }
        }
    }

    // Show the cursor again before we exit.
    fn clear (&self) {
    }
}

