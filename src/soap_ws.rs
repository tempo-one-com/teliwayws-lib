use log::debug;
use maud::{Markup, html};
use regex::Regex;

use crate::error::{Result, Error};

#[derive(Debug)]
pub struct WebServiceTeliwaySoap {
    pub url: String,
    pub username: String,
    pub password: String,
}

impl WebServiceTeliwaySoap {
    pub fn new(url: &str, username: &str, password: &str) -> Self {
        Self {
            url: url.to_owned(),
            username: username.to_owned(),
            password: password.to_owned(),
        }
    }

    pub fn from_ws_url(url: &str) -> Result<Self> {
        Self::parse_ws_url(url)
    }

    pub fn parse_ws_url(ws_url: &str) -> Result<Self> {
        let re = Regex::new(r"^(.*)://(.*):(.*)@(.*)$")
            .map_err(|e| Error::Parsing(e.to_string()))?;
    
        if let Some(captures) = re.captures(ws_url) {
            let http = captures.get(1).map_or(String::default(), |x| x.as_str().to_string());
            let username = captures.get(2).map_or(String::default(), |x| x.as_str().to_string());
            let password = captures.get(3).map_or(String::default(), |x| x.as_str().to_string());
            let host = captures.get(4).map_or(String::default(), |x| x.as_str().to_string());
    
            let url = format!("{http}://{host}");

            Ok(WebServiceTeliwaySoap {
                url,
                username,
                password,    
            })
        } else {
            Err(Error::Parsing(format!("url invalide: {ws_url}")))
        }
    }



    pub async fn send(&self, body: String) -> Result<String>{
        let envelope = self.build_envelope(body);

        debug!("{envelope}");

        reqwest::Client::new()
            .post(self.url.clone())
            .header("Content-Type", "application/soap+xml")            
            .body(envelope)
            .send()
            .await
            .map_err(|e| Error::ApiConnexion(e.to_string()))?
            .text()
            .await
            .map_err(|e| Error::ApiResult(e.to_string()))
        
    }

    pub fn build_envelope(&self, body: String) -> String {
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
                sLogin {(self.username)}
                sPassword {(self.password)}
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ws_url() {
        let ws = WebServiceTeliwaySoap::parse_ws_url("http://monuser:monmdp@mondomain.com/operation.php").unwrap();
        assert_eq!(ws.url, "http://mondomain.com/operation.php");
        assert_eq!(ws.username, "monuser");
        assert_eq!(ws.password, "monmdp");
    }

    #[test]
    fn parse_ws_url_ssl() {
        let ws = WebServiceTeliwaySoap::parse_ws_url("https://monuser:monmdp@mondomain.com/operation.php").unwrap();
        assert_eq!(ws.url, "https://mondomain.com/operation.php");
        assert_eq!(ws.username, "monuser");
        assert_eq!(ws.password, "monmdp");
    }

}
