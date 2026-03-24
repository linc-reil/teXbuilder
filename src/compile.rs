use crate::document;
use std::process::Command;
use thiserror::Error;
use std::fs;

const PDFLATEX_COMMAND: &'static str =
    "pdflatex -synctex=1 --shell-escape -interaction=nonstopmode {}.tex";
const BIBLATEX_COMMAND: &'static str = "biblatex {}.aux";
const BIBER_COMMAND: &'static str = "biber {}";
const MAKEGLOSSARIESS_COMMAND: &'static str = "makeglossaries {}"

#[derive(Debug, Error)]
pub enum LaTeXCompilationError {
    #[error("error reading filesystem")]
    IOError,
    #[error("no file found of filename \"{0}\"")]
    FileNotFound(String),
    #[error("no LaTeX files found in current working directory")]
    NoFilesFound
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
        Err(_) => return Err(LaTeXCompilationError::IOError)
    };

    let dir_contents = match fs::read_dir(cwd) {
        Ok(contents) => contents,
        Err(_) => return Err(LaTeXCompilationError::IOError)
    };

    for item in dir_contents {
        let item = match item {
            Ok(val) => val,
            Err(_) => return Err(LaTeXCompilationError::IOError)
        };
        let path = item.path();
        if !path.is_dir() && path.extension().and_then(|ext| ext.to_str()).map(|ext| ext.eq_ignore_ascii_case("tex")).unwrap_or(false) {
            let filename = match path.file_name() {
                Some(val) => match val.to_owned().into_string() {
                    Ok(string) => string,
                    Err(_) => return Err(LaTeXCompilationError::IOError)
                },
                None => return Err(LaTeXCompilationError::IOError)
            };
            files.push(filename);
        }
    }

    if files.len() == 0usize {
        return Err(LaTeXCompilationError::NoFilesFound)
    }

    file_list.append(&mut files);

    Ok(())
}

pub fn compile(filename: Option<String>, bibcmd: document::BiblatexCommand, glossary: bool) -> Result<(), LaTeXCompilationError> {
    let mut files: Vec<String> = Vec::new();
    match filename {
        Some(file) => {
            match fs::exists(file.clone()) {
                Ok(true) => {},
                Ok(false) => return Err(LaTeXCompilationError::FileNotFound(file)),
                Err(_) => return Err(LaTeXCompilationError::IOError),
            }
            files.push(remove_file_extension(file));
        },
        None => find_all_latex_files(&mut files)?
    }

    Ok(())
}
