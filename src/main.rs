use std::net::SocketAddr;

use axum::Extension;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::Method;
use tower_http::cors::{AllowOrigin, CorsLayer};

use handlers::{persons, recordings, todos};
use types::HandledResult;

pub mod mongo;
pub mod handlers;
pub mod errors;
pub mod types;
pub mod jwt;
pub mod utils;

#[tokio::main]
async fn main() -> HandledResult<()> {
    let cors_layer = CorsLayer::new()
        .allow_origin(AllowOrigin::any())
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let (
        persons,
        recordings,
        todos
    ) = mongo::setup().await?;

    let app = persons::router()
        .merge(recordings::router())
        .merge(todos::router())
        .layer(cors_layer)
        .layer(Extension(persons))
        .layer(Extension(recordings))
        .layer(Extension(todos));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await.unwrap();

    Ok(())
}
