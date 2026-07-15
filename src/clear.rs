use std::{env, fs};

use thiserror::Error;

type ClearResult = Result<(), ClearError>;

#[derive(Debug, Error)]
pub enum ClearError {
    #[error("error interacting with filesystem.")]
    IOError,
    #[error("no LaTeX files found in the current working directory.")]
    NoFilesFound,
    #[error("could not find file \"{0}\" in the current working directory.")]
    CouldNotFindFile(String),
    #[error(
        "cannot specify a filename and the --all flag at the same time. See \"texbuilder clear --help\" for available subcommands."
    )]
    BadOptionsSpecified,
    #[error(
        "no options were specified. See \"texbuilder clear --help\" for available subcommands."
    )]
    NoOptionsSpecified,
    #[error("error removing file {0}.")]
    ErrorRemovingFile(String),
}

fn remove_file_extension(filename: String) -> String {
    match filename.rfind('.') {
        Some(pos) => filename[..pos].to_string(),
        None => filename,
    }
}

fn get_all_tex_files(filename_vec: &mut Vec<String>) -> ClearResult {
    let mut filenames: Vec<String> = Vec::new();

    let cwd = match env::current_dir() {
        Ok(path) => path,
        Err(_) => return Err(ClearError::IOError),
    };

    let dir_contents = match fs::read_dir(cwd) {
        Ok(contents) => contents,
        Err(_) => return Err(ClearError::IOError),
    };

    for item in dir_contents {
        let item = match item {
            Ok(val) => val,
            Err(_) => return Err(ClearError::IOError),
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
                    Err(_) => return Err(ClearError::IOError),
                },
                None => return Err(ClearError::IOError),
            };
            filenames.push(filename);
        }
    }

    if filenames.len() == 0usize {
        return Err(ClearError::NoFilesFound);
    }

    let mut files = filenames
        .into_iter()
        .map(|file| remove_file_extension(file))
        .collect();

    filename_vec.append(&mut files);

    Ok(())
}

fn remove_related_files(name: String, pdf: bool) -> ClearResult {
    let cwd = match env::current_dir() {
        Ok(path) => path,
        Err(_) => return Err(ClearError::IOError),
    };

    let dir_contents = match fs::read_dir(cwd) {
        Ok(contents) => contents,
        Err(_) => return Err(ClearError::IOError),
    };

    let is_deletable = |ext: &str| match (ext, pdf) {
        ("tex", _) => false,
        ("pdf", false) => false,
        ("pdf", true) => true,
        ("", _) => false,
        _ => true,
    };

    for item in dir_contents {
        let item = match item {
            Ok(val) => val,
            Err(_) => return Err(ClearError::IOError),
        };

        let path = item.path();
        let extension = match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) => ext,
            None => "",
        };

        let filename = match path.file_name() {
            Some(fi) => fi.to_str().unwrap_or("a.tex"),
            None => continue,
        };

        if !path.is_dir() && filename.starts_with(&name) && is_deletable(extension) {
            match fs::remove_file(path.clone()) {
                Ok(_) => continue,
                Err(_) => {
                    return Err(ClearError::ErrorRemovingFile(
                        path.clone().to_string_lossy().to_string(),
                    ));
                }
            }
        }
    }

    Ok(())
}

pub fn clear(filename: Option<String>, pdf: bool, all: bool) -> ClearResult {
    let mut filenames: Vec<String> = Vec::new();
    match (filename, all) {
        (Some(name), false) => match fs::exists(name.clone()) {
            Ok(true) => filenames.push(name),
            Ok(false) => return Err(ClearError::CouldNotFindFile(name.clone())),
            Err(_) => return Err(ClearError::IOError),
        },
        (None, true) => get_all_tex_files(&mut filenames)?,
        (Some(_), true) => return Err(ClearError::BadOptionsSpecified),
        (None, false) => return Err(ClearError::NoOptionsSpecified),
    }

    for file in filenames {
        remove_related_files(file, pdf)?;
    }

    Ok(())
}
