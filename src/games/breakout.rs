use super::{Action, Assets, Game};

// whatever you want
pub struct Breakout {
    left_count: u8,
    right_count: u8,
    position: u8,
    sink: rodio::SpatialSink,
    bricks: [[bool; 2]; 5],
    ball_x: u8,
    ball_y: u8,
    hit_r_wall: bool,
    hit_top: bool,
}

impl Game for Breakout {
    fn update(&mut self, act: Option<Action>, _device: &rodio::Device) -> bool {
        match act {
            Some(Action::Left) => {
                self.left_count += 1;
                self.position -= 1;
            }
            Some(Action::Right) => {
                self.right_count += 1;
                self.position += 1;
            }
            _ => {}
        };

        if self.ball_x == 5 {
            self.hit_r_wall = true;
        } else if self.ball_x == 0 {
            self.hit_r_wall = false;
        }

        if self.ball_y == 7 {
            self.hit_top = true;
        } else if self.ball_y == 0 && self.position == self.ball_x {
            self.hit_top = false;
        }

        if self.hit_r_wall == false {
            self.ball_x += 1;
        } else {
            self.ball_x -= 1;
        }
        if self.hit_top == false {
            self.ball_y += 1;
        } else {
            self.ball_y -= 1;
        }

        self.ball_x += 1;
        self.ball_y += 1;

        println!("{:?} l: {} r: {}", act, self.left_count, self.right_count);
        self.sink
            .set_emitter_position([self.position as f32 / 10., 0., 0.]);
        false
    }
}

// Create a new game
pub fn new(device: &rodio::Device) -> Breakout {
    let sink = rodio::SpatialSink::new(
        device,
        [0., 0., 0.],  // object
        [1., 0., 0.],  // left ear
        [-1., 0., 0.], // right ear
    );
    let source = audio!("music.ogg");
    sink.append(source.repeat_infinite());

    Breakout {
        left_count: 0,
        right_count: 0,
        position: 2,
        sink,
        bricks: [[true; 2]; 5],
        ball_x: 2,
        ball_y: 0,
        hit_top: false,
        hit_r_wall: false,
    }
}

// One-line description
pub fn about() -> &'static str {
    "A"
}

// Complete description
pub fn description() -> &'static str {
    "A long"
}
