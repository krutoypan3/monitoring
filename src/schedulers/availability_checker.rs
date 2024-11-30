use sqlx::{Pool, Postgres};

pub async fn check(pg_pool: Pool<Postgres>, url: &str) {
    let timeout = 30000;
    match reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .timeout(std::time::Duration::from_millis(timeout))
        .build()
    {
        Err(err) => {
            println!("availability_checker::check error: {}", err);
        },
        Ok(client) => {
            match client.get(url).send().await {
                Ok(resp) => add_to_db(pg_pool, url, resp.status().as_str().parse().unwrap_or(404)),
                Err(err) => {
                    println!("availability_checker::check error: {}", err);
                    add_to_db(pg_pool, url, 404)
                }
            }.await;
        }
    }
}

async fn add_to_db(pg_pool: Pool<Postgres>, ip_or_domain: &str, status_code: i16) {
    let _ = sqlx::query!(
        "INSERT INTO availability(ip_or_domain, status_code) VALUES ($1,$2)",
        ip_or_domain,
        status_code,
    ).execute(&pg_pool).await.unwrap();
}