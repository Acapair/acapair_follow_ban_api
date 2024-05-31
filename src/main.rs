use acapair_follow_ban_api::{
    db::db_operations::connect,
    routing,
    utils::{database_config, server_config, tls_config},
    AppState,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let tls_config = tls_config().await;
    let database_config = database_config().await;
    let server_config = server_config().await;
    println!("{:#?}", database_config);

    let state = AppState {
        db: connect(&database_config).await.unwrap(),
    };

    let app = routing::routing(axum::extract::State(state)).await;
    let addr = SocketAddr::new(server_config.ip_address, server_config.port);
    println!(
        "\n\n\tOn Air -> https://{}\n\n",
        format!("{}:{}", server_config.ip_address, server_config.port)
    );
    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
