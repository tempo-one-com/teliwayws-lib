use chrono::{DateTime, Local};
use maud::{html, Markup};

use crate::{error::Result, soap_ws::WebServiceTeliwaySoap, format_to_teliway_ws_datetimez};

use super::PositionEventMarkerWsResponse;

pub struct PositionEventMarkerWsRequest {
    pub position_ids: Vec<i32>,
    pub event_code: String,
    pub date: DateTime<Local>,
    pub created_by: String,
    pub agence_code: String,
}

pub async fn send(
    req: PositionEventMarkerWsRequest,
    app_code: &str,
    ws: &WebServiceTeliwaySoap,
) -> Result<PositionEventMarkerWsResponse> {
    let body = req.build_body(app_code);

    let position_events = ws.send(body.into()).await?;

    let position_events: PositionEventMarkerWsResponse = position_events.try_into()?;

    Ok(position_events)
}

impl PositionEventMarkerWsRequest {
    fn build_body(&self, app_code: &str) -> Markup {
        let date = format_to_teliway_ws_datetimez(self.date);
        
        html!(
            pointerPosition {
                pointagePositionDemande {
                    tabIdPosition {
                    @for id in &self.position_ids {
                        item {(id)}
                    }}
                    sCodeEvenement { (self.event_code) }
                    sCreateur {(app_code)}
                    iOrigine {"1"}
                    dtmDateHeureEvenement { (date) }
                    sCodeTiersEmetteur { (self.agence_code) }
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
            
        let req = PositionEventMarkerWsRequest {
            position_ids: vec![10,100, 1000],
            event_code: "MLVCFM".to_string(),
            date: DateTime::default(),
            created_by: "test".to_string(),
            agence_code: "13M".to_string(),
        };

        let envelope = ws.build_envelope(req.build_body("appTest").into());

        assert!(envelope.contains("<sLogin>testusr</sLogin>"));        
        assert!(envelope.contains("<dtmDateHeureEvenement>1970-01-01T01:00:00.0+0100</dtmDateHeureEvenement>"));
        assert!(envelope.contains("<sCodeEvenement>MLVCFM</sCodeEvenement>"));
        assert!(envelope.contains("<pointagePositionDemande><tabIdPosition><item>10</item><item>100</item><item>1000</item></tabIdPosition>"));
    }
}