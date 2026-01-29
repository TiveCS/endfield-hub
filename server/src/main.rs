mod app;
mod internal;

use actix_web::{App, HttpServer, Responder, get, post, web};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use utoipa::OpenApi;
use utoipa::ToSchema;
use utoipa_actix_web::{AppExt, scope};
use utoipa_scalar::{Scalar, Servable};

use internal::db::establish_connection;

// ============================================================================
// MODELS
// ============================================================================

#[derive(Serialize, ToSchema)]
pub struct GreetingResponse {
    pub message: String,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, ToSchema)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

// ============================================================================
// HANDLERS - Just add #[utoipa::path], routes are auto-collected!
// ============================================================================

#[utoipa::path(
    responses((status = 200, body = GreetingResponse)),
    tag = "General"
)]
#[get("/")]
async fn hello() -> impl Responder {
    web::Json(GreetingResponse {
        message: "Hello world!".to_string(),
    })
}

#[utoipa::path(
    responses(
        (status = 200, body = User),
        (status = 404, description = "User not found")
    ),
    tag = "Users"
)]
#[get("/users/{id}")]
async fn get_user(path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    web::Json(User {
        id,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
    })
}

#[utoipa::path(
    responses(
        (status = 201, body = User),
        (status = 400, description = "Invalid input")
    ),
    tag = "Users"
)]
#[post("/users")]
async fn create_user(body: web::Json<CreateUserRequest>) -> impl Responder {
    web::Json(User {
        id: 1,
        name: body.name.clone(),
        email: body.email.clone(),
    })
}

// ============================================================================
// OPENAPI - Minimal! No need to list paths or schemas manually
// ============================================================================

#[derive(OpenApi)]
#[openapi(
    info(title = "Endfield Hub API", version = "1.0.0"),
    tags(
        (name = "General", description = "General endpoints"),
        (name = "Auth", description = "Auth")
    )
)]
pub struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db: DatabaseConnection = establish_connection(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Server running at http://localhost:8080");
    println!("API Docs at http://localhost:8080/scalar");

    let db_data = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            // into_utoipa_app() enables auto-collection
            .into_utoipa_app()
            // Use utoipa_actix_web::scope instead of web::scope for auto-detection
            .service(
                scope("/api")
                    .service(hello)
                    .service(get_user)
                    .service(create_user),
            )
            // This auto-collects all routes registered above!
            .openapi_service(|api| Scalar::with_url("/scalar", api))
            .into_app()
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
