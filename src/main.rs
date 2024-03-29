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

    let listener = TcpListener::bind(IP_ADDRESS).await.unwrap();

    info!("Listening on http://{}", IP_ADDRESS);

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
