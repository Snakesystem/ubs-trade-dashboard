use actix_web::{web, Responder, Scope, get, HttpResponse};
use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use serde_json::json;

use crate::{contexts::model::{ActionResult, ResultList, TableDataParams}, services::data_service::DataService};

pub fn data_scope() -> Scope {
    web::scope("/data")
        .service(get_header)
        .service(get_table_data)
}

#[get("/header")]
pub async fn get_header(pool: web::Data<Pool<ConnectionManager>>) -> impl Responder {

   let result = DataService::get_header().await;

   HttpResponse::Ok().json(result)
}

#[get("/get-table")]
async fn get_table_data(params: web::Query<TableDataParams>, pool: web::Data<Pool<ConnectionManager>>) -> impl Responder {

    let data: Result<ResultList, Box<dyn std::error::Error>> = DataService::get_table_data(params.into_inner(), pool).await;

    match data {
        Ok(response) => {
            return HttpResponse::Ok().json(response);
        },
        Err(e) => {
            return HttpResponse::InternalServerError().json(
                json!({"error": e.to_string()})
            );
        },
        
    }
}