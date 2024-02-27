# Description
Effectue les appels aux WS Soap de Teliway en masquant la partie Soap.
Les requêtes/réponses sont sous forme de struct

# Structures
## Gestion tiers
- Modification du siret edi du tiers  

Requête
```
pub struct TiersUpdateSiretWsRequest {
   pub tiers_id: i32,
   pub tiers_type: String,
   pub siret: Option<String>,
}
```

Réponse
```
pub struct TiersWsResponse {
    pub tiers_id: i32,
    pub code: String,
    pub tiers_type: i32,
    pub name: String,
    pub siret_edi: String,
    pub siret_administratif: String,
}
```

## Gestion position
- Marquage position avec évènement simple (pas les POD qui nécessite un fichier)  

Requête
```
pub struct PositionEventMarkerWsRequest {
    pub position_ids: Vec<i32>,
    pub event_code: String,
    pub date: DateTime<Local>,
    pub created_by: String,
    pub agence_code: String,
}
```

Réponse
```
pub struct PositionEventMarkerWsResponse {
    pub positions: Vec<PositionEvent>,
}

pub struct PositionEvent {
    pub recepisse: String,
    pub event_id: i32,
}
```
  
# Versions
## 0.1.3 27/02/24
NEW: new sur WebServiceTeliwaySoap

## 0.1.2 22/01/24
Downgrade version regex (1.10.2 -> 1.0.0) pour éviter conflit avec axum 0.7.2

## 0.1.1 19/01/24
Implémentation des appels suivants:
- GestionTiers : update du siret (edi) du tiers
- GestionPosition : marque des positions avec un évènement 