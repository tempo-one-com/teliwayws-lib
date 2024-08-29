use crate::{
    error::{Error, Result},
    utils::xml::extract_xml_tag,
};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ReclamationSoapResponse {
    #[serde(rename = "idReclamation")]
    pub id: i32,
}

impl TryFrom<String> for ReclamationSoapResponse {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        let tag = "resultatCreationModificationReclamation";
        let xml = extract_xml_tag(tag, &value).ok_or(Error::Parsing(tag.to_string()))?;
        let data: core::result::Result<ReclamationSoapResponse, quick_xml::DeError> =
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
        let text = r#"<return xsi:type="ns1:CreationModificationReclamationReponse">
        <resultatCreationModificationReclamation xsi:type="ns1:Reclamation">
            <idReclamation xsi:type="xsd:int">599</idReclamation>
        </resultatCreationModificationReclamation>
      </return>"#;

        let part = extract_xml_tag("idReclamation", text);

        assert_eq!(
            part,
            Some(r#"<idReclamation xsi:type="xsd:int">599</idReclamation>"#)
        )
    }

    #[test]
    fn into_struct() {
        let text = r#"<return xsi:type="ns1:CreationModificationReclamationReponse">
        <resultatCreationModificationReclamation xsi:type="ns1:Reclamation">
          <idReclamation xsi:type="xsd:int">599</idReclamation>
        </resultatCreationModificationReclamation>
      </return>"#;

        let s: core::result::Result<ReclamationSoapResponse, _> = text.to_owned().try_into();
        assert_eq!(s.clone().unwrap().id, 599);
    }
}
