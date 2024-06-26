pub struct Config {
    pub db_url: String,
    pub host: String,
    pub port: u32,
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            db_url: std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file"),
            host: std::env::var("HOST").expect("HOST is not set in .env file"),
            port: std::env::var("PORT")
                .expect("PORT is not set in .env file")
                .parse()
                .expect("PORT is not a number"),
        }
    }

    pub fn get_server_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
