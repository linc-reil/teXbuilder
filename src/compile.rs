use crate::document;
use std::fs;
use std::process::{Command, Stdio};
use thiserror::Error;

const PDFLATEX_COMMAND: &'static str =
    "pdflatex -synctex=1 --shell-escape -interaction=nonstopmode {}.tex";
const BIBLATEX_COMMAND: &'static str = "biblatex {}.aux";
const BIBER_COMMAND: &'static str = "biber {}";
const MAKEGLOSSARIESS_COMMAND: &'static str = "makeglossaries {}";

#[derive(Debug, Error)]
pub enum LaTeXCompilationError {
    #[error("error reading filesystem")]
    IOError,
    #[error("no file found of filename \"{0}\"")]
    FileNotFound(String),
    #[error("no LaTeX files found in current working directory")]
    NoFilesFound,
    #[error("compile command \"{0}\" failed to run")]
    CommandError(String),
    #[error(
        "cannot specify a given filename \"{0}\" and the --all flag at the same time. see \"texbuilder build --help\" for a list of available subcommands."
    )]
    FilenameGivenButAllOptionSpecified(String),
    #[error(
        "no files or --all flag specified. see \"texbuilder build --help\" for a list of available subcommands."
    )]
    NothingSpecified,
}

fn remove_file_extension(filename: String) -> String {
    match filename.rfind('.') {
        Some(pos) => filename[..pos].to_string(),
        None => filename,
    }
}

fn find_all_latex_files(file_list: &mut Vec<String>) -> Result<(), LaTeXCompilationError> {
    let mut files: Vec<String> = Vec::new();

    let cwd = match std::env::current_dir() {
        Ok(path) => path,
        Err(_) => return Err(LaTeXCompilationError::IOError),
    };

    let dir_contents = match fs::read_dir(cwd) {
        Ok(contents) => contents,
        Err(_) => return Err(LaTeXCompilationError::IOError),
    };

    for item in dir_contents {
        let item = match item {
            Ok(val) => val,
            Err(_) => return Err(LaTeXCompilationError::IOError),
        };
        let path = item.path();
        if !path.is_dir()
            && path
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("tex"))
                .unwrap_or(false)
        {
            let filename = match path.file_name() {
                Some(val) => match val.to_owned().into_string() {
                    Ok(string) => string,
                    Err(_) => return Err(LaTeXCompilationError::IOError),
                },
                None => return Err(LaTeXCompilationError::IOError),
            };
            files.push(filename);
        }
    }

    if files.len() == 0usize {
        return Err(LaTeXCompilationError::NoFilesFound);
    }

    let mut files = files
        .into_iter()
        .map(|file| remove_file_extension(file))
        .collect();

    file_list.append(&mut files);

    Ok(())
}

fn execute_command(command: &str, verbose: bool) -> Result<(), LaTeXCompilationError> {
    let output = match verbose {
        true => Command::new("sh")
            .stdout(Stdio::inherit())
            .stdin(Stdio::inherit())
            .arg("-c")
            .arg(command)
            .output(),
        false => Command::new("sh").arg("-c").arg(command).output(),
    };
    if output.is_err() {
        Err(LaTeXCompilationError::CommandError(command.to_string()))
    } else {
        Ok(())
    }
}

fn execute_build_commands(
    filename: String,
    bibcmd: document::BiblatexCommand,
    glossary: bool,
    verbose: bool,
) -> Result<(), LaTeXCompilationError> {
    let pdfl_cmd = PDFLATEX_COMMAND.replace("{}", &filename);
    execute_command(&pdfl_cmd, verbose)?;
    let bib_cmd: String = match bibcmd {
        document::BiblatexCommand::Biber => BIBER_COMMAND.replace("{}", &filename),
        document::BiblatexCommand::Biblatex => BIBLATEX_COMMAND.replace("{}", &filename),
        document::BiblatexCommand::None => {
            "echo 'no bibliography command, continuing...'".to_string()
        }
    };
    execute_command(&bib_cmd, verbose)?;

    if glossary {
        execute_command(&MAKEGLOSSARIESS_COMMAND.replace("{}", &filename), verbose)?;
    }

    execute_command(&pdfl_cmd, verbose)?;

    Ok(())
}

pub fn compile(
    filename: Option<String>,
    bibcmd: document::BiblatexCommand,
    glossary: bool,
    all: bool,
) -> Result<(), LaTeXCompilationError> {
    let mut files: Vec<String> = Vec::new();
    match (filename, all) {
        (Some(file), false) => {
            match fs::exists(file.clone()) {
                Ok(true) => {}
                Ok(false) => return Err(LaTeXCompilationError::FileNotFound(file)),
                Err(_) => return Err(LaTeXCompilationError::IOError),
            }
            files.push(remove_file_extension(file));
        }
        (None, true) => find_all_latex_files(&mut files)?,
        (None, false) => return Err(LaTeXCompilationError::NothingSpecified),
        (Some(file), true) => {
            return Err(LaTeXCompilationError::FilenameGivenButAllOptionSpecified(
                file,
            ));
        }
    }

    if files.len() == 0usize {
        return Err(LaTeXCompilationError::NoFilesFound);
    }

    for file in files {
        execute_build_commands(file, bibcmd, glossary, false)?;
    }

    Ok(())
}
