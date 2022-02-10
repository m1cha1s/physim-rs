mod physim_rs;

use physim_rs::prelude::*;
use std::f64::consts::*;

fn main() {
    let mut obj = Object::new(Vector::new(), Vector::new2(0.0, 100.0), Vector::new(), 1.0);
    let mut drag = AirDrag::new(1.225, PI, 0.45);

    for _ in 0..1 {
        drag.apply_drag_force(&mut obj); // FIXME: this produces a NaN. Why?
        obj
            .apply_force(Vector::new2(0.0, -10.0)) // Applying gravity
            .dump() // Printing object stats to terminal
            .update(0.2); // Updating object
    }
}
