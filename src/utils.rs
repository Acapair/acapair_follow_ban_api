use axum_server::tls_rustls::RustlsConfig;
use tokio::{fs::File, io::AsyncReadExt};

use crate::DataBaseConfig;

pub async fn tls_config() -> RustlsConfig {
    RustlsConfig::from_pem_file("certificates/fullchain.pem", "certificates/privkey.pem")
        .await
        .unwrap()
}

pub async fn database_config() -> DataBaseConfig {
    let mut config_file = File::open("configs/databaseconfig").await.unwrap();
    let mut config_unparsed = String::new();
    config_file
        .read_to_string(&mut config_unparsed)
        .await
        .unwrap();

    let configs_parsed: Vec<&str> = config_unparsed.split_terminator("\n").collect();
    let mut configs_cleaned: Vec<&str> = vec![];

    for element in configs_parsed {
        let dirty: Vec<&str> = element.split(": ").collect();
        configs_cleaned.push(dirty[1]);
    }

    DataBaseConfig {
        address: configs_cleaned[0].to_string(),
        username: configs_cleaned[1].to_string(),
        password: configs_cleaned[2].to_string(),
        namespace: configs_cleaned[3].to_string(),
        database: configs_cleaned[4].to_string(),
    }
}
