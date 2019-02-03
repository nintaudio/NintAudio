use rand::Rng;

use super::{once, Action, Game};

// whatever you want
pub struct Moles {
    left_count: u8,
    right_count: u8,
    position: i16,
    score: i16,
    spawn_time: u16,
    spawn_rate: u16,
    game_time: u16,
    moles: [bool; 3],
    unspawn: u8,
    spawn: u8,
}

// Create a new game
pub fn new(device: &rodio::Device) -> Moles {
    Moles {
        left_count: 0,
        right_count: 0,
        position: 0,
        score: 0,
        moles: [false; 3],
        unspawn: 3,
        spawn: 0,
        spawn_time: 3_00 / 2,
        spawn_rate: 3_00 / 2,
        game_time: 60_00 / 2,
    }
}

impl Game for Moles {
    fn update(&mut self, act: Option<Action>, device: &rodio::Device) -> Option<u32> {
        //Game Timer
        self.game_time -= 1;
        if self.game_time == 0 {
            return Some(self.score as u32);
        }

        //Spawn Timer
        self.spawn_time = if self.spawn_time == 20 {
            //Spawn New Mole
            self.spawn = spawn(self.unspawn, &mut self.moles, &device);

            //Reduce Timer
            self.spawn_rate -= if self.spawn_rate > 25 { 2 } else { 0 };
            19
        } else if self.spawn_time == 0 {
            //Unspawn Old Mole
            unspawn(self.unspawn, self.spawn, &mut self.moles, &device);
            self.unspawn = self.spawn;
            self.spawn_rate
        } else {
            //Decrease Time
            self.spawn_time - 1
        };

        match act {
            Some(Action::Left) => {
                self.score += action_check(self.moles[0], 0, &device);
            }
            Some(Action::Right) => {
                self.score += action_check(self.moles[2], 2, &device);
            }
            Some(Action::Up) => {
                self.score += action_check(self.moles[1], 1, &device);
            }
            _ => {}
        };

        if self.score < 0 {
            self.score = 0;
        }

        println!("{:?} l: {} r: {} Score: {} Time: {} SpawnTime: {} Moles: {}, {}, {} Unspawn: {} Spawn: {}",
             act,
             self.left_count,
             self.right_count,
             self.score,
             self.game_time/50,
             self.spawn_time,
             if self.moles[0]{"1"}else{"0"},
             if self.moles[1]{"1"}else{"0"},
             if self.moles[2]{"1"}else{"0"},
             self.unspawn,
             self.spawn);
        None
    }
}

fn unspawn(unspawn: u8, spawn: u8, moles: &mut [bool; 3], device: &rodio::Device) {
    if unspawn < 3 {
        moles[unspawn as usize] = false;
        once(device, "enemy_unspawn.ogg", x(unspawn), y(unspawn));
    }
}

fn spawn(unspawn: u8, moles: &mut [bool; 3], device: &rodio::Device) -> u8 {
    let spawn: u8;
    loop {
        let slot = rand::thread_rng().gen_range(0, 3) as usize;
        if !moles[slot] && (slot as u8) != unspawn {
            moles[slot] = true;
            spawn = slot as u8;
            once(device, "enemy_spawn.ogg", x(spawn), y(spawn));
            break;
        }
    }
    spawn
}

fn action_check(hit: bool, emitter: u8, device: &rodio::Device) -> i16 {
    if hit {
        once(device, "swing_hit.ogg", x(emitter), y(emitter));
        2
    } else {
        once(device, "swing_miss_hit.ogg", x(emitter), y(emitter));
        -1
    }
}

fn x(position: u8) -> f32 {
    if position == 0 {
        2.
    } else if position == 2 {
        -2.
    } else {
        0.
    }
}

fn y(position: u8) -> f32 {
    if position == 1 {
        2.
    } else {
        0.
    }
}

// One-line description
pub fn about() -> &'static str {
    "Hit as many moles as possible!"
}

// Complete description
pub fn description() -> &'static str {
    "Listen carefully and hit the moles when they pop out! Gain points by hitting them before they hide again, lose points by hitting an empty spot."
}
