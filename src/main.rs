#[macro_use]
extern crate rustacuda;

use std::{
    cell::RefCell,
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
        } => {
            simulate_sequence(Sequence::new(
                &prefix,
                &suffix,
                begin as usize,
                end as usize,
                true,
            ));
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

const SEQ_ITER_PER_FRAME: usize = 40;

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
