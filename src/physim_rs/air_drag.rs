use crate::physim_rs::object::Object;

pub struct AirDrag {
    pub fluid_density: f64,
    pub crossection_area: f64,
    pub drag_coefficient: f64,
}

impl AirDrag {
    pub fn new(fluid_density: f64, crossection_area: f64, drag_coefficient: f64) -> AirDrag {
        AirDrag {
            fluid_density: fluid_density,
            crossection_area: crossection_area,
            drag_coefficient: drag_coefficient,
        }
    }

    pub fn apply_drag_force(&mut self, obj: &mut Object) -> &mut Self {
        let drag_force = 0.5 * self.fluid_density * obj.vel.mag() * obj.vel.mag() * self.drag_coefficient * self.crossection_area; // Drag equation

        // Creating a vector pointing opposite way than velocity;
        let mut vel = obj.vel.copy();
        vel.norm().mult_val(-drag_force);

        // Applying drag force
        obj.apply_force(vel);

        self
    }
}