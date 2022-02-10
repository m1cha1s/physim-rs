use crate::physim_rs::vector::*;

pub struct Object {
    pub pos: Vector,
    pub vel: Vector,
    pub acc: Vector,
    pub mass: f64,
}


impl Object {
    pub fn new(pos: Vector, vel: Vector, acc: Vector, mass: f64) -> Object {
        Object {
            pos: pos,
            vel: vel,
            acc: acc,
            mass: mass,
        }
    }

    pub fn update(&mut self, dt: f64) -> &mut Self {
        self.vel.add(self.acc.copy().mult_val(dt));
        self.pos.add(self.vel.copy().mult_val(dt));
        self.acc.mult_zero();
        self
    }

    pub fn dump(&mut self) -> &mut Self {
        println!("Position:");
        println!("X: {} \nY: {} \nZ: {}", self.pos.x, self.pos.y, self.pos.z);
        println!("Velocity:");
        println!("X: {} \nY: {} \nZ: {}", self.vel.x, self.vel.y, self.vel.z);
        println!("Acceleration:");
        println!("X: {} \nY: {} \nZ: {}", self.acc.x, self.acc.y, self.acc.z);
        self
    }
    
    pub fn apply_force(&mut self, mut force: Vector) -> &mut Self {
        self.acc.add(force.div_val(self.mass));
        self
    }
}