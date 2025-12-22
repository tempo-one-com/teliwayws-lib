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
## 0.14.0 22/12/25
Ajout sRemarques dans position

## 0.13.0 16/12/25
Valeur par défaut pour TiersCreateOrUpdateWsRequest pour éviter de devoir tout remplir au niveau du client

## 0.12.0 16/12/25
Ajout des valeurs teliway pour TiersType

## 0.11.0 15/12/25
Ajout TiersCreationModification

## 0.10.0 15/09/25
Ajout info_palette_rendues

## 0.8.0 09/09/25
Ajout dtmDateHeureRDV dans PositionEventMarkerWsRequest

## 0.7.0 14/08/25
Ajout pour tous les web services d'un nouveau constructeur : new_from_url_with_access (pour les urls de type: https://<user>:<password>@<host>)

## 0.6.0 13/08/25
Ajout pour tous les web services d'un nouveau constructeur : new_with_auth qui prend en arguement WsAuth

## 0.5.2 11/04/25
Ajout extraction libellé sur PositionEventMarkerWsResponse

## 0.5.1 11/04/25
Ajout extraction libellé sur réponse de PositionEventMarkerSoapResponse

## 0.5.0 11/04/25
Prise en compte d'une erreur (évnènement non existant) sur PointerPosition

## 0.4.0 21/10/24
Utilisation de NaiveDateTime sur Pointer Position

## 0.3.1 19/09/24
ReclamationCreateSoapRequest : remplacement de "extranet" pour sLogin par req.author

## 0.3 29/08/24
NEW: ajout création réclamation

## 0.1.4 28/02/24
NEW: Refactorisation pour simplifier code des programmes appelant cette librairie. Tous les web service (PositionEventMarkerWs, TiersUpdateSiretWs) doivent maintenant hériter de WebServiceTeliwaySoap

## 0.1.3 27/02/24
NEW: new sur WebServiceTeliwaySoap

## 0.1.2 22/01/24
Downgrade version regex (1.10.2 -> 1.0.0) pour éviter conflit avec axum 0.7.2

## 0.1.1 19/01/24
Implémentation des appels suivants:
- GestionTiers : update du siret (edi) du tiers
- GestionPosition : marque des positions avec un évènement
