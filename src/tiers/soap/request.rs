use maud::{html, Markup};

use crate::tiers::request::TiersUpdateSiretWsRequest;

pub struct TiersUpdateSiretSoapRequest;

impl TiersUpdateSiretSoapRequest {
    pub fn from_request(req: &TiersUpdateSiretWsRequest) -> Markup {
        let siret = req.siret.clone().unwrap_or_default();

        html!(
            creerModifierTiers {
                infosCreationModificationTiers {
                    idTiers { (req.tiers_id) }
                    iTypeTiers { (req.tiers_type) }
                    sSiret { (siret) }
                }
            }
        )
    }
}
