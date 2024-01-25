use tokio::net::TcpListener;
use tracing::info;

mod routes;
mod store;

const IP_ADDRESS: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let app = routes::routes();

    info!("Listening on http://{}", IP_ADDRESS);

    let listener = TcpListener::bind(IP_ADDRESS).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
