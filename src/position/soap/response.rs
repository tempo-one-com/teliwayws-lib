use serde::Deserialize;

use crate::{
    error::{Error, Result},
    utils::xml::extract_xml_tag,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct PositionEventMarkerSoapResponse {
    pub items: Vec<PositionTag>,
}

impl TryFrom<String> for PositionEventMarkerSoapResponse {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        let tag = "tabResultatsPointagePosition";
        let xml = extract_xml_tag(tag, &value).ok_or(Error::Parsing(tag.to_string()))?;

        let data: core::result::Result<PositionTagList, quick_xml::DeError> =
            quick_xml::de::from_str(xml);

        match data {
            Ok(x) => Ok(Self { items: x.items }),
            _ => Err(Error::Parsing(tag.to_owned())),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct PositionEventRoot {
    #[serde(rename = "tabResultatsPointagePosition")]
    pub root_list: PositionTagList,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct PositionTagList {
    #[serde(rename = "item")]
    pub items: Vec<PositionTag>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct PositionTag {
    #[serde(rename = "sLibelle")]
    pub label: String,

    #[serde(rename = "oObjetCible")]
    pub event: EventTag,
}

impl PositionTag {
    pub fn get_recepisse(&self) -> Result<String> {
        //<sLibelle xsi:type="xsd:string">Pointage de la position  récép. 87147  du 03/01/2024  avec le code EXPCFM</sLibelle>
        let dot = self
            .label
            .find('.')
            .ok_or(Error::MissingParam(".".to_string()))?;
        let du = self
            .label
            .find("du")
            .ok_or(Error::MissingParam("du".to_string()))?;

        let recepisse = self.label[dot + 1..du].trim();

        Ok(recepisse.to_owned())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EventTag {
    #[serde(rename = "idObjet")]
    pub id: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_parsing() {
        let text = r#"<return xsi:type="ns1:PointagePositionReponse">
        <tabResultatsPointagePosition SOAP-ENC:arrayType="ns1:RetourReponse[1]" xsi:type="ns1:ArrayOfResultatsPointagePosition">
          <item xsi:type="ns1:RetourReponse">
            <sCode xsi:nil="true"/>
            <sLibelle xsi:type="xsd:string">Pointage de la position  récép. 87147  du 03/01/2024  avec le code EXPCFM</sLibelle>
            <oObjetCible xsi:type="ns1:ObjetMetier">
              <sTypeObjet xsi:type="xsd:string">CEvenement</sTypeObjet>
              <idObjet xsi:type="xsd:int">108933929</idObjet>
              <sDescription xsi:type="xsd:string"> EXPCFM 05/01/2024 à 12h09 </sDescription>
            </oObjetCible>
          </item>
        </tabResultatsPointagePosition>
      </return>"#;

        let part = extract_xml_tag("return", text);
        let s: core::result::Result<PositionEventRoot, _> = quick_xml::de::from_str(part.unwrap());
        let s = s.unwrap();
        let item = s.root_list.items.first().unwrap();

        assert_eq!(item.get_recepisse().unwrap(), "87147".to_owned());
        assert_eq!(item.event.id, 108933929);
    }
}
