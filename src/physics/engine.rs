use super::force::Force;

pub const ATTRACTOR_MASS: f32 = 10.0;
pub const PARTICLE_MASS: f32 = 1.0;

pub const TIMESTEP: f32 = 1.0;

const G: f32 = 1.0 / 1000.0;
pub fn gravitational_force(
    x1: u32,
    y1: u32,
    mass1: f32,
    x2: u32,
    y2: u32,
    mass2: f32,
) -> Force<f32> {
    let rx = (x2 as i32) - (x1 as i32);
    let ry = (y2 as i32) - (y1 as i32);
    let radius_squared = (rx * rx + ry * ry) as f32;

    if radius_squared == 0.0 {
        return Force::default();
    }
    let cos_alpha = (rx as f32) / (radius_squared as f32);
    let sin_alpha = (ry as f32) / (radius_squared as f32);

    // LAW IS:
    // F = G * m1 * m2 / r^2
    // Where G is a constant. We'll set this constant however we want, as to fine-tune our simulation.
    Force {
        x_component: G * mass1 * mass2 / radius_squared * cos_alpha,
        y_component: G * mass1 * mass2 / (radius_squared) * sin_alpha,
    }
}
