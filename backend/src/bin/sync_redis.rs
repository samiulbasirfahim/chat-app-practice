use std::env;

use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABSE_URL is missing");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL is missing");

    let db_pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Failed to connect with database");
    let redis_client = redis::Client::open(redis_url).expect("Failed to open REDIS");
    let mut redis = redis_client
        .get_multiplexed_async_connection()
        .await
        .expect("Failed to get Redis connection");

    let keys: Vec<String> = redis::cmd("KEYS")
        .arg("username:*")
        .query_async(&mut redis)
        .await
        .expect("Failed to retrieve usernames from REDIS");
    println!(
        "Found {} username keys in REDIS, clearing up...",
        keys.len()
    );

    if !keys.is_empty() {
        let _: () = redis::cmd("DEL")
            .arg(&keys)
            .query_async(&mut redis)
            .await
            .expect("Failed to clear up usernames from REDIS");
        println!("Cleared up {} username keys from REDIS", keys.len());
    }

    let record = sqlx::query!("SELECT username FROM users")
        .fetch_all(&db_pool)
        .await
        .expect("Failed to retrieve username from DATABASE");
    print!(
        "Found {} usernames in DATABASE, writing to REDIS...",
        record.len()
    );

    let mut pipeline = redis::pipe();

    for record in &record {
        let redis_key = format!("username:{}", record.username.to_lowercase());
        pipeline.cmd("SET").arg(redis_key).arg("1").ignore();
    }

    let _: () = pipeline
        .query_async(&mut redis)
        .await
        .expect("Failed to write username on REDIS");
    println!("Done!");
}
