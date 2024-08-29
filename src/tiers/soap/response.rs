use serde::Deserialize;

use crate::{
    error::{Error, Result},
    utils::xml::extract_xml_tag,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct TiersSoapResponse {
    #[serde(rename = "idTiers")]
    pub tiers_id: i32,

    #[serde(rename = "sCode")]
    pub code: String,

    #[serde(rename = "iTypeTiers")]
    pub tiers_type: i32,

    #[serde(rename = "sNom")]
    pub name: String,

    #[serde(rename = "sSiret")]
    pub siret_edi: String,

    #[serde(rename = "sSiretAdministratif")]
    pub siret_administratif: String,
}

impl TryFrom<String> for TiersSoapResponse {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        let tag = "resultatCreationModificationTiers";
        let xml = extract_xml_tag(tag, &value).ok_or(Error::Parsing(tag.to_string()))?;
        let data: core::result::Result<TiersSoapResponse, quick_xml::DeError> =
            quick_xml::de::from_str(xml);

        match data {
            Ok(x) => Ok(x),
            _ => Err(Error::Parsing(tag.to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extractor_simple() {
        let text = r#"<return xsi:type="ns1:CreationModificationTiersReponse">
        <resultatCreationModificationTiers xsi:type="ns1:Tiers">
            <idTiers xsi:type="xsd:int">599</idTiers>
            <sCode xsi:type="xsd:string">10001</sCode>
        </resultatCreationModificationTiers>
      </return>"#;

        let part = extract_xml_tag("sCode", text);

        assert_eq!(part, Some(r#"<sCode xsi:type="xsd:string">10001</sCode>"#))
    }

    #[test]
    fn extractor_embedded() {
        let text = r#"<return xsi:type="ns1:CreationModificationTiersReponse"><resultatCreationModificationTiers xsi:type="ns1:Tiers"><idTiers xsi:type="xsd:int">599</idTiers><sCode xsi:type="xsd:string">10001</sCode></resultatCreationModificationTiers></return>"#;

        let part = extract_xml_tag("resultatCreationModificationTiers", text);

        assert_eq!(
            part,
            Some(
                r#"<resultatCreationModificationTiers xsi:type="ns1:Tiers"><idTiers xsi:type="xsd:int">599</idTiers><sCode xsi:type="xsd:string">10001</sCode></resultatCreationModificationTiers>"#
            )
        )
    }

    #[test]
    fn into_struct() {
        let text = r#"<return xsi:type="ns1:CreationModificationTiersReponse">
        <resultatCreationModificationTiers xsi:type="ns1:Tiers">
            <idTiers xsi:type="xsd:int">599</idTiers>
            <sCode xsi:type="xsd:string">10001</sCode>
            <sNom xsi:type="xsd:string">PIRELLI</sNom>
            <iTypeTiers xsi:type="xsd:int">1</iTypeTiers>
            <sSiret xsi:type="xsd:int">123</sSiret>
            <sSiretAdministratif xsi:type="xsd:int">789</sSiretAdministratif>
        </resultatCreationModificationTiers>
      </return>"#;

        let s: core::result::Result<TiersSoapResponse, _> = text.to_owned().try_into();
        assert_eq!(s.clone().unwrap().tiers_id, 599);
        assert_eq!(s.clone().unwrap().code, "10001".to_owned());
        assert_eq!(s.clone().unwrap().name, "PIRELLI".to_owned());
        assert_eq!(s.clone().unwrap().tiers_type, 1);
        assert_eq!(s.clone().unwrap().siret_edi, "123".to_owned());
        assert_eq!(s.clone().unwrap().siret_administratif, "789".to_owned());
    }
}
