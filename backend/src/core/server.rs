use std::time::Duration;

use crate::core::config::get_config;
use crate::core::db::create_db_pool;
use crate::endpoints::register_endpoints;
use crate::utils::error_handler::{generate_error_handlers, generate_validation_handler};
use actix_web::{App, HttpServer, web};

use super::db::build_redis_client;

pub async fn run_server() -> std::io::Result<()> {
    let port = get_config().port;
    let db = web::Data::new(create_db_pool().await.expect("Failed to create DB pool"));
    let redis = web::Data::new(build_redis_client().await.expect("Failed to open REDIS"));

    println!("Server is running on PORT: {}", port);

    HttpServer::new(move || {
        let error_handler = generate_error_handlers();
        let validation_handler = generate_validation_handler();
        App::new().service(
            web::scope("/api")
                .app_data(db.clone())
                .app_data(redis.clone())
                .app_data(validation_handler)
                .app_data(error_handler)
                .configure(register_endpoints),
        )
    })
    .client_request_timeout(Duration::from_secs(30))
    .workers(1)
    .bind(("0.0.0.0", port))?
    .run()
    .await?;
    Ok(())
}
