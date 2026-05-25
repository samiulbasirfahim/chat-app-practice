use actix_web::main;

#[main]
async fn main() {
    if let Err(e) = backend::run_server().await {
        eprintln!("Server error: {}", e);
    }
}
