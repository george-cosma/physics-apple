#[macro_use]
extern crate rustacuda;

use std::{
    cell::RefCell,
    path::Path,
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

use clap::Parser;
use cli::{CLIArgs, Commands};
use gui::{HEIGHT, WIDTH};
use physics::{generate_board, load_image, GenerateResult};
use sequence::Sequence;

mod cli;
mod gpu;
mod gui;
mod physics;
mod sequence;

const USE_FPS: bool = true;
const FPS: u128 = 30;

const USE_FIXED_ITER: bool = false;
const ITER_PER_FRAME: u32 = 100;
fn main() {
    let args = CLIArgs::parse();

    match args.command {
        Commands::Generate { files, threads } => {
            generate_and_save_field_paralel(files, threads);
        }
        Commands::ViewField { file } => {
            view_field(&file);
        }
        Commands::SimulateFile { file } => {
            simulate_file(&file);
        }
        Commands::SimulateSequence {
            prefix,
            begin,
            end,
            suffix,
            save_to_file,
        } => {
            let seq = Sequence::new(
                &prefix,
                &suffix,
                begin as usize,
                end as usize,
                true,
            );
            if save_to_file {
                simulate_and_save_sequence(seq)
            } else {
                simulate_sequence(seq);
            }
        }
    }
}

fn generate_and_save_field_paralel(files: Vec<String>, user_threads: usize) {
    let max_threads = thread::available_parallelism().unwrap().get();
    let threads = if user_threads > max_threads {
        max_threads
    } else {
        user_threads
    };
    let frames = files.len();

    let mut handles = Vec::new();

    let seq_ref = Arc::new(Mutex::new(Sequence::new(
        &"".to_string(),
        &"".to_string(),
        0,
        frames - 1,
        false,
    )));

    for i in 0..threads {
        let seq_ref = Arc::clone(&seq_ref);
        let files = files.clone();
        let handle = thread::spawn(move || loop {
            let mut seq = seq_ref.lock().unwrap();
            let next = seq.next_number();
            drop(seq);

            if let Some(i) = next {
                generate_and_save_field(&files[i]);
            } else {
                break;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn generate_and_save_field(file: &String) {
    let (board, result) = physics::generate_board(file).unwrap();
    if result == GenerateResult::FieldGenerated {
        let str_path = format!("{}.field", file);
        let path = std::path::Path::new(&str_path);
        board.save_field(path).unwrap();
    }
}

fn view_field(file: &String) {
    let board_ref = Rc::new(RefCell::new(generate_board(file).unwrap().0));
    board_ref.borrow_mut().random_particles(WIDTH * HEIGHT / 8);

    gui::run(
        move |buffer| {
            board_ref.borrow().draw_static_field(buffer);
        },
        || {},
    );
}

fn simulate_file(file: &String) {
    let board_ref = Rc::new(RefCell::new(generate_board(file).unwrap().0));
    board_ref.borrow_mut().random_particles(WIDTH * HEIGHT / 8);

    let boar_ref_clone = board_ref.clone();
    gui::run(
        move |buffer| {
            board_ref.borrow().draw_particles(buffer);
        },
        move || {
            if USE_FPS {
                let start = std::time::Instant::now();
                let mut now = start.clone();
                let mut iter = 0;
                while (now - start).as_nanos() < (1_000_000_000 / FPS) {
                    boar_ref_clone.borrow_mut().update();
                    now = std::time::Instant::now();
                    iter += 1;
                }
                println!("Ran frame for {} iterations.", iter);
            } else if USE_FIXED_ITER {
                for _ in 0..ITER_PER_FRAME {
                    boar_ref_clone.borrow_mut().update();
                }
            } else {
                boar_ref_clone.borrow_mut().update();
            }
        },
    );
}

const SEQ_ITER_PER_FRAME: usize = 20;
// How many frames to output per input frame.
// On Bad Apple, this will make the video from 30 fps to 60fps
const FRAME_HOLD: usize = 2;

// How many frames to keep the simulation going after the last input frame has been simulated
const END_FRAMES: usize = 60 * 10;

fn simulate_sequence(mut seq: Sequence) {
    let board_ref = Rc::new(RefCell::new(
        generate_board(&seq.next().unwrap()).unwrap().0,
    ));
    board_ref.borrow_mut().random_particles(WIDTH * HEIGHT / 16);

    let boar_ref_clone = board_ref.clone();
    gui::run(
        move |buffer| {
            board_ref.borrow().draw_particles(buffer);
            if let Some(filename) = seq.next() {
                physics::update_static_field(
                    &filename,
                    &mut board_ref.borrow_mut(),
                    load_image(&filename),
                )
                .unwrap();
            }
        },
        move || {
            for _ in 0..SEQ_ITER_PER_FRAME {
                boar_ref_clone.borrow_mut().update();
            }
        },
    );
}

fn simulate_and_save_sequence(mut seq: Sequence) {
    let mut board = generate_board(&seq.next().unwrap()).unwrap().0;
    board.random_particles(WIDTH * HEIGHT / 16);

    let mut buffer_array = [0u8; (WIDTH * HEIGHT * 4) as usize];
    let buffer = buffer_array.as_mut_slice();

    let max_frames = (seq.end() - seq.start() + 1) * FRAME_HOLD + END_FRAMES;
    let frame_count_size = format!("{}", max_frames).len();

    let mut frame_hold_counter = FRAME_HOLD;

    for current_frame in 1..=max_frames {
        // Render to buffer
        board.draw_particles(buffer);

        // Save
        let path_str = format!(
            "./render/render.{:0>width$}.png",
            current_frame,
            width = frame_count_size
        );
        let path = Path::new(&path_str);
        image::save_buffer(path, buffer, WIDTH, HEIGHT, image::ColorType::Rgba8).unwrap();

        // Update particles
        for _ in 0..SEQ_ITER_PER_FRAME {
            board.update();
        }

        // Update field
        if frame_hold_counter != 1 {
            frame_hold_counter -= 1;
        } else {
            frame_hold_counter = FRAME_HOLD;

            if let Some(filename) = seq.next() {
                physics::update_static_field(&filename, &mut board, load_image(&filename)).unwrap();
            }
        }
    }
}
