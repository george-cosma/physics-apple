use std::{error::Error, path::Path};

use image::GenericImageView;

use crate::physics::board::{Board, BoardCell};

pub mod board;
mod engine;
pub mod force;
pub mod particle;

#[derive(PartialEq)]
pub enum FieldLoadOutcome {
    /// The static attraction field was just generated.
    FieldGenerated,
    /// An existing static attraction field was loaded.
    FieldLoaded,
}

/// Generates a board from an image file. The board will have the same dimensions as the image.
///
/// # Arguments
/// - file: The path to the image file.
///
/// # Returns
/// A tuple containing the generated board and the outcome of the field loading. The field loading
/// outcome can be used to determine if the field was generated or loaded from disk.
pub fn generate_board(
    file: &str,
    use_gpu: bool,
) -> Result<(Board, FieldLoadOutcome), Box<dyn Error>> {
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

    // Try loading or generating the static field.
    let field_result = update_static_field(file, &mut board, img, use_gpu)?;

    Ok((board, field_result))
}

pub fn load_image(full_path: &str) -> image::DynamicImage {
    let path = Path::new(&full_path);
    image::open(path).unwrap().grayscale()
}

pub fn update_static_field(
    frame_filename: &str,
    board: &mut Board,
    img: image::DynamicImage,
    use_gpu: bool,
) -> Result<FieldLoadOutcome, Box<dyn Error>> {
    let str_field_path = format!("{}.field", frame_filename);
    let field_path = Path::new(&str_field_path);
    if Path::exists(&field_path) {
        println!(
            "[Debug] Found static attraction field for '{}'.",
            frame_filename
        );

        let load_result = board.load_static_field(field_path);
        if load_result.is_ok() {
            if !board.is_field_corrupted() {
                return Ok(FieldLoadOutcome::FieldLoaded);
            } else {
                board.clear_static_field();
            }
        }
        println!("Corrupted field '{frame_filename}'")
    }

    println!(
        "[Debug] Generating static attraction field for '{}'.",
        frame_filename
    );

    if use_gpu {
        board.cuda_generate_static_field(get_attractors(img));
    } else {
        board.generate_static_field(get_attractors(img));
    }
    Ok(FieldLoadOutcome::FieldGenerated)
}

fn get_attractors(img: image::DynamicImage) -> Vec<(u32, u32)> {
    let mut attractors = vec![];
    for (x, y, color) in img.pixels() {
        if filter_white(color) {
            attractors.push((x, y));
        }
    }
    return attractors;
}

/// Returns true if the pixel is "close enough" to white. In this case, "close enough" means all color channels are above half active.
fn filter_white(color: image::Rgba<u8>) -> bool {
    color.0[0] > 255 / 2 && color.0[1] > 255 / 2 && color.0[2] > 255 / 2 && color.0[3] > 255 / 2
}
