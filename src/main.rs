// src/main.rs
#[derive(Debug)]
struct DbConfig {
    host: String,
    port: u16,
    username: String,
    password: Option<String>,
    database: String,
    ssl: bool,
}

// El Builder
struct DbConfigBuilder {
    host: String,
    port: u16,
    username: String,
    password: Option<String>,
    database: String,
    ssl: bool,
}

impl DbConfigBuilder {
    fn new() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            username: "root".to_string(),
            password: None,
            database: "default".to_string(),
            ssl: false,
        }
    }

    fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    fn username(mut self, username: &str) -> Self {
        self.username = username.to_string();
        self
    }

    fn password(mut self, password: &str) -> Self {
        self.password = Some(password.to_string());
        self
    }

    fn database(mut self, database: &str) -> Self {
        self.database = database.to_string();
        self
    }

    fn ssl(mut self, enable: bool) -> Self {
        self.ssl = enable;
        self
    }

    fn build(self) -> DbConfig {
        DbConfig {
            host: self.host,
            port: self.port,
            username: self.username,
            password: self.password,
            database: self.database,
            ssl: self.ssl,
        }
    }
}

fn main() {
    // Example 1: Default configuration
    let default_config = DbConfigBuilder::new().build();
    println!("Default config: {:?}", default_config);

    // Example 2: Personalized configuration
    let custom_config = DbConfigBuilder::new()
        .host("db.myserver.com")
        .port(3306)
        .username("saUser")
        .password("secretPassword")
        .database("mi_app")
        .ssl(true)
        .build();

    println!("Custom config: {:?}", custom_config);
}
