pub mod auth_service;
pub mod chat_service;
pub mod health_service;
pub mod user;

mod utils {
    pub fn kill_process() {
        println!("Killing process...");
    }
}

pub fn run_service() {
    crate::service::utils::kill_process();
}
