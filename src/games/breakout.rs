use rodio::source::Source;

use super::{audio, once, Action, Game};

// whatever you want
pub struct Breakout {
    left_count: u8,
    right_count: u8,
    position: u8,
    sink: rodio::SpatialSink,
    bricks: [[bool; 3]; 6],
    ball_x: u8,
    ball_y: u8,
    hit_r_wall: bool,
    hit_top: bool,
    time: u32,
    points: u16,
}

impl Game for Breakout {
    fn update(&mut self, act: Option<Action>, device: &rodio::Device) -> Option<u32> {
        self.time += 1;

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

        if self.time % 50 == 0 {
            if self.ball_x == 5 {
                self.hit_r_wall = true;
                once(
                    device,
                    "hit_wall.ogg",
                    (f32::from(self.ball_x) - f32::from(self.position)) / 2.,
                    f32::from(self.ball_y) / 2.,
                );
            } else if self.ball_x == 0 {
                self.hit_r_wall = false;
                once(
                    device,
                    "hit_wall.ogg",
                    (f32::from(self.ball_x) - f32::from(self.position)) / 2.,
                    f32::from(self.ball_y) / 2.,
                );
            }
            if self.ball_y > 4 && self.ball_y < 7 {
                println!("{} {}", self.ball_y, self.ball_x);
                if self.bricks[usize::from(self.ball_x)][usize::from(self.ball_y - 5)] == true {
                    self.points += 1;
                    self.bricks[usize::from(self.ball_x)][usize::from(self.ball_y - 5)] = false;
                    self.hit_top = true;
                    once(
                        device,
                        "short_bit_medium.ogg",
                        (f32::from(self.ball_x) - f32::from(self.position)) / 2.,
                        f32::from(self.ball_y) / 2.,
                    );
                }
            }
            if self.ball_y == 6 {
                self.hit_top = true;
                once(
                    device,
                    "hit_wall.ogg",
                    (f32::from(self.ball_x) - f32::from(self.position)) / 2.,
                    f32::from(self.ball_y) / 2.,
                );

            } else if self.ball_y == 0 && self.position == self.ball_x {
                self.hit_top = false;
                once(
                    device,
                    "hit_paddle.ogg",
                    (f32::from(self.ball_x) - f32::from(self.position)) / 2.,
                    f32::from(self.ball_y) / 2.,
                );
            } else if self.ball_y == 0 {
                println!("You lost.");
                once(
                    device,
                    "Sounds_NintAudio/entity_passing_by.mp3",
                    (f32::from(self.ball_x) - f32::from(self.position)) / 2.,
                    f32::from(self.ball_y) / 2.,
                );
                return Some(self.points.into());
            }
            
            if self.time == 2800 {
                println!("You won!");
                return Some(self.points.into());
            }

            if !self.hit_r_wall {
                self.ball_x += 1;
            } else {
                self.ball_x -= 1;
            }
            if !self.hit_top {
                self.ball_y += 1;
            } else {
                self.ball_y -= 1;
            }


            self.sink.set_emitter_position([
                (f32::from(self.ball_x) - f32::from(self.position)) / 2.,
                ((f32::from(self.ball_y)) / 2.),
                0.,
            ]);
        }

        None
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
    let source = audio("object_movement_big.ogg");

    sink.append(source.repeat_infinite());

    Breakout {
        left_count: 0,
        right_count: 0,
        position: 2,
        sink,
        bricks: [[true; 3]; 6],
        ball_x: 2,
        ball_y: 0,
        hit_top: false,
        hit_r_wall: false,
        time: 0,
        points: 0,
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
