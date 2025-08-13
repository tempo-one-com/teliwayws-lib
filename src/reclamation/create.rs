use crate::{auth::WsAuth, error::Result, soap_ws::WebServiceTeliwaySoap};

use super::{
    request::ReclamationCreateWsRequest,
    response::ReclamationWsResponse,
    soap::{request::ReclamationCreateSoapRequest, response::ReclamationSoapResponse},
};

#[derive(Debug, Clone)]
pub struct ReclamationCreationWs {
    auth: WsAuth,
}

impl ReclamationCreationWs {
    pub fn new(url: &str, username: &str, password: &str) -> Self {
        Self {
            auth: WsAuth::new(url, username, password),
        }
    }

    pub fn new_with_auth(auth: WsAuth) -> Self {
        Self { auth }
    }

    pub async fn send(&self, req: ReclamationCreateWsRequest) -> Result<ReclamationWsResponse> {
        let body = ReclamationCreateSoapRequest::from_request(&req);

        let response = self.send_request(body.into()).await?;
        let response = ReclamationSoapResponse::try_from(response)?;
        let response = ReclamationWsResponse::from(response);

        Ok(response)
    }
}

impl WebServiceTeliwaySoap for ReclamationCreationWs {
    fn get_auth(&self) -> WsAuth {
        self.auth.clone()
    }
}
