use std::{cell, path::Path};

use image::GenericImageView;

use crate::physics::board::{Board, BoardCell};

pub mod board;
mod engine;
pub mod force;
pub mod particle;

pub fn generate_board(filename: String) -> Board {
    println!("[Debug] Generating board for '{}'.", &filename);

    // Load Image
    let str_path = format!("./frames/{}", filename);
    let path = Path::new(&str_path);
    let img = image::open(path).unwrap().grayscale();

    // Create Board
    let mut cells = Vec::with_capacity((img.width() * img.height()) as usize);
    for y in 0..img.height() {
        for x in 0..img.width() {
            cells.push(BoardCell::new(x, y));
        }
    }

    let particles = vec![];
    let mut board = Board {
        width: img.width(),
        heigth: img.height(),
        cells: cells,
        particles: particles,
    };

    // Create static field
    println!(
        "[Debug] Generatic static attraction field for '{}'.",
        &filename
    );

    board.generate_static_field(get_attractors(img));

    return board;
}

fn get_attractors(img: image::DynamicImage) -> Vec<(u32, u32)> {
    let mut attractors = vec![];
    for (x, y, color) in img.pixels() {
        if filter(color) {
            attractors.push((x, y));
        }
    }
    return attractors;
}

/// Returns true if the pixel is "close enough" to white. In this case, "close enough" means all color channels are above half active.
fn filter(color: image::Rgba<u8>) -> bool {
    color.0[0] > 255 / 2 && color.0[1] > 255 / 2 && color.0[2] > 255 / 2 && color.0[3] > 255 / 2
}
