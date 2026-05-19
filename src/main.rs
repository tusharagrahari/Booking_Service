use std::sync::Arc;

use actix_web::{App, HttpServer, middleware, web};
use lumel::flash_sale::{
    routes,
    state::{SalesState, SharedState},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared = Arc::new(SharedState::new(SalesState::default()));

    println!("Listening on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(Arc::clone(&shared)))
            .app_data(web::JsonConfig::default().error_handler(|err, _| {
                let msg = format!("JSON parse error: {err}");
                actix_web::error::InternalError::from_response(
                    err,
                    actix_web::HttpResponse::BadRequest().json(serde_json::json!({ "error": msg })),
                )
                .into()
            }))
            .configure(routes::configure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
