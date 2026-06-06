use actix_web::main;
use vyxra::{get_config, run_server};

#[main]
async fn main() {
    std::fs::create_dir_all(&get_config().upload_dir).expect("Failed to create UPLOAD directory");

    if let Err(e) = run_server().await {
        eprintln!("Server error: {}", e);
    }
}
