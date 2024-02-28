use regex::Regex;

use crate::error::{Error, Result};

#[derive(Clone, Debug)]
pub struct WsAuth {
    pub url: String,
    pub username: String,
    pub password: String,
}

impl WsAuth {
    pub fn new(url: &str, username: &str, password: &str) -> Self {
        Self {
            url: url.to_owned(),
            username: username.to_owned(),
            password: password.to_owned(),
        }
    }
}

impl<'a> TryFrom<&'a str> for WsAuth {
    type Error = Error;

    fn try_from(ws_url: &str) -> Result<Self> {
        let re = Regex::new(r"^(.*)://(.*):(.*)@(.*)$")
            .map_err(|e| Error::Parsing(e.to_string()))?;
    
        if let Some(captures) = re.captures(ws_url) {
            let http = captures.get(1).map_or(String::default(), |x| x.as_str().to_string());
            let username = captures.get(2).map_or(String::default(), |x| x.as_str().to_string());
            let password = captures.get(3).map_or(String::default(), |x| x.as_str().to_string());
            let host = captures.get(4).map_or(String::default(), |x| x.as_str().to_string());
    
            let url = format!("{http}://{host}");

            Ok(WsAuth {
                url,
                username,
                password,    
            })
        } else {
            Err(Error::Parsing(format!("url invalide: {ws_url}")))
        }        
    }
}