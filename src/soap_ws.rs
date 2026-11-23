use std::time::Duration;

use async_trait::async_trait;
use log::{Level, debug, log_enabled};
use maud::{Markup, html};

use crate::{
    auth::WsAuth,
    error::{Error, Result},
};

/// Délai maximal d'un appel : un web service bloqué ne doit pas figer l'appelant
const TIMEOUT: Duration = Duration::from_secs(10);

/// Remplace le mot de passe dans les logs
const MASKED_PASSWORD: &str = "***";

#[async_trait]
pub trait WebServiceTeliwaySoap {
    fn get_auth(&self) -> WsAuth;

    async fn send_request(&self, body: String) -> Result<String> {
        if log_enabled!(Level::Debug) {
            let auth = self.get_auth();
            let header = auth_header(&auth.username, MASKED_PASSWORD).into_string();
            debug!("{}", envelope(&header, &body));
        }

        let envelope = self.build_envelope(body);

        reqwest::Client::builder()
            .timeout(TIMEOUT)
            .build()
            .map_err(|e| Error::ApiConnexion(e.to_string()))?
            .post(self.get_auth().url.clone())
            .header("Content-Type", "application/soap+xml")
            .body(envelope)
            .send()
            .await
            .map_err(|e| Error::ApiConnexion(e.to_string()))?
            .text()
            .await
            .map_err(|e| Error::ApiResult(e.to_string()))
    }

    fn build_envelope(&self, body: String) -> String {
        envelope(&self.build_header().into_string(), &body)
    }

    fn build_header(&self) -> Markup {
        let auth = self.get_auth();
        auth_header(&auth.username, &auth.password)
    }
}

fn envelope(header: &str, body: &str) -> String {
    format!(
        r#"<SOAP:Envelope xmlns:SOAP="http://schemas.xmlsoap.org/soap/envelope/">
            <SOAP:Header>{header}</SOAP:Header>
                <SOAP:Body>{body}</SOAP:Body>
            </SOAP:Envelope>"#
    )
}

fn auth_header(username: &str, password: &str) -> Markup {
    html!(
        AuthentificationHeader {
            sLogin {(username)}
            sPassword {(password)}
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Ws;

    #[async_trait]
    impl WebServiceTeliwaySoap for Ws {
        fn get_auth(&self) -> WsAuth {
            WsAuth::new("http://mondomain.com/operation.php", "monuser", "monmdp")
        }
    }

    #[test]
    fn envelope_carries_password_but_log_masks_it() {
        assert!(
            Ws.build_envelope("<op/>".to_owned())
                .contains("<sPassword>monmdp</sPassword>")
        );
        let logged = envelope(
            &auth_header("monuser", MASKED_PASSWORD).into_string(),
            "<op/>",
        );
        assert!(logged.contains("<sLogin>monuser</sLogin><sPassword>***</sPassword>"));
        assert!(!logged.contains("monmdp"));
    }

    #[test]
    fn parse_ws_url() {
        let ws = WsAuth::try_from("http://monuser:monmdp@mondomain.com/operation.php").unwrap(); // parse_ws_url("http://monuser:monmdp@mondomain.com/operation.php").unwrap();
        assert_eq!(ws.url, "http://mondomain.com/operation.php");
        assert_eq!(ws.username, "monuser");
        assert_eq!(ws.password, "monmdp");
    }

    #[test]
    fn parse_ws_url_ssl() {
        let ws = WsAuth::try_from("https://monuser:monmdp@mondomain.com/operation.php").unwrap();
        assert_eq!(ws.url, "https://mondomain.com/operation.php");
        assert_eq!(ws.username, "monuser");
        assert_eq!(ws.password, "monmdp");
    }
}
