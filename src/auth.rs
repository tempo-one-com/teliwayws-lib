use std::fmt;

use regex::Regex;

use crate::error::{Error, Result};

#[derive(Clone)]
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

/// Mot de passe masqué : un `{:?}` ne doit pas l'afficher dans les logs
impl fmt::Debug for WsAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WsAuth")
            .field("url", &self.url)
            .field("username", &self.username)
            .field("password", &"***")
            .finish()
    }
}

impl TryFrom<&str> for WsAuth {
    type Error = Error;

    fn try_from(ws_url: &str) -> Result<Self> {
        let re =
            Regex::new(r"^(.*)://(.*):(.*)@(.*)$").map_err(|e| Error::Parsing(e.to_string()))?;

        if let Some(captures) = re.captures(ws_url) {
            let http = captures
                .get(1)
                .map_or(String::default(), |x| x.as_str().to_string());
            let username = captures
                .get(2)
                .map_or(String::default(), |x| x.as_str().to_string());
            let password = captures
                .get(3)
                .map_or(String::default(), |x| x.as_str().to_string());
            let host = captures
                .get(4)
                .map_or(String::default(), |x| x.as_str().to_string());

            let url = format!("{http}://{host}");

            Ok(WsAuth {
                url,
                username,
                password,
            })
        } else {
            // l'url porte les identifiants : elle n'est pas reprise dans le message
            Err(Error::Parsing(
                "url invalide, format attendu : https://<user>:<password>@<host>".to_owned(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::auth::WsAuth;

    #[test]
    fn new_url_access() {
        let ws =
            WsAuth::try_from("http://username:password@domaine.com/GestionTiers/gestionTiers.php");
        assert!(ws.is_ok());
        assert_eq!(
            ws.clone().unwrap().url,
            "http://domaine.com/GestionTiers/gestionTiers.php"
        );
        assert_eq!(ws.clone().unwrap().username, "username");
        assert_eq!(ws.clone().unwrap().password, "password");
    }

    #[test]
    fn debug_masks_password() {
        let ws = WsAuth::new("http://domaine.com", "username", "secret");
        let text = format!("{ws:?}");
        assert!(text.contains("username"));
        assert!(!text.contains("secret"));
    }
}
