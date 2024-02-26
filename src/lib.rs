/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-02-09
 */

pub use crate::credentials::Credentials;
use crate::git_command::{Error as GitCommandError, GitCommand};
use std::path::PathBuf;
use std::str::FromStr;

pub mod credentials;
pub mod git_command;

pub struct GitLib {}

impl GitLib {
    /// Add a Git remote to a local repository
    pub fn remote_add(
        repo: &str,
        url: &str,
        path: Option<&PathBuf>,
    ) -> Result<(), GitCommandError> {
        let _ = GitCommand::git_command::<String>(
            "remote",
            Some(vec!["add", repo, url]),
            None,
            Some(&Self::path(path.cloned())),
        )?;
        Ok(())
    }

    /// Ask Git for the first URL of the named remote
    pub fn remote_url(repo: &str, path: Option<&PathBuf>) -> Result<String, GitCommandError> {
        let output = GitCommand::git_command::<String>(
            "remote",
            Some(vec!["get-url", repo]),
            None,
            Some(&Self::path(path.cloned())),
        )?;
        Ok(output.trim_end_matches('\n').to_owned())
    }

    /// Ask Git if a path is in a Git working directory.
    /// The Git documentation says this command will return false if the directory isn't inside a
    /// work tree, but in fact it will fail to run, because
    /// "fatal: not a git repository (or any of the parent directories): .git".
    pub fn is_inside_work_tree(path: Option<&PathBuf>) -> Result<bool, GitCommandError> {
        let _ = GitCommand::git_command::<String>(
            "rev-parse",
            Some(vec!["--is-inside-work-tree"]),
            None,
            Some(&Self::path(path.cloned())),
        )?;
        Ok(true)
    }

    /// Ask Git for the working directory related to a path.
    pub fn top_level(path: Option<&PathBuf>) -> Result<PathBuf, GitCommandError> {
        let output = GitCommand::git_command::<String>(
            "rev-parse",
            Some(vec!["--show-toplevel"]),
            None,
            Some(&Self::path(path.cloned())),
        )?;
        match PathBuf::from_str(output.trim_end_matches('\n')) {
            Ok(path) => Ok(path),
            Err(error) => Err(GitCommandError::new(error.to_string())),
        }
    }

    /// Ask Git for the top level folder of a path.
    /// Ask Git to fill in the username and password for the given url and return the full credentials.
    pub fn credentials_fill(url: &str) -> Result<Credentials, GitCommandError> {
        let credentials = Credentials::with_url(url);
        let output =
            GitCommand::git_command("credential", Some(vec!["fill"]), Some(credentials), None)?;
        match Credentials::from_str(output.as_str()) {
            Ok(credentials) => Ok(credentials),
            Err(_) => Err(GitCommandError::new(
                "Failed converting output to credentials".to_string(),
            )),
        }
    }

    /// Tell Git that the given credentials were accepted by an operation.
    pub fn credentials_approve(credentials: &Credentials) -> Result<(), GitCommandError> {
        let _ =
            GitCommand::git_command("credential", Some(vec!["approve"]), Some(credentials), None)?;
        Ok(())
    }

    /// Tell Git that the given credentials were rejected by an operation.
    pub fn credentials_reject(credentials: &Credentials) -> Result<(), GitCommandError> {
        let _ =
            GitCommand::git_command("credential", Some(vec!["reject"]), Some(credentials), None)?;
        Ok(())
    }
}

impl GitLib {
    fn path(path: Option<PathBuf>) -> PathBuf {
        if let Some(path) = path {
            path
        } else {
            std::env::current_dir().expect("Error getting cwd")
        }
    }
}

// All of these tests suck....
#[test]
// This is not appropriate for automated testing, because it relies on user input.
// What's more, it requires that the enters specific values.
fn credential() {
    let url = "http://example.com";
    let username = "baravelli";
    let password = "swordfish";

    // Get the username and password for the url from the user.
    // The user is expected to enter the values above.
    let credentials = GitLib::credentials_fill(url).expect("Failed to fill credentials (prompt)");

    // Store the credentials for the url in the credential manager
    GitLib::credentials_approve(&credentials).expect("Failed to approve credentials");

    // Get the credentials for the url again.
    // This time they should come from the credential manager.
    let credentials_fill =
        GitLib::credentials_fill(url).expect("Failed to fill credentials (approved)");

    // Verify that the username and password are what are expected
    assert_eq!(
        credentials_fill.username().as_ref(),
        Some(&username.to_string())
    );
    assert_eq!(
        credentials_fill.password().as_ref(),
        Some(&password.to_string())
    );

    // Remove the credentials for the url from the credential manager
    GitLib::credentials_reject(&credentials_fill).expect("Failed to reject credentials");

    // If the above worked, the credential manager will prompt the user again.
    // The user is expected to cancel the dialog.
    let credentials_fill =
        GitLib::credentials_fill(url).expect("Failed to fill credentials (rejected)");
    assert_eq!(credentials_fill.username().as_ref(), Some(&"".to_string()));
    assert_eq!(credentials_fill.password().as_ref(), Some(&"".to_string()));
}

#[test]
fn is_in_work_tree() {
    assert!(GitLib::is_inside_work_tree(None) // cwd
        .expect("Not inside work tree"));
}

#[test]
fn is_in_work_tree_cwd() {
    assert!(GitLib::is_inside_work_tree(Some(
        &std::env::current_dir().expect("Error getting cwd")
    ))
    .expect("Not inside work tree"));
}

#[test]
fn top_level() {
    // cargo test --lib top_level -- --show-output
    let cwd = std::env::current_dir().expect("Error getting cwd");
    let mut dir = cwd.clone();
    dir.push("src");
    let top_level = GitLib::top_level(Some(&dir)).expect("No top level");
    assert_eq!(cwd, top_level)
}

#[test]
fn remote_url() {
    const REMOTE_NAME: &str = "origin";
    const TEST_PATH: &str = "C:\\Users\\psobo\\Development\\rust\\ckpath";
    const REMOTE_URL: &str = "http://marconi/gitea/psobolik/ckpath-rust.git";

    let mut path = PathBuf::new();
    path.push(TEST_PATH);
    let remote_url =
        GitLib::remote_url(REMOTE_NAME, Some(&path)).expect("Error getting remove URL");
    assert_eq!(remote_url, REMOTE_URL);
}

#[test]
#[should_panic]
fn no_remote_url() {
    const REMOTE_NAME: &str = "origin";
    const TEST_PATH: &str = "C:\\Users\\psobo\\Development\\rust";

    match GitLib::remote_url(REMOTE_NAME, Some(&PathBuf::from(TEST_PATH))) {
        Ok(_) => {}
        Err(error) => panic!("{:?}", error),
    }
}

#[test]
#[should_panic]
fn not_is_in_work_tree() {
    const TEST_PATH: &str = "C:\\Users\\psobo\\Development\\rust";

    assert!(
        !GitLib::is_inside_work_tree(Some(&PathBuf::from(TEST_PATH)))
            .expect("Not inside work tree")
    );
}

#[test]
#[should_panic]
fn no_top_level() {
    const TEST_PATH: &str = "C:\\Users\\psobo\\Development\\rust";

    match GitLib::top_level(Some(&PathBuf::from(TEST_PATH))) {
        Ok(_) => {}
        Err(error) => panic!("{:?}", error),
    }
}
