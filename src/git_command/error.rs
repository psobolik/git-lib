/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-02-09
 */

use std::fmt;

pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: String) -> Error {
        Error { message }
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message) // user-facing output
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {{ file: {}, line: {} }}",
            self.message,
            file!(),
            line!()
        ) // programmer-facing output
    }
}
