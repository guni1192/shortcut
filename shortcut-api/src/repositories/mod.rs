pub mod links;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::env;

pub struct MySqlClient {
    host: String,
    port: String,
    user: String,
    password: String,
    database_name: String,
}

impl MySqlClient {
    pub fn from_env() -> Self {
        let host = env::var("MYSQL_HOST").expect("MYSQL_HOST is not set");
        let port = env::var("MYSQL_PORT").expect("MYSQL_PORT is not set");
        let user = env::var("MYSQL_USER").expect("MYSQL_USER is not set");
        let password = env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD is not set");
        let database_name = env::var("MYSQL_DATABASE").expect("MYSQL_DATABASE is not set");
        Self {
            host,
            port,
            user,
            password,
            database_name,
        }
    }

    fn get_url(&self) -> String {
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database_name
        );
        url
    }

    pub async fn connect(&self) -> Result<Pool<MySql>, sqlx::Error> {
        let url = self.get_url();

        let pool_opts = MySqlPoolOptions::new();
        pool_opts.connect(&url).await
    }
}
