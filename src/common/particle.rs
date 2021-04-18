use super::geometry::{Vec2, Vector};

pub struct Particle {
    position: Vec2,
    velocity: Vec2,
}

impl Particle {
    pub fn new(position: Vec2, velocity: Vec2) -> Self {
        Self { position, velocity }
    }

    pub fn update(&mut self, dt: f64) -> Vec2 {
        self.position += dt * self.velocity;
        self.position
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn velocity(&self) -> Vec2 {
        self.velocity
    }
}
