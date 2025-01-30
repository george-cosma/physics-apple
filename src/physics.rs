use std::{
    error::{self, Error},
    path::Path,
};

use image::GenericImageView;

use crate::physics::board::{Board, BoardCell};

pub mod board;
mod engine;
pub mod force;
pub mod particle;

#[derive(PartialEq)]
pub enum GenerateResult {
    FieldGenerated,
    FieldLoaded,
}

pub fn generate_board(file: &String) -> Result<(Board, GenerateResult), Box<dyn Error>> {
    println!("[Debug] Generating board for '{}'.", file);

    // Load Image
    let img = load_image(file);

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
    match update_static_field(file, &mut board, img) {
        Ok(generate_result) => {
            return Ok((board, generate_result));
        }
        Err(err) => {
            panic!();
        }
    }
}

pub fn load_image(filename: &String) -> image::DynamicImage {
    let path = Path::new(&filename);
    let img = image::open(path).unwrap().grayscale();
    img
}

pub fn update_static_field(
    frame_filename: &String,
    board: &mut Board,
    img: image::DynamicImage,
) -> Result<GenerateResult, Box<dyn Error>> {
    let str_field_path = format!("{}.field", frame_filename);
    let field_path = Path::new(&str_field_path);
    if Path::exists(&field_path) {
        println!(
            "[Debug] Found static attraction field for '{}'.",
            frame_filename
        );

        let load_result = board.load_static_field(field_path);
        match load_result {
            Ok(_) => {
                if !board.is_field_corrupted() {
                    return Ok(GenerateResult::FieldLoaded);
                } else {
                    board.clear_static_field();
                }
            }
            Err(_) => (),
        }
        println!("Corrupted field '{frame_filename}'")
    }
    println!(
        "[Debug] Generating static attraction field for '{}'.",
        frame_filename
    );
    board.CUDA_generate_static_field(get_attractors(img));

    Ok(GenerateResult::FieldGenerated)
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
