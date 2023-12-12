use todo::configuration::{get_connection_pool, load_configuration};
use todo::model::AppState;
use todo::rest::get_routes;

#[tokio::main]
async fn main() {
    let configuration = load_configuration().expect("Failed to read configuration.");
    let connection_pool = get_connection_pool(&configuration.database);

    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let app = get_routes()
        .with_state(AppState { db: connection_pool });

    let url = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = tokio::net::TcpListener::bind(url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}