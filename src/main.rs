#[macro_use]
extern crate rustacuda;

use std::{
    cell::RefCell,
    io::BufWriter,
    path::{Path, PathBuf},
    rc::Rc,
    sync::{
        atomic::{AtomicBool, AtomicUsize},
        Arc, Mutex,
    },
    thread,
};

use byteorder::{LittleEndian, WriteBytesExt};
use clap::Parser;
use cli::{CLIArgs, Commands};
use gui::{HEIGHT, WIDTH};
use physics::{force::Force, generate_board, load_image, FieldLoadOutcome};

mod cli;
mod gpu;
mod gui;
mod physics;

const USE_FPS: bool = true;
const FPS: u128 = 30;

const USE_FIXED_ITER: bool = false;
const ITER_PER_FRAME: u32 = 100;
fn main() {
    let args = CLIArgs::parse();

    match args.command {
        Commands::Generate { path, threads, gpu } => {
            let files = list_directory(path);
            if files.is_empty() {
                println!("No files found in directory.");
                return;
            }

            if gpu {
                generate_fields_gpu(files);
            } else {
                generate_fields(
                    files,
                    threads.unwrap_or(thread::available_parallelism().unwrap().get()),
                );
            }
        }
        Commands::ViewField { file } => {
            view_field(&file);
        }
        Commands::SimulateFile { file } => {
            simulate_file(&file);
        }
        Commands::SimulateSequence { path, save_to_file } => {
            let files = list_directory(path);

            if save_to_file {
                simulate_and_save_sequence(files)
            } else {
                simulate_sequence(files);
            }
        }
    }
}

fn list_directory(path: String) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(path).expect("Invalid input directory!") {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().unwrap() == "png" {
            files.push(path);
        }
    }

    files.sort();
    files
}

fn generate_fields(files: Vec<PathBuf>, thread_count: usize) {
    let frames = files.len();
    let mut handles = Vec::new();
    let next_frame = Arc::new(AtomicUsize::new(0));

    for _ in 0..thread_count {
        let files = files.clone();
        // Clone the arc so each frame gets a unique reference :)
        let next_frame = next_frame.clone();
        let handle = thread::spawn(move || loop {
            // We don't actually care if this is reordered, though it probably won't.
            let next_idx = next_frame.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

            if next_idx < frames {
                let path_str = files[next_idx].to_str().unwrap();
                let (board, result) = physics::generate_board(&path_str, false).unwrap();

                if result == FieldLoadOutcome::FieldGenerated {
                    let str_path = format!("{}.field", path_str);
                    let path = std::path::Path::new(&str_path);
                    board.save_field(path).unwrap();
                }
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

fn generate_fields_gpu(files: Vec<PathBuf>) {
    let result_buffer: Arc<Mutex<Vec<(Vec<Force<f32>>, PathBuf)>>> =
        Arc::new(Mutex::new(Vec::new()));

    let done = Arc::new(AtomicBool::new(false));

    let file_write_thread = thread::spawn({
        let result_buffer = result_buffer.clone();
        let done = done.clone();

        move || loop {
            let mut buffer = result_buffer.lock().unwrap();
            if !buffer.is_empty() {
                println!("{} fields to write.", buffer.len());
                let (forces, path) = buffer.pop().unwrap();
                drop(buffer);
                let mut file = BufWriter::new(std::fs::File::create(path).unwrap());
                for force in forces {
                    file.write_f32::<LittleEndian>(force.x_component).unwrap();
                    file.write_f32::<LittleEndian>(force.y_component).unwrap();
                }
            } else {
                if done.load(std::sync::atomic::Ordering::SeqCst) {
                    break;
                }
            }
        }
    });

    for file in files {
        let path_str = file.to_str().unwrap();
        let (board, result) = physics::generate_board(&path_str, true).unwrap();

        if result == FieldLoadOutcome::FieldGenerated {
            let str_path = format!("{}.field", path_str);
            let path_buf = std::path::Path::new(&str_path).to_path_buf();
            result_buffer
                .lock()
                .unwrap()
                .push((board.to_field(), path_buf));
        }
    }

    done.store(true, std::sync::atomic::Ordering::SeqCst);

    file_write_thread.join().unwrap();
}

fn view_field(file: &String) {
    let board_ref = Rc::new(RefCell::new(generate_board(file, false).unwrap().0));
    board_ref.borrow_mut().random_particles(WIDTH * HEIGHT / 8);

    gui::run(
        move |buffer| {
            board_ref.borrow().draw_static_field(buffer);
        },
        || {},
    );
}

fn simulate_file(file: &String) {
    let board_ref = Rc::new(RefCell::new(generate_board(file, false).unwrap().0));
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
const FRAME_HOLD: usize = 1;

// How many frames to keep the simulation going after the last input frame has been simulated
const END_FRAMES: usize = 48 * 5;

const REALTIME_FPS: usize = 30;

fn simulate_sequence(files: Vec<PathBuf>) {
    let mut file_counter = 0;

    let board_ref = Rc::new(RefCell::new(
        generate_board(&files[0].to_str().unwrap(), false)
            .unwrap()
            .0,
    ));
    board_ref.borrow_mut().random_particles(WIDTH * HEIGHT / 16);

    let mut time_since_last_frame = std::time::Instant::now();
    let boar_ref_clone = board_ref.clone();
    gui::run(
        move |buffer| {
            let elapsed = time_since_last_frame.elapsed();
            let frame_time = std::time::Duration::from_secs(1) / REALTIME_FPS as u32;
            if elapsed < frame_time {
                return;
            }

            board_ref.borrow().draw_particles(buffer);

            file_counter += 1;

            if file_counter < files.len() {
                let filename = files[file_counter].to_str().unwrap();
                physics::update_static_field(
                    &filename,
                    &mut board_ref.borrow_mut(),
                    load_image(&filename),
                    false,
                )
                .unwrap();
            }

            time_since_last_frame = std::time::Instant::now();
        },
        move || {
            for _ in 0..SEQ_ITER_PER_FRAME {
                boar_ref_clone.borrow_mut().update();
            }
        },
    );
}

fn simulate_and_save_sequence(files: Vec<PathBuf>) {
    let mut board = generate_board(files[0].to_str().unwrap(), false).unwrap().0;
    board.random_particles(WIDTH * HEIGHT / 16);

    let mut buffer_array = Box::new([0u8; (WIDTH * HEIGHT * 4) as usize]);
    let buffer = buffer_array.as_mut_slice();

    let max_frames = files.len() * FRAME_HOLD + END_FRAMES;
    let frame_count_size = format!("{}", max_frames).len();

    let mut frame_hold_counter = FRAME_HOLD;
    let mut file_counter = 1;

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

            if file_counter < files.len() {
                let filename = files[file_counter].to_str().unwrap();
                physics::update_static_field(&filename, &mut board, load_image(&filename), false)
                    .unwrap();
            }
            file_counter += 1;
        }
    }
}
