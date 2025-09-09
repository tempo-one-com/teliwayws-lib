use crate::{
    position::request::PositionEventMarkerWsRequest, utils::date::format_to_teliway_ws_datetimez,
};
use maud::{html, Markup};

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
                }
            }
        )
    }
}
