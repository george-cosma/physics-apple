use std::{path::Path, error::{self, Error}};

use image::GenericImageView;

use crate::physics::board::{Board, BoardCell};

pub mod board;
mod engine;
pub mod force;
pub mod particle;

#[derive(PartialEq)]
pub enum GenerateResult {
    FieldGenerated,
    FieldLoaded
}

pub fn generate_board(file: &String) -> Result<(Board,GenerateResult), Box<dyn Error>> {
    println!("[Debug] Generating board for '{}'.", file);

    // Load Image
    let path = Path::new(&file);
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

    // Try loading static field
    let str_field_path = format!("{}.field", file);
    let field_path = Path::new(&str_field_path);

    if Path::exists(&field_path) {
        println!("[Debug] Found static attraction field for '{}'.", file);

        let load_result = board.load_static_field(field_path);
        match load_result {
            Ok(_) => return Ok((board, GenerateResult::FieldLoaded)) ,
            Err(_) => println!("Corrupted field '{file}'"),
        }
         
    }
    // Create static field
    println!("[Debug] Generating static attraction field for '{}'.", file);

    board.generate_static_field(get_attractors(img));
    return Ok((board, GenerateResult::FieldGenerated));
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
