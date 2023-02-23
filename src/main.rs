use std::{cell::RefCell, rc::Rc};

use clap::Parser;
use cli::{CLIArgs, Commands};
use gui::{HEIGHT, WIDTH};
use physics::generate_board;

mod cli;
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
        Commands::Generate { files } => {
            for file in &files {
                generate_and_save_field(file);
            }
        }
        Commands::ViewField { file } => {
            view_field(&file);
        }
        Commands::SimulateFile { file } => {
            simulate_file(&file);
        }
        Commands::SimulateSequence {
            prefix: _,
            begin: _,
            end: _,
            suffix: _,
        } => todo!(),
    }
}

fn generate_and_save_field(file: &String) {
    let board = physics::generate_board(file);

    let str_path = format!("{}.field", file);
    let path = std::path::Path::new(&str_path);
    board.save_field(path).unwrap();
}

fn view_field(file: &String) {
    let board_ref = Rc::new(RefCell::new(generate_board(file)));
    board_ref.borrow_mut().random_particles(WIDTH * HEIGHT / 8);

    gui::run(
        move |buffer| {
            board_ref.borrow().draw_static_field(buffer);
        },
        || {},
    );
}

fn simulate_file(file: &String) {
    let board_ref = Rc::new(RefCell::new(generate_board(file)));
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