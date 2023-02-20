use super::{
    engine::{PARTICLE_MASS, TIMESTEP},
    force::Force,
};

#[derive(Clone)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub velocity: Force<f64>,
}

impl Particle {
    pub fn get_render_position(&self, max_x: u32, max_y: u32) -> (u32, u32) {
        let render_x = (self.x.round() as u32).clamp(0, max_x);
        let render_y = (self.y.round() as u32).clamp(0, max_y);
        return (render_x, render_y);
    }

    pub fn update_velocity(&mut self, total_force: Force<f64>) {
        // F = M * A , so the acceleration is
        // A = F / M
        let acceleration = total_force / PARTICLE_MASS;

        // Now, the new velocity would be:
        // V = V0 + A*dt
        // where V0 is the previous acceleration
        // and "dt" is a how much "time" passed between now and the previous time update was called
        // Note: this time is relative to the simulation, and not real-life time

        self.velocity = self.velocity.clone() + acceleration * TIMESTEP;
    }

    pub fn update_position(&mut self) {
        // X = X0 + V*dt
        // where X0 is the previous position (on the x coordinate)
        // and "dt" is a how much "time" passed between now and the previous time update was called
        // Note: this time is relative to the simulation, and not real-life time

        self.x = self.x + self.velocity.x_component * TIMESTEP;
        self.y = self.y + self.velocity.y_component * TIMESTEP;
    }
}
