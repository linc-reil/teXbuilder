use std::fs;
use std::process::{Command, Stdio};
use thiserror::Error;

const COUNT_COMMAND: &'static str = "texcount {}";

#[derive(Debug, Error)]
pub enum CountError {
    #[error("could not find file \"{0}\" in the current working directory.")]
    FileNotFound(String),
    #[error("could not run command \"texcount\".")]
    CommandError,
    #[error("error interacting with filesystem.")]
    IOError,
}

pub fn count(filename: String) -> Result<(), CountError> {
    match fs::exists(filename.clone()) {
        Ok(false) => return Err(CountError::FileNotFound(filename.clone())),
        Err(_) => return Err(CountError::IOError),
        _ => {}
    }

    let cmd = COUNT_COMMAND.replace("{}", &filename);

    let output = Command::new("sh")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .arg("-c")
        .arg(&cmd)
        .output();

    match output {
        Ok(_) => Ok(()),
        Err(_) => Err(CountError::CommandError),
    }
}
