use chrono::{DateTime, Utc};
use sqlx::{FromRow, Pool, Postgres};
use plotly::{Plot, Scatter, Layout};
use serde::Serialize;

pub async fn render_graph(pg_pool: Pool<Postgres>) -> String {
    let mut x: Vec<i16> = vec![];
    let mut y: Vec<String> = vec![];

    for item in get_from_db(pg_pool).await {
        x.push(item.status_code);
        y.push(item.timestamp.to_string());
    }

    // Создаем график
    let trace = Scatter::new(y.clone(), x.clone()).name("Trace");
    let layout = Layout::new().title("Доступность всех сервисов");
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);

    // Получаем HTML-код графика
    let html = plot.to_html();
    html
}






async fn get_from_db(pg_pool: Pool<Postgres>) -> Vec<Availability> {
    sqlx::query_as!(
        Availability,
        "SELECT * FROM availability",
    )
        .fetch_all(&pg_pool)
        .await.unwrap()
}

#[derive(Serialize, FromRow)]
pub struct Availability {
    id: i64,
    ip_or_domain: String,
    status_code: i16,
    timestamp: DateTime<Utc>,
}