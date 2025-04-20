
use actix_web::{get, post, web, HttpResponse, Responder, Scope};
use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use serde_json::json;

use crate::{contexts::model::{ActionResult, BarChartParams, BarChartRequest}, services::chart_service::ChartService};

pub fn chart_scope() -> Scope {
    web::scope("/chart")
        .service(get_bar_chart)
        .service(create_bar_chart)
        .service(get_chart_data)
}

#[post("/create-bar")]
async fn create_bar_chart(pool: web::Data<Pool<ConnectionManager>>, request: web::Json<BarChartRequest>) -> impl Responder {

    let result: ActionResult<(), _> = ChartService::save_bar_chart(pool, request.into_inner()).await;

    match result {
        response if response.error.is_some() => {
            HttpResponse::InternalServerError().json(response)
        }, // Jika error, HTTP 500
        response if response.result => HttpResponse::Ok().json(response), // Jika berhasil, HTTP 200
        response => HttpResponse::BadRequest().json(response), // Jika gagal, HTTP 400
    }
}

#[get("/data/{menu_id}")]
pub async fn get_chart_data(pool: web::Data<Pool<ConnectionManager>>, menu_id: web::Path<String>) -> impl Responder {

    let params: String = menu_id.into_inner();

    // Cek apakah params kosong atau hanya spasi
    if params.trim().is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "result": false,
            "message": "Bad Request",
            "error": "Menu ID is empty"
        }));
    }

    let result: ActionResult<Vec<serde_json::Value>, String> = ChartService::get_chart_data(pool, params).await;

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

#[get("/bar")]
pub async fn get_bar_chart(pool: web::Data<Pool<ConnectionManager>>, params: web::Query<BarChartParams>) -> impl Responder {

    let result: ActionResult<Vec<serde_json::Value>, String> = ChartService::get_bar_chart(pool, params.into_inner()).await;

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