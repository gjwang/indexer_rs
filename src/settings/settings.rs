use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: Server,
    pub database: Database,
    pub logging: Logging,
    pub features: Features,
    pub api: Api,
    pub users: Users,
    pub paths: Paths,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    host: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    db_type: String,
    username: String,
    password: String,
    host: String,
    port: u16,
    dbname: String,
}

#[derive(Debug, Deserialize)]
pub struct Logging {
    level: String,
    file_path: String,
}

#[derive(Debug, Deserialize)]
pub struct Features {
    enable_feature_x: bool,
    enable_feature_y: bool,
}

#[derive(Debug, Deserialize)]
pub struct Api {
    version: String,
    timeout_seconds: u32,
}

#[derive(Debug, Deserialize)]
pub struct Users {
    default_role: String,
    max_users: usize,
}

#[derive(Debug, Deserialize)]
pub struct Paths {
    data_directory: String,
    temp_directory: String,
}
