use base64::{Engine, engine::general_purpose::STANDARD};
use maud::{Markup, html};

use crate::{
    position::request::PositionEventMarkerWsRequest, utils::date::format_to_teliway_ws_datetimez,
};

pub struct PositionEventMarkerSoapRequest;

impl PositionEventMarkerSoapRequest {
    pub fn from_request(req: &PositionEventMarkerWsRequest) -> Markup {
        let date = format_to_teliway_ws_datetimez(req.datetime);

        html!(
            pointerPosition {
                pointagePositionDemande {
                    tabIdPosition {
                    @for id in &req.position_ids {
                        item {(id)}
                    }}
                    sCodeEvenement { (req.event_code) }
                    sCreateur {(req.created_by)}
                    iOrigine {"1"}
                    dtmDateHeureEvenement { (date) }
                    sCodeTiersEmetteur { (req.agence_code) }
                    @if let Some(date) = req.date_rdv {
                        dtmDateHeureRDV { (format_to_teliway_ws_datetimez(date)) }
                    }
                    @if let Some(info) = req.info_palette_rendu.as_deref() {
                        sInfoPalettesRendues { (info) }
                    }
                    @if let Some(remarque) = req.comment.as_deref() {
                        sRemarques {(remarque)}
                    }
                    @if !req.files.is_empty() {
                        tabOFichier {
                        @for file in &req.files {
                            item {
                                sNom { (file.name) }
                                sType { (file.mime_type) }
                                sData { (STANDARD.encode(&file.content)) }
                            }
                        }}
                    }
                }
            }
        )
    }
}
