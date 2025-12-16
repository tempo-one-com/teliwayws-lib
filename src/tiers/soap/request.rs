use chrono::Local;
use maud::{Markup, html};

use crate::tiers::{
    TiersType,
    request::{TiersCreateOrUpdateWsRequest, TiersUpdateSiretWsRequest},
};
use crate::utils::date::format_to_teliway_ws_date;
use crate::utils::xml::format_bool_to_int;

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
        let date = tiers
            .date
            .map(format_to_teliway_ws_date)
            .unwrap_or(format_to_teliway_ws_date(Local::now().date_naive()));

        html!(
            creerModifierTiers {
                infosCreationModificationTiers {
                    @if let Some(x) = tiers.tiers_id {
                        idTiers {(x)}
                    }
                    iTypeTiers {(tiers.tiers_type.clone() as u8)}
                    sCode {(tiers.code)}
                    sNom {(tiers.name)}
                    @if let Some(x) = &tiers.group_code {
                        sCodeGroupe {(x)}
                    }
                    sCodePays {(tiers.country_code)}
                    sAdresse1 {(tiers.address1)}
                    @if let Some(x) = &tiers.address2 {
                        sAdresse2 {(x)}
                    }
                    @if let Some(x) = &tiers.address3 {
                        sAdresse3 {(x)}
                    }
                    sCodePostal {(tiers.zipcode)}
                    sVille {(tiers.town)}
                    bActif {(tiers.is_active.map(format_bool_to_int).unwrap_or(1))}
                    bSensible {(tiers.is_risky.map(format_bool_to_int).unwrap_or(0))}
                    iSousType {(tiers.sub_type.unwrap_or(0))}
                    @match tiers.tiers_id {
                        Some(_) => dDateModification {(date)}
                        None => dDateCreation {(date)}
                    }
                    sNomPourDocument {};
                    @if let Some(x) = &tiers.siret_administrative {
                        sSiretAdministratif {(x)}
                    }
                    @if let Some(x) = &tiers.code_naf {
                        sCodeNAF {(x)}
                    }
                    sNumeroTVA {(tiers.vat_number)}
                    iNatureTVA {(tiers.vat_nature)}
                    sComptableAuxiliaire {(tiers.auxilary_account)}
                    @if let Some(x) = &tiers.supplier_auxiliary_account && tiers.tiers_type == TiersType::Carrier {
                        sCompteComptableAuxiliaireFournisseur {(x)}
                    }
                    @if let Some(x) = &tiers.phone {
                        sTelephone {(x)}
                    }
                    @if let Some(x) = &tiers.fax {
                        sFax {(x)}
                    }
                    @if let Some(x) = &tiers.email {
                        sEmail {(x)}
                    }
                    @if let Some(x) = &tiers.email_invoice {
                        sEmailFacture {(x)}
                    }
                    iFlagInfos {(0)}
                    @if let Some(x) = tiers.packaging_tracking {
                        bSuiviEmballage {(format_bool_to_int(x))}
                    }
                    @if let Some(x) = tiers.packing_loss_management {
                        bEmballageGestionNonRendu {(format_bool_to_int(x))}
                    }
                    fEmballageTauxFreinte {(tiers.packaging_loss_rate.unwrap_or(0.0))}
                    @if let Some(x) = &tiers.note {
                        sCommentaire {(x)}
                    }
                    @if let Some(x) = tiers.sales_contact {
                        idContactSuiviComm {(x)}
                    }
                    idAgenceCommerciale {(tiers.sales_agency_id.unwrap_or(0))}
                    @if let Some(x) = tiers.client_contact {
                        idContactSuiviClient {(x)}
                    }
                    @if let Some(x) = tiers.tracking_code {
                        sCodeTracking {(x)};
                    }
                    iNiveauSuivi {(1)}
                    @if let Some(x) = tiers.trust_level {
                        iEtatConfiance {(x)}
                    }
                    @match tiers.tiers_id {
                        Some(_) => sNomUtilisateurModif {"AppTiers"}
                        None => sNomUtilisateurCreateur {"AppTiers"}
                    }
                }
            }
        )
    }
}
