use super::soap::response::TiersSoapResponse;

pub struct TiersWsResponse {
    pub tiers_id: i32,
    pub code: String,
    pub tiers_type: i32,
    pub name: String,
    pub siret_edi: String,
    pub siret_administratif: String,
}

impl From<TiersSoapResponse> for TiersWsResponse {
    fn from(value: TiersSoapResponse) -> Self {
        Self {
            tiers_id: value.tiers_id,
            code: value.code,
            tiers_type: value.tiers_id,
            name: value.name,
            siret_edi: value.siret_edi,
            siret_administratif: value.siret_administratif,
        }
    }
}
