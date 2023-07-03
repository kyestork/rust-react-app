mod libs;
use libs::error::AppError;
use tower_http::cors::{Any, CorsLayer};
use std::net::SocketAddr;
use axum::{routing::get, response::Json, Router};
use serde_json::{Value, json};
use models::User;

// use user::Model;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);
    
    let app = Router::new()
        .route("/api", get(root))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 9000));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Json<Value> {
    Json(json!({ 
        "code": 200,
        "status": "success",
        "data": {
            "text" : "It works."
        }
    }))
}