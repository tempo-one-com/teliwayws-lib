use crate::{auth::WsAuth, error::Result, soap_ws::WebServiceTeliwaySoap};

use super::{
    request::TiersUpdateSiretWsRequest,
    response::TiersWsResponse,
    soap::{request::TiersUpdateSiretSoapRequest, response::TiersSoapResponse},
};

#[derive(Debug, Clone)]
pub struct TiersUpdateSiretWs {
    auth: WsAuth,
}

impl TiersUpdateSiretWs {
    pub fn new(url: &str, username: &str, password: &str) -> Self {
        Self {
            auth: WsAuth::new(url, username, password),
        }
    }

    pub fn new_with_auth(auth: WsAuth) -> Self {
        Self { auth }
    }

    pub async fn send(&self, req: TiersUpdateSiretWsRequest) -> Result<TiersWsResponse> {
        let body = TiersUpdateSiretSoapRequest::from_request(&req);

        let response = self.send_request(body.into()).await?;
        let response = TiersSoapResponse::try_from(response)?;
        let response = TiersWsResponse::from(response);

        Ok(response)
    }
}

impl WebServiceTeliwaySoap for TiersUpdateSiretWs {
    fn get_auth(&self) -> WsAuth {
        self.auth.clone()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn build_envelope() {
        let ws = TiersUpdateSiretWs::new(
            "http://gtra.teliway.com/GestionTiers/gestionTiers.php",
            "testusr",
            "testpwd",
        );

        let req = TiersUpdateSiretWsRequest {
            tiers_id: 42,
            tiers_type: "1".to_string(),
            siret: Some("123".to_string()),
        };

        let enveloppe = ws.build_envelope(TiersUpdateSiretSoapRequest::from_request(&req).into());

        assert!(enveloppe.contains("<sLogin>testusr</sLogin>"));
        assert!(enveloppe.contains("<creerModifierTiers><infosCreationModificationTiers><idTiers>42</idTiers><iTypeTiers>1</iTypeTiers><sSiret>123</sSiret></infosCreationModificationTiers></creerModifierTiers>"));
    }
}
