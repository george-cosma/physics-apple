use std::{cell::RefCell, rc::Rc};

use super::{engine::gravitational_force, force::Force, particle::Particle};

pub struct Board {
    pub width: u32,
    pub heigth: u32,
    pub cells: Vec<BoardCell>,
    pub particles: Vec<Particle>,
}

#[derive(Default)]
pub struct BoardCell {
    pub x: u32,
    pub y: u32,
    pub static_field: Force<f64>,
    pub particles: Vec<Rc<RefCell<Particle>>>,
}
impl BoardCell {
    pub fn new(x: u32, y: u32) -> BoardCell {
       BoardCell { x: x, y: y, ..Default::default() } 
    }
}

const ATTRACTOR_MASS: f64 = 10.0;
const PARTICLE_MASS: f64 = 1.0;
impl Board {
    pub fn generate_static_field(&mut self, attractors: Vec<(u32, u32)>) {
        for y in 0..self.heigth {
            println!("[DEBUG] y = {}", y);
            for x in 0..self.width {
                for (a_x, a_y) in &attractors {
                    self.get_cell(x, y).static_field +=
                        gravitational_force(x, y, PARTICLE_MASS, *a_x, *a_y, ATTRACTOR_MASS);
                }
            }
        }
    }

    pub fn get_cell(&mut self, x: u32, y: u32) -> &mut BoardCell {
        return &mut self.cells[(x + y * self.width) as usize];
    }
}
