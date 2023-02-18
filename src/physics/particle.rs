use super::force::Force;

pub struct Particle {
    x: f64,
    y: f64,
    velocity: Force<f64>,
}
