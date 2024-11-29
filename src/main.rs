use axum::{extract::State, routing::get, Router};
use dotenv::dotenv;
use std::future::IntoFuture;

mod schedulers;

#[derive(Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: String,
    pub test_env_load: String,
    pub pkg_name: String,
    pub pkg_version: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let server_config = ServerConfig {
        host: dotenv::var("SERVER_HOST").unwrap_or("0.0.0.0".to_string()),
        port: dotenv::var("SERVER_PORT").unwrap_or("3000".to_string()),
        test_env_load: dotenv::var("ENV_LOAD_TEST").unwrap_or("ENV NOT LOADED".to_string()),
        pkg_name: env!("CARGO_PKG_NAME").into(),
        pkg_version: env!("CARGO_PKG_VERSION").into(),
    };

    // our router
    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/about", get(about))
        .route("/foo/bar", get(foo_bar))
        .with_state(server_config.clone());



    // run our app with hyper, listening globally on port 3000
    let addr = format!("{}:{}", &server_config.host, &server_config.port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    let (_res1, _res2) = futures::join!(
        schedulers::start(),
        axum::serve(listener, app).into_future(),
    );
}


async fn about(State(server_config): State<ServerConfig>) -> String {
    format!("Package: {}\nVersion: {}\nTEST LOAD ENV: {}", &server_config.pkg_name, &server_config.pkg_version, &server_config.test_env_load)
}
async fn root() -> String {
    "Root, World!".to_string()
}
async fn get_foo() -> String {
    "Get, World!".to_string()
}
async fn post_foo() -> String {
    "Post, World!".to_string()
}
async fn foo_bar() -> String {
    "Foo, World!".to_string()
}