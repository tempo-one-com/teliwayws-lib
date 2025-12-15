use crate::{
    auth::WsAuth,
    error::{Error, Result},
    soap_ws::WebServiceTeliwaySoap,
};

use super::{
    request::TiersCreateOrUpdateWsRequest,
    response::TiersWsResponse,
    soap::{request::TiersCreateOrUpdateSoapRequest, response::TiersSoapResponse},
};

#[derive(Debug, Clone)]
pub struct TiersCreateOrUpdateWs {
    auth: WsAuth,
}

impl TiersCreateOrUpdateWs {
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

    pub async fn send(&self, req: TiersCreateOrUpdateWsRequest) -> Result<TiersWsResponse> {
        let body = TiersCreateOrUpdateSoapRequest::from_request(&req);

        let response = self.send_request(body.into()).await?;
        let response = TiersSoapResponse::try_from(response)?;
        let response = TiersWsResponse::from(response);

        Ok(response)
    }
}

impl WebServiceTeliwaySoap for TiersCreateOrUpdateWs {
    fn get_auth(&self) -> WsAuth {
        self.auth.clone()
    }
}

#[cfg(test)]
mod tests {

    use chrono::NaiveDate;

    use super::*;
    use crate::tiers::TiersType;

    #[test]
    fn build_envelope_create_or_update_tiers() {
        let ws = TiersCreateOrUpdateWs::new(
            "http://gtra.teliway.com/GestionTiers/gestionTiers.php",
            "testusr",
            "testpwd",
        );

        let req = TiersCreateOrUpdateWsRequest {
            tiers_id: Some(42),
            tiers_type: TiersType::Client,
            code: "701433".to_string(),
            name: "Hankook".to_string(),
            country_code: "FR".to_string(),
            address1: "5 allée du matin calme".to_string(),
            is_active: true,
            date: NaiveDate::from_ymd_opt(2025, 12, 15).unwrap_or_default(),
            vat_number: "FRVAT_123456".to_string(),
            ..Default::default()
        };

        let enveloppe =
            ws.build_envelope(TiersCreateOrUpdateSoapRequest::from_request(&req).into());
        println!("{enveloppe}");
        assert!(enveloppe.contains("<creerModifierTiers><infosCreationModificationTiers><idTiers>42</idTiers><iTypeTiers>1</iTypeTiers>"));
        assert!(enveloppe.contains("<dDateModification>2025-12-15</dDateModification>"));
        assert!(enveloppe.contains("<bActif>1</bActif>"))
    }
}
