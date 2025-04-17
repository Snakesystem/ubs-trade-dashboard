use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize};
// pub struct DateTimeConverter;

// impl DateTimeConverter {
//     pub fn from_string(date_str: &str) -> Result<chrono::NaiveDateTime, chrono::ParseError> {
//         let format = "%Y-%m-%d %H:%M:%S";
//         chrono::NaiveDateTime::parse_from_str(date_str, format)
//     }
// }

#[derive(Debug, Serialize)]
pub struct ActionResult<T, E> {
    pub result: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<E>,
}

// Implementasi Default
impl<T, E> Default for ActionResult<T, E> {
    fn default() -> Self {
        Self {
            result: false, // Default-nya false
            message: String::new(),
            data: None,
            error: None,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Company {
    pub company_id: String,
    pub company_name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ListData {
    pub data_id: i32,
    pub code: String,
    pub description: String,
}

fn serialize_datetime<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let formatted = dt.format("%Y-%m-%d %H:%M:%S").to_string();
    serializer.serialize_str(&formatted)
}

fn deserialize_date_only<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let date_str: Option<String> = Option::deserialize(deserializer)?;
    if let Some(date) = date_str {
        let naive_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
            .map_err(serde::de::Error::custom)?;
        let datetime = Utc.from_utc_datetime(&naive_date.and_hms_opt(0, 0, 0).unwrap());
        return Ok(Some(datetime));
    }
    Ok(None)
}

#[derive(Debug, Serialize)]
pub struct Order {
    pub id: i32,
    pub customer_name: String,
    pub total_price: f64,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TableDataParams {
    pub tablename: String,
    pub limit: i32,
    pub offset: i32,
    pub filter: Option<String>,
    pub sort: Option<String>,
    pub order: Option<String>,
    pub nidkey: Option<String>,
    // pub nidvalue: Option<String>,
}

#[derive(Debug)]
pub struct QueryClass {
    pub query: String,
    pub query_total_all: String,
    pub query_total_with_filter: String,
}

#[derive(Debug, Serialize)]
pub struct ResultList {
    pub total: i64,
    pub total_with_filter: i64,
    pub rows: Vec<serde_json::Value>, // Pastikan ini bisa dikonversi ke JSON
}