use maud::{Markup, html};

use crate::tiers::request::{TiersCreateOrUpdateWsRequest, TiersType, TiersUpdateSiretWsRequest};
use crate::utils;

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

pub struct TiersCreateOrUpdateSoapRequest;

impl TiersCreateOrUpdateSoapRequest {
    pub fn from_request(tiers: &TiersCreateOrUpdateWsRequest) -> Markup {
        let date = utils::date::format_to_teliway_ws_date(tiers.date);

        html!(
            creerModifierTiers {
                infosCreationModificationTiers {
                    @if let Some(id) = tiers.tiers_id {
                        idTiers {(id)}
                    }
                    iTypeTiers {(tiers.tiers_type.clone() as u8)}
                    sCode {(tiers.code)}
                    sNom {(tiers.name)}
                    sCodeGroupe {(tiers.group_code)}
                    sCodePays {(tiers.country_code)}
                    sAdresse1 {(tiers.address1)}
                    sAdresse2 {(tiers.address2)}
                    sAdresse3 {(tiers.address3)}
                    sCodePostal {(tiers.zipcode)}
                    sVille {(tiers.town)}
                    bActif {(utils::xml::format_bool_to_int(tiers.is_active))}
                    bSensible {(tiers.is_risky)}
                    iSousType {(tiers.sub_type)}
                    @match tiers.tiers_id {
                        Some(_) => dDateModification {(date)}
                        None => dDateCreation {(date)}
                    }
                    sNomPourDocument {};
                    sSiretAdministratif {(tiers.siret_administrative.clone().unwrap_or_default())}
                    sCodeNAF {(tiers.code_naf.clone().unwrap_or_default())}
                    sNumeroTVA {(tiers.vat_number)}
                    iNatureTVA {(tiers.vat_nature)}
                    sComptableAuxiliaire {(tiers.auxilary_account)}
                    @if tiers.tiers_type == TiersType::Carrier {
                        sCompteComptableAuxiliaireFournisseur {(tiers.supplier_auxiliary_account)}
                    }
                    sTelephone {(tiers.phone)}
                    sFax {(tiers.fax)}
                    sEmail {(tiers.email)}
                    sEmailFacture {(tiers.email_invoice)}
                    iFlagInfos {(0)}
                    bSuiviEmballage {(utils::xml::format_bool_to_int(tiers.packaging_tracking))}
                    bEmballageGestionNonRendu {(utils::xml::format_bool_to_int(tiers.packing_loss_management))}
                    fEmballageTauxFreinte {(tiers.packaging_loss_rate.unwrap_or(0.0))}
                    sCommentaire {(tiers.note)}
                    idAgenceCommerciale {(tiers.sales_agency_id.unwrap_or(0))}
                    idContactSuiviComm {(tiers.sales_contact)}
                    idContactSuiviClient {(0)}
                    sCodeTracking {};
                    iNiveauSuivi {(1)}
                    iEtatConfiance {(tiers.trust_level)}
                    @match tiers.tiers_id {
                        Some(_) => sNomUtilisateurModif {"AppTiers"}
                        None => sNomUtilisateurCreateur {"AppTiers"}
                    }
                }
            }
        )
    }
}
