mod physim_rs;

use physim_rs::prelude::*;

fn main() {
    let mut obj = Object::new(Vector::new(), Vector::new2(0.0, 100.0), Vector::new(), 1.0);

    for _ in 0..101 {
        obj
            .apply_force(Vector::new2(0.0, -10.0)) // Applying gravity
            .dump() // Printing object stats to terminal
            .update(0.2); // Updating object
    }
}
