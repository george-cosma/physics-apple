use clap::{command, Parser, Subcommand};

/// A program to generate a particle-based simulation
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CLIArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Generate the static field for a file.
    #[command(arg_required_else_help = true)]
    Generate {
        /// The files to generate the static field for.
        files: Vec<String>,
    },

    /// Simulate a single file.
    #[command(arg_required_else_help = true)]
    SimulateFile {
        /// The file to base the simulation on.
        file: String,
    },

    /// Simulate a sequence of files. The files' format is <prefix><index><suffix>.
    #[command(arg_required_else_help = true)]
    SimulateSequence {
        /// The prefix of the files' names.
        #[arg(index = 1)]
        prefix: String,
        /// The first file index in the sequence.
        #[arg(index = 2)]
        begin: u32,
        /// The last file index in the sequence.
        #[arg(index = 3)]
        end: u32,
        /// The suffix of the files' names.
        #[arg(index = 4)]
        suffix: String,
    },
}
