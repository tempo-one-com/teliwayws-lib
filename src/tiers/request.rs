use chrono::NaiveDate;

#[derive(Debug, Clone, Default, PartialEq)]
#[repr(u8)]
pub enum TiersType {
    #[default]
    Client = 1,
    Carrier = 5,
    Agency = 4,
}

impl TryFrom<u8> for TiersType {
    type Error = String; // Ou un type d'erreur plus descriptif, ex. String

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(TiersType::Client),
            5 => Ok(TiersType::Carrier),
            4 => Ok(TiersType::Agency),
            _ => Err(format!("TiersType valeur {value} inconnue")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TiersUpdateSiretWsRequest {
    pub tiers_id: i32,
    pub tiers_type: String,
    pub siret: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct TiersCreateOrUpdateWsRequest {
    pub tiers_id: Option<i32>,
    pub tiers_type: TiersType,
    pub code: String,
    pub name: String,
    pub group_code: String,
    pub country_code: String,
    pub address1: String,
    pub address2: String,
    pub address3: String,
    pub zipcode: String,
    pub town: String,
    pub is_active: bool,
    pub is_risky: bool,
    pub sub_type: i32,
    pub date: NaiveDate,
    pub document_name: Option<String>,
    pub siret_administrative: Option<String>,
    pub code_naf: Option<String>,
    pub vat_number: String,
    pub vat_nature: i32,
    pub auxilary_account: String,
    pub supplier_auxiliary_account: String,
    pub phone: String,
    pub fax: String,
    pub email: String,
    pub email_invoice: String,
    pub packaging_tracking: bool,
    pub packing_loss_management: bool, //non rendu
    pub packaging_loss_rate: Option<f32>,
    pub note: String,
    pub sales_agency_id: Option<i32>,
    pub sales_contact: i32,
    pub client_contact: i32,
    pub tracking_code: i32,
    pub tracking_level: i32,
    pub trust_level: i32,
}
