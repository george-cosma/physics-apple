use clap::{command, Parser, Subcommand};

/// A program to generate a particle-based simulation. You can exit with ESC or Q.
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct CLIArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Generate the static field for an entire directory.
    #[command(arg_required_else_help = true)]
    Generate {
        /// Path to directory containing the desired files. Program will only generate files with the
        /// .png extension.
        path: String,
        /// How many threads to use. Has no effect if processing is done on the GPU. Default: Maximum
        #[arg(short, long)]
        threads: Option<usize>,
        /// Use the GPU to generate the static field. Only supports NVIDIA GPUs.
        #[arg(short, long)]
        gpu: bool,
    },

    /// View the static field of a file.
    #[command(arg_required_else_help = true)]
    ViewField {
        /// The path to the file you want to see the static field.
        file: String,
    },

    /// Simulate a single file.
    #[command(arg_required_else_help = true)]
    SimulateFile {
        /// The file to base the simulation on.
        file: String,
    },

    /// Simulate a sequence of files from a directory by their alphabetical order. Make sure the
    /// files have leading zeros whe numbered.
    #[command(arg_required_else_help = true)]
    SimulateSequence {
        /// Path to directory containing the desired files. Program will simulate files with the
        /// .field extension.
        path: String,

        /// Enable saving the simulation to a file.
        #[arg(short, long)]
        save_to_file: bool,
    },
}
