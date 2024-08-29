use crate::reclamation::request::ReclamationCreateWsRequest;
use maud::{html, Markup};

pub struct ReclamationCreateSoapRequest;

impl ReclamationCreateSoapRequest {
    pub fn from_request(req: &ReclamationCreateWsRequest) -> Markup {
        html!(
            creerModifierReclamation {
                infosCreationModificationReclamation {
                    idPosition {(req.position_id)}
                    iNumero {(req.number)}
                    sTypeReclamation {(req.r#type)}
                    iOrigin {"1"}
                    iStatut {"0"}
                    sNomContact {(req.author)}
                    sMotifCreation {(req.motif)}
                    sLoginOuverture {"extranet"}
                }
            }
        )
    }
}
