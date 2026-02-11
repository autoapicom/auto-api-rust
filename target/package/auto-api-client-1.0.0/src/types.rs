use serde::Deserialize;
use serde_json::Value;

/// Parameters for `get_offers()`.
/// Use `..Default::default()` for optional fields.
#[derive(Debug, Default, Clone)]
pub struct OffersParams {
    pub page: i32,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub configuration: Option<String>,
    pub complectation: Option<String>,
    pub transmission: Option<String>,
    pub color: Option<String>,
    pub body_type: Option<String>,
    pub engine_type: Option<String>,
    pub year_from: Option<i32>,
    pub year_to: Option<i32>,
    pub mileage_from: Option<i32>,
    pub mileage_to: Option<i32>,
    pub price_from: Option<i32>,
    pub price_to: Option<i32>,
}

impl OffersParams {
    /// Converts parameters to a vector of (key, value) pairs for query string.
    /// Only non-None values are included.
    pub(crate) fn to_query_pairs(&self) -> Vec<(String, String)> {
        let mut pairs = vec![("page".to_string(), self.page.to_string())];

        if let Some(ref v) = self.brand { pairs.push(("brand".into(), v.clone())); }
        if let Some(ref v) = self.model { pairs.push(("model".into(), v.clone())); }
        if let Some(ref v) = self.configuration { pairs.push(("configuration".into(), v.clone())); }
        if let Some(ref v) = self.complectation { pairs.push(("complectation".into(), v.clone())); }
        if let Some(ref v) = self.transmission { pairs.push(("transmission".into(), v.clone())); }
        if let Some(ref v) = self.color { pairs.push(("color".into(), v.clone())); }
        if let Some(ref v) = self.body_type { pairs.push(("body_type".into(), v.clone())); }
        if let Some(ref v) = self.engine_type { pairs.push(("engine_type".into(), v.clone())); }
        if let Some(v) = self.year_from { pairs.push(("year_from".into(), v.to_string())); }
        if let Some(v) = self.year_to { pairs.push(("year_to".into(), v.to_string())); }
        if let Some(v) = self.mileage_from { pairs.push(("mileage_from".into(), v.to_string())); }
        if let Some(v) = self.mileage_to { pairs.push(("mileage_to".into(), v.to_string())); }
        if let Some(v) = self.price_from { pairs.push(("price_from".into(), v.to_string())); }
        if let Some(v) = self.price_to { pairs.push(("price_to".into(), v.to_string())); }

        pairs
    }
}

/// Response from `get_offers()` and `get_offer()`.
#[derive(Debug, Deserialize)]
pub struct OffersResponse {
    pub result: Vec<OfferItem>,
    pub meta: Meta,
}

/// Response from `get_changes()`.
#[derive(Debug, Deserialize)]
pub struct ChangesResponse {
    pub result: Vec<ChangeItem>,
    pub meta: ChangesMeta,
}

/// Pagination metadata for offers.
#[derive(Debug, Deserialize)]
pub struct Meta {
    pub page: i32,
    pub next_page: i32,
    pub limit: i32,
}

/// Pagination metadata for changes feed.
#[derive(Debug, Deserialize)]
pub struct ChangesMeta {
    pub cur_change_id: i64,
    pub next_change_id: i64,
    pub limit: i32,
}

/// A single item in the offers result array.
/// `data` is `serde_json::Value` because the structure varies between sources.
#[derive(Debug, Deserialize)]
pub struct OfferItem {
    pub id: i64,
    pub inner_id: String,
    pub change_type: String,
    pub created_at: String,
    pub data: Value,
}

/// A single item in the changes result array.
/// `data` is `serde_json::Value` because the structure varies between sources.
#[derive(Debug, Deserialize)]
pub struct ChangeItem {
    pub id: i64,
    pub inner_id: String,
    pub change_type: String,
    pub created_at: String,
    pub data: Value,
}

/// Common offer data fields shared across all sources.
/// Since each source may have additional fields, deserialize from
/// `OfferItem.data` into this or into your own struct.
#[derive(Debug, Deserialize)]
pub struct OfferData {
    pub inner_id: String,
    pub url: String,
    pub mark: String,
    pub model: String,
    pub generation: String,
    pub configuration: String,
    pub complectation: String,
    pub year: String,
    pub color: String,
    pub price: String,
    pub km_age: String,
    pub engine_type: String,
    pub transmission_type: String,
    pub body_type: String,
    pub address: String,
    pub seller_type: String,
    pub is_dealer: bool,
    pub displacement: String,
    pub offer_created: String,
    pub images: Vec<String>,
}

/// Response from `get_change_id()`.
#[derive(Debug, Deserialize)]
pub(crate) struct ChangeIdResponse {
    pub change_id: i64,
}
