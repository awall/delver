use crate::types::*;

const NORMAL: bool = true;

pub struct Direction {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

pub struct Player {
    pub rotation: Angle,
    pub position: WorldPosition,
    pub attacking: bool,
    start_attack: bool,
    attack_timer: Option<f64>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            rotation: 0.,
            position: [0., 0.],
            attacking: false,
            start_attack: false,
            attack_timer: None,
        }
    }

    pub fn rotate_from_pos(&mut self, pos: WorldPosition) {
        let x = pos[0] - self.position[0];
        let y = pos[1] - self.position[1];
        if !NORMAL {
            self.rotation = y.atan2(x);
        }
    }

    pub fn attack(&mut self) {
        if self.attack_timer == None {
            self.start_attack = true;
        }
    }

    pub fn update(&mut self, dt: f64, dir: &Direction) {
        if self.start_attack {
            self.start_attack = false;
            self.attacking = true;
            self.attack_timer = Some(0.35);
        }

        if let Some(t) = self.attack_timer {
            let t = t - dt;
            if t < 0.0 {
                self.attacking = false;
                self.start_attack = false;
                self.attack_timer = None;
            } else {
                self.attack_timer = Some(t);
            }
        }        

        const VELOCITY: f64 = 250.0;
        let x = if NORMAL { 0.0 } else { self.rotation.cos() };
        let y = if NORMAL { 1.0 } else { -self.rotation.sin() };

        if dir.up {
            self.position[0] += x * VELOCITY * dt;
            self.position[1] += y * VELOCITY * dt;
            if NORMAL {
                self.rotation = 90f64.to_radians();
            }
        }
        if dir.down {
            self.position[0] -= x * VELOCITY * dt;
            self.position[1] -= y * VELOCITY * dt;
            if NORMAL {
                self.rotation = -90f64.to_radians();
            }            
        }
        if dir.left {
            self.position[0] += -y * VELOCITY * dt;
            self.position[1] += x * VELOCITY * dt;
            if NORMAL {
                self.rotation = 180f64.to_radians();
            }
        }
        if dir.right {
            self.position[0] -= -y * VELOCITY * dt;
            self.position[1] -= x * VELOCITY * dt;
            if NORMAL {
                self.rotation = 0.0;
            }
        }
    }
}