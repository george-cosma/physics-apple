use std::ops::Mul;

use super::force::Force;

const G: f64 = 1.0 / 1000.0;
pub fn gravitational_force(
    x1: u32,
    y1: u32,
    mass1: f64,
    x2: u32,
    y2: u32,
    mass2: f64,
) -> Force<f64> {
    let rx = (x2 as i32) - (x1 as i32);
    let ry = (y2 as i32) - (y1 as i32);
    let radius_squared = (rx * rx + ry * ry) as f64;

    if radius_squared == 0.0 {
        return Force::default();
    }
    let cos_alpha = (rx as f64) / (radius_squared as f64);
    let sin_alpha = (ry as f64) / (radius_squared as f64);

    // LAW IS:
    // F = G * m1 * m2 / r^2
    // Where G is a constant. We'll set this constant however we want, as to fine-tune our simulation.
    Force { 
        x_component: G * mass1 * mass2 / radius_squared * cos_alpha, 
        y_component: G * mass1 * mass2 / (radius_squared) * sin_alpha
    }
}
