use maud::{Markup, html};

use crate::{auth::WsAuth, error::Result, soap_ws::WebServiceTeliwaySoap};

use super::TiersWsResponse;

#[derive(Debug, Clone)]
pub struct TiersUpdateSiretWsRequest {
   pub tiers_id: i32,
   pub tiers_type: String,
   pub siret: Option<String>,
}

pub struct TiersUpdateSiretWs {
    wsauth: WsAuth,
}

impl TiersUpdateSiretWs {
    pub fn new(url: &str, username: &str, password: &str) -> Self {
        Self {
            wsauth: WsAuth::new(url, username, password),
        }
    }

    pub async fn send(
        &self,   
        tiers: TiersUpdateSiretWsRequest,
    ) -> Result<TiersWsResponse> {      
       let body = tiers.build_body();    
       let tiers = self.send_request(body.into()).await?;
    
       let tiers: TiersWsResponse = tiers.try_into()?;
       
       Ok(tiers)
    }    
}

impl WebServiceTeliwaySoap for TiersUpdateSiretWs {
    fn get_auth(&self) -> WsAuth {
        self.wsauth.clone()
    }
}

impl TiersUpdateSiretWsRequest {
    fn build_body(&self) -> Markup {
    let siret = self.siret.clone().unwrap_or_default();

    html!(
        creerModifierTiers {
            infosCreationModificationTiers {
                idTiers { (self.tiers_id) }
                iTypeTiers { (self.tiers_type) }
                sSiret { (siret) }
            }
        }
    )
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
            siret: Some("123".to_string())
        };

        let enveloppe = ws.build_envelope(req.build_body().into());

        assert!(enveloppe.contains("<sLogin>testusr</sLogin>"));
        assert!(enveloppe.contains("<creerModifierTiers><infosCreationModificationTiers><idTiers>42</idTiers><iTypeTiers>1</iTypeTiers><sSiret>123</sSiret></infosCreationModificationTiers></creerModifierTiers>"));
    }
}