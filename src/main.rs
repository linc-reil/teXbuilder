use clap::Parser;
mod cli;
mod compile;
mod document;

fn main() {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Command::Build {
            file,
            bibcmd,
            glossary,
        } => compile::compile(file, bibcmd, glossary),
        cli::Command::Create => println!("create dat bitch"),
    }
}
