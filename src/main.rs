use todo::configuration::{get_connection_pool, load_configuration};
use todo::model::AppState;
use todo::rest::get_routes;

#[tokio::main]
async fn main() {
    let configuration = load_configuration().expect("Failed to read configuration.");
    let connection_pool = get_connection_pool(&configuration.database);

    let app = get_routes()
        .with_state(AppState { db: connection_pool });

    let url = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = tokio::net::TcpListener::bind(url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}