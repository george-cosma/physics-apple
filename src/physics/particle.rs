use super::{
    engine::{PARTICLE_MASS, TIMESTEP},
    force::Force,
};

#[derive(Clone)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub velocity: Force<f32>,
}

impl Particle {
    pub fn get_render_position(&self, max_x: u32, max_y: u32) -> (u32, u32) {
        let render_x = (self.x.round() as u32).clamp(0, max_x);
        let render_y = (self.y.round() as u32).clamp(0, max_y);
        return (render_x, render_y);
    }

    pub fn is_inside(&self, max_x: u32, max_y: u32) -> bool {
        let unclamped_x = self.x.round() as i32;
        let unclamped_y = self.y.round() as i32;

        let (render_x, render_y) = self.get_render_position(max_x, max_y);
        return (unclamped_x == (render_x as i32)) && (unclamped_y == (render_y as i32));
    }

    pub fn is_heading_inside(&self, max_x: i32, max_y: i32) -> bool {
        let unclamped_x = self.x.round() as i32;
        let unclamped_y = self.y.round() as i32;

        let heading_inside_x = (unclamped_x < 0 && self.velocity.x_component >= 0.0)
            || (unclamped_x > max_x && self.velocity.x_component <= 0.0);

        let heading_inside_y = (unclamped_y < 0 && self.velocity.y_component > 0.0)
            || (unclamped_y > max_y && self.velocity.y_component <= 0.0);

        return heading_inside_x && heading_inside_y;
    }

    pub fn update_velocity(&mut self, total_force: Force<f32>) {
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
