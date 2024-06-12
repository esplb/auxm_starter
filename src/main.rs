use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(index))
        .layer(tower_http::trace::TraceLayer::new_for_http());
    let addr="0.0.0.0:3000";
    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> &'static str {
    "Hello, World!"
}
