use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{
    cell::RefCell,
    fs::File,
    io::{Cursor, Error, Read, Write},
    rc::Rc,
};

use rand::Rng;

use super::{engine::gravitational_force, force::Force, particle::Particle};
use crate::physics::engine::{ATTRACTOR_MASS, PARTICLE_MASS};
pub struct Board {
    pub width: u32,
    pub heigth: u32,
    pub cells: Vec<BoardCell>,
    pub particles: Vec<Rc<RefCell<Particle>>>,
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
        BoardCell {
            x: x,
            y: y,
            ..Default::default()
        }
    }
}

impl Board {
    pub fn generate_static_field(&mut self, attractors: Vec<(u32, u32)>) {
        for y in 0..self.heigth {
            println!("[DEBUG] y = {}", y);
            for x in 0..self.width {
                for (a_x, a_y) in &attractors {
                    self.get_cell_mut(x, y).static_field +=
                        gravitational_force(x, y, PARTICLE_MASS, *a_x, *a_y, ATTRACTOR_MASS);
                }
            }
        }
    }

    pub fn update(&mut self) {
        // Update velocities of particles
        for particle_ref in &self.particles {
            let mut particle = particle_ref.borrow_mut();
            let (x, y) = particle.get_render_position(self.width - 1, self.heigth - 1);
            let force = self.get_cell(x, y).static_field.clone(); // + attraction to other particles, in future
            particle.update_velocity(force);
        }
        // Update positions of particles
        // TODO: cloning the whole particle array seems expensive... should fix later (do measure first)
        for particle_ref in self.particles.clone() {
            let mut particle = particle_ref.borrow_mut();
            let (x, y) = particle.get_render_position(self.width - 1, self.heigth - 1);
            particle.update_position();

            // Move particle to new cell, if needed
            let (new_x, new_y) = particle.get_render_position(self.width - 1, self.heigth - 1);
            if x != new_x || y != new_y {
                self.get_cell_mut(x, y)
                    .remove_particle(particle_ref.clone());
                self.get_cell_mut(new_x, new_y)
                    .add_particle(particle_ref.clone());
            }
        }
    }

    pub fn random_particles(&mut self, amount: u32) {
        for i in 0..amount {
            let x = rand::thread_rng().gen_range(0..self.width);
            let y = rand::thread_rng().gen_range(0..self.heigth);

            self.add_particle(x, y);
        }
    }

    pub fn add_particle(&mut self, x: u32, y: u32) {
        let particle = Particle {
            x: x as f64,
            y: y as f64,
            velocity: Force::default(),
        };

        let particle_ref = Rc::new(RefCell::new(particle));

        self.get_cell_mut(x, y).add_particle(particle_ref.clone());
        self.particles.push(particle_ref);
    }

    pub fn get_cell(&self, x: u32, y: u32) -> &BoardCell {
        return &self.cells[(x + y * self.width) as usize];
    }

    pub fn get_cell_mut(&mut self, x: u32, y: u32) -> &mut BoardCell {
        return &mut self.cells[(x + y * self.width) as usize];
    }

    pub fn draw_static_field(&self, pixels: &mut [u8]) {
        for (cell, pixel) in self.cells.iter().zip(pixels.chunks_exact_mut(4)) {
            let color = if cell.static_field.x_component == 0.0
                && cell.static_field.y_component == 0.0
            {
                [0, 0, 0, 0xff]
            } else if cell.static_field.x_component >= 0.0 && cell.static_field.y_component >= 0.0 {
                [0xff, 0, 0, 0xff]
            } else if cell.static_field.x_component <= 0.0 && cell.static_field.y_component >= 0.0 {
                [0, 0xff, 0, 0xff]
            } else if cell.static_field.x_component <= 0.0 && cell.static_field.y_component <= 0.0 {
                [0, 0, 0xff, 0xff]
            } else if cell.static_field.x_component >= 0.0 && cell.static_field.y_component <= 0.0 {
                [0xff, 0xff, 0, 0xff]
            } else {
                println!("[ERROR] Vector somehow breaks laws of physics. Cell[{}][{}] has force (x: {}, y:{})", cell.x, cell.y, cell.static_field.x_component, cell.static_field.y_component);
                [0xff, 0xff, 0xff, 0xff]
            };

            pixel.copy_from_slice(&color);
        }
    }

    pub fn draw_particles(&self, pixels: &mut [u8]) {
        for (cell, pixel) in self.cells.iter().zip(pixels.chunks_exact_mut(4)) {
            // let color = if cell.particles.len() <= 0 {
            //     [0, 0, 0, 0xff]
            // } else {
            //     [0xff, 0xff, 0xff, (0xff_u8 / 4).saturating_mul(cell.particles.len() as u8)]
            // };
            let color = if cell.particles.len() == 0 {
                [0, 0, 0, 0xff]
            } else {
                [0xff, 0xff, 0xff, 0xff]
            };

            pixel.copy_from_slice(&color);
        }
    }

    pub fn load_static_field(&mut self, field_path: &std::path::Path) -> Result<(), Error> {
        let mut file = File::open(field_path)?;
        let mut bytes: Vec<u8> = vec![];
        file.read_to_end(&mut bytes)?;
        let mut reader = Cursor::new(bytes);

        for y in 0..self.heigth {
            for x in 0..self.width {
                let force_x = reader.read_f64::<LittleEndian>()?;
                let force_y = reader.read_f64::<LittleEndian>()?;
                self.get_cell_mut(x, y).static_field = Force {
                    x_component: force_x,
                    y_component: force_y,
                };
            }
        }

        return Ok(());
    }

    pub fn save_field(&self, path: &std::path::Path) -> Result<(), Error> {
        let mut file = File::create(path)?;
        for y in 0..self.heigth {
            for x in 0..self.width {
                let force = self.get_cell(x, y).static_field.clone();
                file.write_f64::<LittleEndian>(force.x_component)?;
                file.write_f64::<LittleEndian>(force.y_component)?;
            }
        }
        return Ok(());
    }
}

impl BoardCell {
    pub fn add_particle(&mut self, particle: Rc<RefCell<Particle>>) {
        self.particles.push(particle);
    }

    pub fn remove_particle(&mut self, particle: Rc<RefCell<Particle>>) {
        self.particles.retain(|x| !Rc::ptr_eq(&x, &particle));
    }

    // pub fn remove_particles(&mut self, particles: Vec<Rc<RefCell<Particle>>>) {
    //     for particle in particles {
    //         self.remove_particle(particle);
    //     }
    // }

    // pub fn lost_particles(&self) -> Vec<Rc<RefCell<Particle>>> {
    //     self.particles.clone()
    //         .into_iter()
    //         .filter(|&x| x.borrow().get_render_position(u32::MAX, u32::MAX) != (self.x, self.y))
    //         .collect()
    // }
}
