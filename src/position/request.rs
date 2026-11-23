use chrono::{DateTime, Local};

#[derive(Debug, Clone, Default)]
pub struct PositionEventMarkerWsRequest {
    pub position_ids: Vec<i32>,
    pub event_code: String,
    pub datetime: DateTime<Local>,
    pub created_by: String,
    pub agence_code: String,
    pub date_rdv: Option<DateTime<Local>>,
    pub info_palette_rendu: Option<String>,
    pub comment: Option<String>,
    /// Pièces jointes de l'événement (POD…), envoyées dans `tabOFichier`
    pub files: Vec<EventFile>,
}

/// Fichier joint à un événement
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventFile {
    /// Nom du fichier, avec son extension (`pod.pdf`)
    pub name: String,
    /// Type MIME (`application/pdf`, `image/jpeg`…)
    pub mime_type: String,
    /// Contenu brut, encodé en base64 à l'envoi
    pub content: Vec<u8>,
}
