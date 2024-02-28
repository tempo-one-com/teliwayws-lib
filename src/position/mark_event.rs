use chrono::{DateTime, Local};
use maud::{html, Markup};

use crate::{auth::WsAuth, error::Result, format_to_teliway_ws_datetimez, soap_ws::WebServiceTeliwaySoap};

use super::PositionEventMarkerWsResponse;

#[derive(Debug, Clone)]
pub struct PositionEventMarkerWsRequest {
    pub position_ids: Vec<i32>,
    pub event_code: String,
    pub date: DateTime<Local>,
    pub created_by: String,
    pub agence_code: String,
}

#[derive(Debug, Clone)]
pub struct PositionEventMarkerWs {
    wsauth: WsAuth,
}

impl PositionEventMarkerWs {
    pub fn new(url: &str, username: &str, password: &str) -> Self {
        Self {
            wsauth: WsAuth::new(url, username, password),
        }
    }

    pub async fn send(
        &self,
        req: PositionEventMarkerWsRequest,
    ) -> Result<PositionEventMarkerWsResponse> {
        let body = req.build_body();    
        
        self
            .send_request(body.into())
            .await?
            .try_into()
    }    
}

impl WebServiceTeliwaySoap for PositionEventMarkerWs {
    fn get_auth(&self) -> WsAuth {
        self.wsauth.clone()
    }
}

impl PositionEventMarkerWsRequest {
    fn build_body(&self) -> Markup {
        let date = format_to_teliway_ws_datetimez(self.date);
        
        html!(
            pointerPosition {
                pointagePositionDemande {
                    tabIdPosition {
                    @for id in &self.position_ids {
                        item {(id)}
                    }}
                    sCodeEvenement { (self.event_code) }
                    sCreateur {(self.created_by)}
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
        let ws = PositionEventMarkerWs::new(
            "http://gtra.teliway.com/GestionTiers/gestionTiers.php", 
            "testusr",
            "testpwd",
        );
            
        let req = PositionEventMarkerWsRequest {
            position_ids: vec![10,100, 1000],
            event_code: "MLVCFM".to_string(),
            date: DateTime::default(),
            created_by: "test".to_string(),
            agence_code: "13M".to_string(),
        };

        let envelope = ws.build_envelope(req.build_body().into());

        assert!(envelope.contains("<sLogin>testusr</sLogin>"));        
        assert!(envelope.contains("<dtmDateHeureEvenement>1970-01-01T01:00:00.0+0100</dtmDateHeureEvenement>"));
        assert!(envelope.contains("<sCodeEvenement>MLVCFM</sCodeEvenement>"));
        assert!(envelope.contains("<pointagePositionDemande><tabIdPosition><item>10</item><item>100</item><item>1000</item></tabIdPosition>"));
    }
}