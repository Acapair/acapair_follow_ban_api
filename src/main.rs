use acapair_follow_ban_api::{
    db::db_operations::connect,
    routing,
    utils::{database_config, tls_config},
    AppState,
};
use std::{env, net::SocketAddr};

fn take_args() -> String {
    let mut bind_address: String = String::new();
    for element in env::args() {
        bind_address = element;
    }
    println!("\n\n\tOn Air -> https://{}\n\n", bind_address);
    bind_address
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let tls_config = tls_config().await;
    let database_config = database_config().await;

    println!("{:#?}", database_config);

    let state = AppState {
        db: connect(&database_config).await.unwrap(),
    };

    let app = routing::routing(axum::extract::State(state)).await;
    let addr = SocketAddr::from(take_args().parse::<SocketAddr>().unwrap());

    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
