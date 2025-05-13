use actix_web::{get, web, HttpResponse, Responder};
use utoipa::{OpenApi, ToSchema};

use crate::contexts::model::{ActionResult, Claims, HeaderParams, LoginRequest, TableDataParams};

#[derive(serde::Serialize, ToSchema)]
struct HealthCheckResponse {
    message: String,
}

// Login Docs
#[utoipa::path(post, path = "/api/v1/auth/login", request_body = LoginRequest,
    responses(
        (status = 200, description = "Check Session", body = ActionResult<Claims, String>, example = json!({"result": true, "message": "Login Success", "data": {
            "user_id": "1",
            "username": "admin",
            "email": "LXh4N@example.com",
            "company_id": "SS",
            "company_name": "Snake System Tech"
        }})),
        (status = 401, description = "Unauthorized", body = ActionResult<String, String>, example = json!({
            "result": false, 
            "message": "Unauthorized", 
            "error": "Unauthorized"
        })),
        (status = 500, description = "Internal Server Error", body = ActionResult<String, String>, example = json!({
            "result": false, 
            "message": "User not found", 
            "error": "Internal Server Error"
        })),
        (status = 400, description = "Bad Request", body = ActionResult<String, String>, example = json!({
            "result": false, 
            "message": "Token not found", 
            "error": "Bad Request"
        }))
    ),
    tag = "1. Auth Endpoints"
)]
#[allow(dead_code)]
pub fn login_doc() {}

// Check Session Docs
#[utoipa::path(
    get,
    path = "/api/v1/auth/session",
    summary = "Cek sesi login pengguna",
    description = "`Wajib login terlebih dahulu. Memerlukan token dari cookies` untuk mengecek sesi login pengguna",
    responses(
        (status = 200, description = "Check Session", body = ActionResult<Claims, String>, example = json!({
            "result": true,
            "message": "Session active",
            "data": {
                "user_id": "1",
                "username": "admin",
                "email": "admin@example.com"
            }
        })),
        (status = 401, description = "Unauthorized", body = ActionResult<String, String>, example = json!({
            "result": false,
            "message": "Unauthorized",
            "error": "Unauthorized"
        })),
        (status = 500, description = "Internal Server Error", body = ActionResult<String, String>, example = json!({
            "result": false,
            "message": "Token has expired",
            "error": "Internal Server Error"
        })),
        (status = 400, description = "Bad Request", body = ActionResult<String, String>, example = json!({
            "result": false,
            "message": "Token not found",
            "error": "Bad Request"
        }))
    ),
    tag = "1. Auth Endpoints"
)]
#[allow(dead_code)]
pub fn check_session_doc() {}

// Logout Docs
#[utoipa::path(post, path = "/api/v1/auth/logout", 
    responses(
        (status = 200, description = "Logout Success", body = ActionResult<String, String>)
    ),
    tag = "1. Auth Endpoints"
)]
#[allow(dead_code)]
pub fn logout_doc() {}

// Company Docs
#[utoipa::path(get, path = "/api/v1/generic/company",
    responses(
        (status = 200, description = "Check Session", body = ActionResult<Claims, String>, example = json!({"result": true, "message": "Login Success", "data": {
            "company_id": "SS",
            "company_name": "Snake System Tech"
        }})),
        (status = 500, description = "Internal Server Error", body = ActionResult<String, String>, example = json!({
            "result": false, 
            "message": "User not found", 
            "error": "Internal Server Error"
        }))
    ),
    tag = "2. Generic Endpoints"
)]
#[allow(dead_code)]
pub fn get_company_docs() {}

// Company Docs
#[utoipa::path(get, path = "/random-url/test",
    responses(
        (status = 404, description = "Internal Server Error", body = ActionResult<String, String>, example = json!({
            "result": false, 
            "message": "Not found", 
            "error": "Url '/random-url/test' not found. Please check the URL."
        }))
    ),
    tag = "2. Generic Endpoints"
)]
#[allow(dead_code)]
pub fn not_found_docs() {}

// Get header Docs
#[utoipa::path(
    get,
    path = "/api/v1/data/header",
    summary = "Get generic columns",
    description = "`Wajib login terlebih dahulu. Memerlukan token dari cookies` untuk mengecek sesi login pengguna",
    params(
        HeaderParams
    ),
    responses(
        (status = 200, description = "Check Session", body = ActionResult<Claims, String>, example = json!({
            "result": true,
            "message": "Data retrieved successfully",
            "data": [
                {
                    "field": "DataNID",
                    "filterControl": "input",
                    "sortable": true,
                    "title": "Data NID"
                },
                {
                    "field": "DataName",
                    "filterControl": "input",
                    "sortable": true,
                    "title": "Data Name"
                },
            ]
        })),
        (status = 401, description = "Unauthorized", body = ActionResult<String, String>, example = json!({
            "result": false,
            "message": "Unauthorized",
            "error": "Unauthorized"
        })),
        (status = 500, description = "Internal Server Error", body = ActionResult<String, String>, example = json!({
            "result": false,
            "message": "Token has expired",
            "error": "Internal Server Error"
        })),
        (status = 400, description = "Bad Request", body = ActionResult<String, String>, example = json!({
            "result": false,
            "message": "Token not found",
            "error": "Bad Request"
        }))
    ),
    tag = "3. Data Endpoints"
)]
#[allow(dead_code)]
pub fn get_header_docs(_: web::Query<HeaderParams>) {}

// Get Table Data Docs
#[utoipa::path(
    get,
    path = "/api/v1/data/get-table",
    summary = "Get generic columns",
    description = "`Wajib get header terlebih dahulu.` untuk mengecek header columns",
    params(
        TableDataParams
    ),
    responses(
        (status = 200, description = "Data retrieved successfully", example = json!({
            "totalNotFiltered": 222,
            "total": 222,
            "rows": [
                {
                "DataNID": 1,
                "DataID": "DATA-123",
                "DataName": "Jasa Keuangan Pasar Senggol",
                "DataDescription": "Jasa Keuangan Pasar Senggol",
                "LastUpdate": "2021-01-01"
                },
                {
                "DataNID": 2,
                "DataID": "DATA-124",
                "DataName": "Jasa Keuangan Pasar Kecil",
                "DataDescription": "Jasa Keuangan Pasar Kecil",
                "LastUpdate": "2021-01-01"
                },
                {
                "DataNID": 3,
                "DataID": "DATA-125",
                "DataName": "Jasa Keuangan Pasar Besar",
                "DataDescription": "Jasa Keuangan Pasar Besar",
                "LastUpdate": "2021-01-01"
                }
            ]
    
        })),
        (status = 500, description = "Internal Server Error", example = json!({
            "error": "Token error: 'Invalid column name 'AutoNID'.' on server S3 executing  on line 1 (code: 207, state: 1, class: 16)"
        })),
    ),
    tag = "3. Data Endpoints"
)]
#[allow(dead_code)]
pub fn get_table_data_docs(params: web::Query<TableDataParams>) {
    params.into_inner();
}

// Health Check Docs
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Health Check Success", body = HealthCheckResponse, example = json!(HealthCheckResponse { message: "Welcome to the snakesystem app!".to_string(), }))
    ),
    tag = "0. Application Default Endpoints"
)]

#[get("/")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthCheckResponse {
        message: "Welcome to the snakesystem app!".to_string(),
    })
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "UBS Trade Dashboard API",
        description = "Dokumentasi untuk RESTful API UBS Trade Dashboard.\n\nSilakan gunakan token JWT untuk mengakses endpoint yang dilindungi.",
        version = "1.0.0"
    ),
    paths(
        health_check,
        login_doc,
        check_session_doc,
        logout_doc,
        get_company_docs,
        not_found_docs,
        get_header_docs,
        get_table_data_docs
    ),
    components(
        schemas(ActionResult<Claims, String>)
    ),
    tags(
        (name = "0. Application Default Endpoints", description = "Default path application endpoints"),
        (name = "1. Auth Endpoints", description = "Authentication related endpoints"),
        (name = "2. Generic Endpoints", description = "Another related endpoints"),
        (name = "3. Data Endpoints", description = "Data related endpoints")
    )
)]

pub struct ApiDoc;