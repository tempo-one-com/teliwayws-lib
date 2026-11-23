use crate::{
    auth::WsAuth,
    error::{Error, Result},
    soap_ws::WebServiceTeliwaySoap,
};

use super::{
    request::PositionEventMarkerWsRequest,
    response::PositionEventMarkerWsResponse,
    soap::{request::PositionEventMarkerSoapRequest, response::PositionEventMarkerSoapResponse},
};

#[derive(Debug, Clone)]
pub struct PositionEventMarkerWs {
    auth: WsAuth,
}

impl PositionEventMarkerWs {
    pub fn new(url: &str, username: &str, password: &str) -> Self {
        Self {
            auth: WsAuth::new(url, username, password),
        }
    }

    pub fn new_with_auth(auth: WsAuth) -> Self {
        Self { auth }
    }

    ///url au format https://<user>:<password>@<host>
    pub fn try_new_from_url_with_access(url: &str) -> Result<Self> {
        match WsAuth::try_from(url) {
            Ok(auth) => Ok(Self::new_with_auth(auth)),
            _ => Err(Error::Parsing(format!(
                "Format Url Database invalide : {url}"
            ))),
        }
    }

    pub async fn send(
        &self,
        req: PositionEventMarkerWsRequest,
    ) -> Result<PositionEventMarkerWsResponse> {
        let body = PositionEventMarkerSoapRequest::from_request(&req);

        let response = self.send_request(body.into()).await?;
        let response = PositionEventMarkerSoapResponse::try_from(response)?;
        let response = PositionEventMarkerWsResponse::from(response);

        Ok(response)
    }
}

impl WebServiceTeliwaySoap for PositionEventMarkerWs {
    fn get_auth(&self) -> WsAuth {
        self.auth.clone()
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;

    use super::*;
    use crate::position::request::EventFile;

    #[test]
    fn build_envelope() {
        let ws = PositionEventMarkerWs::new(
            "http://gtra.teliway.com/GestionTiers/gestionTiers.php",
            "testusr",
            "testpwd",
        );

        let req = PositionEventMarkerWsRequest {
            position_ids: vec![10, 100, 1000],
            event_code: "MLVCFM".to_string(),
            datetime: DateTime::default(),
            created_by: "test".to_string(),
            agence_code: "13M".to_string(),
            ..Default::default()
        };

        let envelope = ws.build_envelope(PositionEventMarkerSoapRequest::from_request(&req).into());

        assert!(envelope.contains("<sLogin>testusr</sLogin>"));
        assert!(
            envelope.contains(
                "<dtmDateHeureEvenement>1970-01-01T01:00:00.0+0100</dtmDateHeureEvenement>"
            )
        );
        assert!(envelope.contains("<sCodeEvenement>MLVCFM</sCodeEvenement>"));
        assert!(envelope.contains("<pointagePositionDemande><tabIdPosition><item>10</item><item>100</item><item>1000</item></tabIdPosition>"));
    }

    #[test]
    fn build_envelope_rdv() {
        let ws = PositionEventMarkerWs::new(
            "http://gtra.teliway.com/GestionTiers/gestionTiers.php",
            "testusr",
            "testpwd",
        );

        let req = PositionEventMarkerWsRequest {
            position_ids: vec![10, 100, 1000],
            event_code: "MLVCFM".to_string(),
            datetime: DateTime::default(),
            created_by: "test".to_string(),
            agence_code: "13M".to_string(),
            date_rdv: Some(DateTime::default()),
            ..Default::default()
        };

        let envelope = ws.build_envelope(PositionEventMarkerSoapRequest::from_request(&req).into());

        assert!(envelope.contains("<sLogin>testusr</sLogin>"));
        assert!(
            envelope.contains(
                "<dtmDateHeureEvenement>1970-01-01T01:00:00.0+0100</dtmDateHeureEvenement>"
            )
        );
        assert!(envelope.contains("<dtmDateHeureRDV>1970-01-01T01:00:00.0+0100</dtmDateHeureRDV>"));
        assert!(envelope.contains("<sCodeEvenement>MLVCFM</sCodeEvenement>"));
        assert!(envelope.contains("<pointagePositionDemande><tabIdPosition><item>10</item><item>100</item><item>1000</item></tabIdPosition>"));
    }

    #[test]
    fn build_envelope_files() {
        let req = PositionEventMarkerWsRequest {
            position_ids: vec![10],
            event_code: "PODCFM".to_string(),
            files: vec![
                EventFile {
                    name: "pod-1.pdf".to_string(),
                    mime_type: "application/pdf".to_string(),
                    content: b"%PDF".to_vec(),
                },
                EventFile {
                    name: "pod-2.jpg".to_string(),
                    mime_type: "image/jpeg".to_string(),
                    content: vec![0xFF, 0xD8],
                },
            ],
            ..Default::default()
        };

        let body = PositionEventMarkerSoapRequest::from_request(&req).into_string();

        assert!(body.contains(
            "<tabOFichier><item><sNom>pod-1.pdf</sNom><sType>application/pdf</sType><sData>JVBERg==</sData></item><item><sNom>pod-2.jpg</sNom><sType>image/jpeg</sType><sData>/9g=</sData></item></tabOFichier>"
        ));
    }

    #[test]
    fn build_envelope_without_files() {
        let body = PositionEventMarkerSoapRequest::from_request(&Default::default()).into_string();

        assert!(!body.contains("tabOFichier"));
    }
}
