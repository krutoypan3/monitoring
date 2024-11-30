use std::thread::sleep;
use std::time::Duration;
use serde::Serialize;
use sqlx::{Error, FromRow, Pool, Postgres};
use crate::schedulers::availability_checker;

#[derive(Serialize, FromRow)]
pub struct PgInfo {
    pub pg_size: Option<String>,
}

pub async fn start(pg_pool: Pool<Postgres>) {
    println!("Scheduler is starting...");
    loop {
        let result: Result<PgInfo, Error> = sqlx::query_as!(
            PgInfo,
            "SELECT pg_size_pretty(pg_database_size('monitoring')) as pg_size"
        ).fetch_one(&pg_pool).await;

        match result {
            Ok(pg_info) => {
                println!("Current time is {:?}, Database size is {}", chrono::Utc::now().format("%H:%M:%S").to_string(), pg_info.pg_size.unwrap_or("unknown".to_string()));
            },
            Err(_e) => {
                println!("Current time is {:?}, Database is not available", chrono::Utc::now().format("%H:%M:%S").to_string());
            }
        }

        for url in [
            "https://ya.ru/",
            "https://vk.com/",
            "https://www.sberbank.ru/",
        ] {
            availability_checker::check(pg_pool.clone(), url).await;
        }


        sleep(Duration::from_secs(30));
    }
}

