/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-02-09
 */

pub mod error;
pub use error::Error;

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

pub struct GitCommand {}

impl GitCommand {
    /// Runs a Git command and returns its output if it succeeds.
    /// The payload can be anything that can be converted to a string
    pub fn git_command<T: ToString>(
        git_command: &str,
        args: Option<Vec<&str>>,
        payload: Option<T>,
        current_dir: Option<&PathBuf>,
    ) -> Result<String, error::Error> {
        let output = GitCommand::run_git_command(git_command, args, payload, current_dir)?;
        if output.status.success() {
            match std::str::from_utf8(&output.stdout) {
                Ok(stdout) => Ok(stdout.to_owned()),
                Err(error) => Err(error::Error::new(error.to_string())),
            }
        } else {
            match std::str::from_utf8(&output.stderr) {
                Ok(stderr) => Err(error::Error::new(stderr.to_owned())),
                Err(error) => Err(error::Error::new(error.to_string())),
            }
        }
    }
}

impl GitCommand {
    /// Returns a vector that includes a Git command and any subcommands, arguments, etc.
    fn git_args<'a>(command: &'a str, args: Option<Vec<&'a str>>) -> Vec<&'a str> {
        let mut git_args = vec![command];
        if let Some(args) = args {
            git_args.extend(&args);
        }
        git_args
    }

    /// Runs a Git command and returns its output
    /// The payload can be anything that can be converted to a string
    pub fn run_git_command<T: ToString>(
        git_command: &str,
        args: Option<Vec<&str>>,
        payload: Option<T>,
        current_dir: Option<&PathBuf>,
    ) -> Result<Output, error::Error> {
        let mut command = Command::new("git");
        if let Some(current_dir) = current_dir {
            command.current_dir(current_dir);
        }
        command.args(Self::git_args(git_command, args));
        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        match command.spawn() {
            Ok(mut child_process) => {
                if let Some(payload) = payload {
                    let mut payload = payload.to_string();
                    payload.push('\n');
                    match child_process.stdin.take() {
                        Some(mut stdin) => match stdin.write_all(payload.as_bytes()) {
                            Ok(_) => {}
                            Err(error) => return Err(error::Error::new(error.to_string())),
                        },
                        None => return Err(error::Error::new("Can't get stdin".to_owned())),
                    }
                }
                match child_process.wait_with_output() {
                    Ok(output) => Ok(output),
                    Err(error) => Err(error::Error::new(error.to_string())),
                }
            }
            Err(error) => Err(error::Error::new(error.to_string())),
        }
    }
}
