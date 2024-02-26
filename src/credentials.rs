/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-02-09
 */

use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub struct Credentials {
    protocol: Option<String>,
    host: Option<String>,
    path: Option<String>,
    username: Option<String>,
    password: Option<String>,
    password_expiry_utc: Option<String>,
    oauth_refresh_token: Option<String>,
    url: Option<String>,
    wwwauth: Option<Vec<String>>,
}

impl Credentials {
    pub fn with_url<T: ToString>(url: T) -> Credentials {
        Credentials {
            url: Some(url.to_string()),
            username: None,
            password: None,
            protocol: None,
            host: None,
            path: None,
            password_expiry_utc: None,
            oauth_refresh_token: None,
            wwwauth: None,
        }
    }

    pub fn with_url_username_password<T: ToString>(
        url: T,
        username: T,
        password: T,
    ) -> Credentials {
        Credentials {
            url: Some(url.to_string()),
            username: Some(username.to_string()),
            password: Some(password.to_string()),
            protocol: None,
            host: None,
            path: None,
            password_expiry_utc: None,
            oauth_refresh_token: None,
            wwwauth: None,
        }
    }

    pub fn protocol(&self) -> &Option<String> {
        &self.protocol
    }
    pub fn host(&self) -> &Option<String> {
        &self.host
    }
    pub fn path(&self) -> &Option<String> {
        &self.path
    }
    pub fn username(&self) -> &Option<String> {
        &self.username
    }
    pub fn password(&self) -> &Option<String> {
        &self.password
    }
    pub fn password_expiry_utc(&self) -> &Option<String> {
        &self.password_expiry_utc
    }
    pub fn oauth_refresh_token(&self) -> &Option<String> {
        &self.oauth_refresh_token
    }
    pub fn url(&self) -> &Option<String> {
        &self.url
    }
    pub fn wwwauth(&self) -> &Option<Vec<String>> {
        &self.wwwauth
    }
}

impl FromStr for Credentials {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut protocol: Option<String> = None;
        let mut host: Option<String> = None;
        let mut path: Option<String> = None;
        let mut username: Option<String> = None;
        let mut password: Option<String> = None;
        let mut password_expiry_utc: Option<String> = None;
        let mut oauth_refresh_token: Option<String> = None;
        let mut url: Option<String> = None;
        // let mut wwwauth: Option<Vec<String>> = None;

        let lines = s.split('\n');
        for line in lines {
            let mut words = line.split('=');
            if let Some(key) = words.next() {
                if let Some(value) = words.next() {
                    match key {
                        "protocol" => {
                            protocol = Some(value.to_string());
                        }
                        "host" => {
                            host = Some(value.to_string());
                        }
                        "path" => {
                            path = Some(value.to_string());
                        }
                        "password" => {
                            password = Some(value.to_string());
                        }
                        "username" => {
                            username = Some(value.to_string());
                        }
                        "password_expiry_utc" => {
                            password_expiry_utc = Some(value.to_string());
                        }
                        "oauth_refresh_token" => {
                            oauth_refresh_token = Some(value.to_string());
                        }
                        "url" => {
                            url = Some(value.to_string());
                        }
                        _ => { /* do nothing? */ }
                    }
                }
            }
        }

        Ok(Credentials {
            url,
            protocol,
            host,
            path,
            username,
            password,
            password_expiry_utc,
            oauth_refresh_token,
            wwwauth: None,
        })
    }
}

impl Display for Credentials {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn maybe_writeln(
            f: &mut Formatter<'_>,
            key: &str,
            value: &Option<String>,
        ) -> std::fmt::Result {
            if let Some(value) = value {
                writeln!(f, "{key}={value}")?
            }
            Ok(())
        }
        maybe_writeln(f, "protocol", self.protocol())?;
        maybe_writeln(f, "host", self.host())?;
        maybe_writeln(f, "path", self.path())?;
        maybe_writeln(f, "username", self.username())?;
        maybe_writeln(f, "password", self.password())?;
        maybe_writeln(f, "password_expiry_utc()", self.password_expiry_utc())?;
        maybe_writeln(f, "oauth_refresh_token()", self.oauth_refresh_token())?;
        maybe_writeln(f, "url", self.url())?;
        Ok(())
    }
}
