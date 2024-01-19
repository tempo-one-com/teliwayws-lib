use maud::{Markup, html};

use crate::{error::Result, soap_ws::WebServiceTeliwaySoap};

use super::TiersWsResponse;

#[derive(Debug, Clone)]
pub struct TiersUpdateSiretWsRequest {
   pub tiers_id: i32,
   pub tiers_type: String,
   pub siret: Option<String>,
}

pub async fn send(
    ws: WebServiceTeliwaySoap,    
    tiers: TiersUpdateSiretWsRequest,
) -> Result<TiersWsResponse> {      
   let body = tiers.build_body();

   let tiers = ws.send(body.into()).await?;

   let tiers: TiersWsResponse = tiers.try_into()?;
   
   Ok(tiers)
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
        let ws = WebServiceTeliwaySoap {
            url: "http://gtra.teliway.com/GestionTiers/gestionTiers.php".to_string(),
            username: "testusr".to_string(),
            password: "testpwd".to_string(),
        };

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