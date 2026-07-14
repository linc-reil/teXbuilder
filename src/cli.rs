use crate::document;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "teXbuilder")]
#[command(version = "0.1.0")]
#[command(about = "A simple command line tool for creating / building LaTeX documents.")]
pub struct Cli {
    /// The command to run
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Compile LaTeX files in the current working directory.
    Build {
        /// The file to build. If not specified, the --all flag must be set.
        file: Option<String>,

        /// Which Biblatex command to use.
        #[arg(short, long, default_value_t = document::BiblatexCommand::Biblatex)]
        bibcmd: document::BiblatexCommand,

        /// Whether or not to run makeglossaries
        #[arg(short, long, default_value_t = false)]
        glossary: bool,

        /// Whether or not to compile all LaTeX files in the current working directory.
        #[arg(short, long, default_value_t = false)]
        all: bool,
    },

    /// Create a new LaTeX document using a TUI.
    Create,
}
