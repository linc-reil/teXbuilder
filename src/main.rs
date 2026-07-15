use clap::Parser;
mod clear;
mod cli;
mod compile;
mod count;
mod document;

macro_rules! run_subcommand {
    ($subcommand:expr) => {
        let output = $subcommand;
        if let Err(e) = output {
            println!("{e}");
        }
    };
}

fn main() {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Command::Build {
            file,
            bibcmd,
            glossary,
            all,
            clear,
        } => {
            run_subcommand!(compile::compile(file.clone(), bibcmd, glossary, all));
            if clear {
                run_subcommand!(clear::clear(file, false, all));
            }
        }
        cli::Command::Create => {
            todo!()
        }

        cli::Command::Count { filename } => {
            run_subcommand!(count::count(filename));
        }

        cli::Command::Clear { filename, pdf, all } => {
            run_subcommand!(clear::clear(filename, pdf, all));
        }
    }
}
