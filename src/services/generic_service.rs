use actix_web::{error, web, HttpRequest, HttpResponse, Responder, Result};
use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use futures::StreamExt;
use serde_json::json;
use tiberius::{QueryItem, QueryStream};

use crate::contexts::model::{ActionResult, Company, Order};

pub struct GenericService;

impl GenericService {
    pub async fn get_company(
        connection: web::Data<Pool<ConnectionManager>>,
    ) -> ActionResult<Company, String> {
        let mut result = ActionResult::default();

        match connection.clone().get().await {
            Ok(mut conn) => {
                let query_result: Result<QueryStream, _> = conn
                    .query("SELECT CompanyID, CompanyName FROM Company", &[])
                    .await;
                match query_result {
                    Ok(rows) => {
                        if let Ok(Some(row)) = rows.into_row().await {
                            result.result = true;
                            result.message = "Company name".to_string();
                            result.data = Some(Company {
                                company_id: row
                                    .get::<&str, _>("CompanyID")
                                    .map_or_else(|| "".to_string(), |s| s.to_string()),
                                company_name: row
                                    .get::<&str, _>("CompanyName")
                                    .map_or_else(|| "".to_string(), |s| s.to_string()),
                            });
                            return result;
                        } else {
                            result.message = "No company found".to_string();
                            return result;
                        }
                    }
                    Err(e) => {
                        result.message = "Internal Server Error".to_string();
                        result.error = Some(e.to_string());
                        return result;
                    }
                }
            }
            Err(e) => {
                result.error = Some(e.to_string());
                return result;
            }
        }
    }

    pub async fn get_orders(
        pool: web::Data<Pool<ConnectionManager>>,
        last_id: Option<i64>,
        limit: Option<i32>,
    ) -> ActionResult<Vec<Order>, String> {
        let mut result = ActionResult::<Vec<Order>, String>::default();

        let limit = limit.unwrap_or(50).min(10000);
        let last_id = last_id.unwrap_or(0);

        match pool.clone().get().await {
            Ok(mut conn) => {
                let query = r#"
                    SELECT TOP (@P1) OrderNID, BuySell, CAST(OrderPrice AS float) as OrderPrice, OrderDate
                    FROM [Order]
                    WHERE OrderNID > @P2
                    ORDER BY OrderNID ASC
                "#;

                match conn.query(query, &[&limit, &last_id]).await {
                    Ok(mut rows) => {
                        let mut data = Vec::new();

                        while let Some(row_result) = rows.next().await {
                            match row_result {
                                Ok(QueryItem::Row(row)) => {
                                    data.push(Order {
                                        id: row.get::<i32, _>("OrderNID").unwrap_or_default(),
                                        customer_name: row
                                            .get::<&str, _>("BuySell")
                                            .unwrap_or_default()
                                            .to_string(),
                                        total_price: row.get::<f64, _>("OrderPrice").unwrap_or(0.0),
                                        created_at: row
                                            .get::<chrono::NaiveDateTime, _>("OrderDate")
                                            .unwrap(),
                                    });
                                }
                                Ok(_) => continue, // Misal QueryItem::Done atau lainnya
                                Err(e) => {
                                    result.message = "Error reading row".to_string();
                                    result.error = Some(e.to_string());
                                    return result;
                                }
                            }
                        }

                        result.result = true;
                        result.message = "List orders".to_string();
                        result.data = Some(data);
                    }
                    Err(e) => {
                        result.message = "Failed to query data".to_string();
                        result.error = Some(e.to_string());
                    }
                }
            }
            Err(e) => {
                result.message = "Failed to connect to DB".to_string();
                result.error = Some(e.to_string());
            }
        }

        result
    }

    pub async fn not_found(req: HttpRequest) -> impl Responder {
        HttpResponse::NotFound().json({
            json!({
                "result": false,
                "message": "Not Found",
                "error": format!("Url '{}' not found. Please check the URL.", req.path())
            })
        })
    }

    pub fn json_error_handler(
        err: error::JsonPayloadError,
        _req: &actix_web::HttpRequest,
    ) -> actix_web::Error {
        let error_message = format!("Json deserialize error: {}", err);

        let result = ActionResult::<String, _> {
            // <- Ubah dari ActionResult<()> ke ActionResult<String>
            result: false,
            message: "Invalid Request".to_string(),
            error: Some(error_message), // <- Sekarang cocok karena `data: Option<String>`
            data: None,
        };

        error::InternalError::from_response(err, HttpResponse::BadRequest().json(result)).into()
    }

    /// Helper untuk validasi path parameter yang harus berupa integer
    pub fn parse_param<T: std::str::FromStr>(param: &str) -> Result<T, HttpResponse> {
        param.parse::<T>().map_err(|_| {
            HttpResponse::BadRequest().json(json!({
                "result": false,
                "message": "Bad Request",
                "error": format!("Invalid parameter '{}'. Please provide a valid {}", param, std::any::type_name::<T>())
            }))
        })
    }
}
