use actix_web::{get, web, HttpResponse, Responder, Scope};
use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use serde::Deserialize;

use crate::{contexts::model::{ActionResult, Company, Order}, services::generic_service::GenericService};

pub fn generic_scope() -> Scope {
    web::scope("/generic")
        .service(get_company)
        .service(get_orders)
}

#[get("/company")]
pub async fn get_company(pool: web::Data<Pool<ConnectionManager>>) -> impl Responder {

    let result: ActionResult<Company, _> = GenericService::get_company(pool).await;

    match result {
        response if response.error.is_some() => {
            HttpResponse::InternalServerError().json(response)
        }, 
        response if response.result => {
            HttpResponse::Ok().json(response)
        }, 
        response => {
            HttpResponse::BadRequest().json(response)
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct OrderQueryParams {
    pub last_id: Option<i64>,
    pub limit: Option<i32>,
}

#[get("/orders")]
pub async fn get_orders(
    pool: web::Data<Pool<ConnectionManager>>,
    query: web::Query<OrderQueryParams>,
) -> impl Responder {
    let result: ActionResult<Vec<Order>, _> =
        GenericService::get_orders(pool, query.last_id, query.limit).await;

    match result {
        response if response.error.is_some() => {
            HttpResponse::InternalServerError().json(response)
        }
        response if response.result => {
            HttpResponse::Ok().json(response)
        }
        response => {
            HttpResponse::BadRequest().json(response)
        }
    }
}
