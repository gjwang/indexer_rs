use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    server: Server,
    database: Database,
    logging: Logging,
    features: Features,
    api: Api,
    users: Users,
    paths: Paths,
}

#[derive(Debug, Deserialize)]
struct Server {
    host: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
struct Database {
    db_type: String,
    username: String,
    password: String,
    host: String,
    port: u16,
    dbname: String,
}

#[derive(Debug, Deserialize)]
struct Logging {
    level: String,
    file_path: String,
}

#[derive(Debug, Deserialize)]
struct Features {
    enable_feature_x: bool,
    enable_feature_y: bool,
}

#[derive(Debug, Deserialize)]
struct Api {
    version: String,
    timeout_seconds: u32,
}

#[derive(Debug, Deserialize)]
struct Users {
    default_role: String,
    max_users: usize,
}

#[derive(Debug, Deserialize)]
struct Paths {
    data_directory: String,
    temp_directory: String,
}
