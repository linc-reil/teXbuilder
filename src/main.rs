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
            all,
        } => {
            let result = compile::compile(file, bibcmd, glossary, all);
            if let Err(e) = result {
                println!("{}", e);
            }
        }
        cli::Command::Create => {
            println!("create dat bitch")
        }
    }
}
