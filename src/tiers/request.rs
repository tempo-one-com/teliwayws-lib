#[derive(Debug, Clone)]
pub struct TiersUpdateSiretWsRequest {
    pub tiers_id: i32,
    pub tiers_type: String,
    pub siret: Option<String>,
}
