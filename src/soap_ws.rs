use async_trait::async_trait;
use log::debug;
use maud::{Markup, html};

use crate::{auth::WsAuth, error::{Error, Result}};

#[async_trait]
pub trait WebServiceTeliwaySoap {
    fn get_auth(&self) -> WsAuth;

    async fn send_request(&self, body: String) -> Result<String>{
        let envelope = self.build_envelope(body);

        debug!("{envelope}");

        reqwest::Client::new()
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
        let header = self.build_header().into_string();

        format!(r#"<SOAP:Envelope xmlns:SOAP="http://schemas.xmlsoap.org/soap/envelope/">
            <SOAP:Header>{header}</SOAP:Header>
                <SOAP:Body>{body}</SOAP:Body>
            </SOAP:Envelope>"#
        )
    }

    fn build_header(&self) -> Markup {
        html!(
            AuthentificationHeader {
                sLogin {(self.get_auth().username)}
                sPassword {(self.get_auth().password)}
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ws_url() {
        let ws = WsAuth::try_from("http://monuser:monmdp@mondomain.com/operation.php").unwrap();// parse_ws_url("http://monuser:monmdp@mondomain.com/operation.php").unwrap();
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
