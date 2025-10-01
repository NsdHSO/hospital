use doppler_rs::apis::{configuration::Configuration, default_api};

#[derive(Debug, Clone)]
pub struct ConfigService {
    pub rust_log: String,
    pub host: String,
    pub port: u16,
    pub app_env: String,
    pub database_url: String,
    pub prod_database_url: String,
    pub database_url_uat: String,
    pub auth_base_url: String,
    pub access_token_public_key: String,
}

impl ConfigService {
    /// Creates a new `ConfigService` instance by loading configuration from environment variables.
    ///
    /// This function retrieves and parses the necessary environment variables to populate the
    /// `ConfigService` struct.
    ///
    /// # Panics
    ///
    /// This method will panic if:
    ///
    /// * Any of the required environment variables are not set.
    /// * `PORT` cannot be parsed into a `u16`.
    /// * `SCHEMA_SYNCHRONIZE`, `SYNCHRONIZE`, or `AUTO_MIGRATE` cannot be parsed into a `bool`.
    ///
    /// # Examples
    ///
    /// ```
    /// // Example of setting environment variables before running the application.
    /// std::env::set_var("RUST_LOG", "info");
    /// std::env::set_var("HOST", "127.0.0.1");
    /// std::env::set_var("PORT", "8080");
    /// std::env::set_var("SCHEMA_SYNCHRONIZE", "true");
    /// std::env::set_var("APP_ENV", "dev");
    /// std::env::set_var("DATABASE_URL", "postgres://user:pass@host/db");
    /// std::env::set_var("PROD_DATABASE_URL", "postgres://user:pass@host/prod_db");
    /// std::env::set_var("DATABASE_URL_UAT", "postgres://user:pass@host/uat_db");
    /// std::env::set_var("SYNCHRONIZE", "true");
    /// std::env::set_var("AUTO_MIGRATE", "true");
    /// std::env::set_var("AUTH_BASE_URL", "[http://auth.service.com](http://auth.service.com)");
    /// std::env::set_var("ACCESS_TOKEN_PUBLIC_KEY", "your-public-key");
    ///
    /// // Create a new instance of the ConfigService
    /// let config = ConfigService::new();
    ///
    /// assert_eq!(config.port, 8080);
    /// assert_eq!(config.host, "127.0.0.1");
    /// ```
    pub async fn new() -> Self {

        let mut config = Configuration::new();
        config.bearer_access_token = Some(
            std::env::var("DOPPLER_TOKEN")
                .expect("DOPPLER_TOKEN must be set"),
        );

        let project = "tevet-troc";
        let config_name = "dev";
        
        // Fetch all secrets concurrently - clone config for each future
        let config_clone = config.clone();
        let rust_log_future = async {
            let secret = default_api::secrets_get(&config_clone, project, config_name, "RUST_LOG").await
                .expect("Failed to get RUST_LOG from Doppler");
            secret.value.as_ref().map(|v| v.computed.clone())
                .expect("RUST_LOG value not found in Doppler")
        };

        let config_clone = config.clone();
        let host_future = async {
            let secret = default_api::secrets_get(&config_clone, project, config_name, "HOST").await
                .expect("Failed to get HOST from Doppler");
            secret.value.as_ref().map(|v| v.computed.clone())
                .expect("HOST value not found in Doppler")
        };

        let config_clone = config.clone();
        let port_future = async {
            let secret = default_api::secrets_get(&config_clone, project, config_name, "PORT").await
                .expect("Failed to get PORT from Doppler");
            secret.value.as_ref().map(|v| v.computed.clone())
                .expect("PORT value not found in Doppler")
        };

        let config_clone = config.clone();
        let app_env_future = async {
            let secret = default_api::secrets_get(&config_clone, project, config_name, "APP_ENV").await
                .expect("Failed to get APP_ENV from Doppler");
            secret.value.as_ref().map(|v| v.computed.clone())
                .expect("APP_ENV value not found in Doppler")
        };

        let config_clone = config.clone();
        let database_url_future = async {
            let secret = default_api::secrets_get(&config_clone, project, config_name, "DATABASE_URL").await
                .expect("Failed to get DATABASE_URL from Doppler");
            secret.value.as_ref().map(|v| v.computed.clone())
                .expect("DATABASE_URL value not found in Doppler")
        };

        let config_clone = config.clone();
        let prod_db_future = async {
            let secret = default_api::secrets_get(&config_clone, project, config_name, "PROD_DATABASE_URL").await
                .expect("Failed to get PROD_DATABASE_URL from Doppler");
            secret.value.as_ref().map(|v| v.computed.clone())
                .expect("PROD_DATABASE_URL value not found in Doppler")
        };

        let config_clone = config.clone();
        let uat_db_future = async {
            let secret = default_api::secrets_get(&config_clone, project, config_name, "DATABASE_URL_UAT").await
                .expect("Failed to get DATABASE_URL_UAT from Doppler");
            secret.value.as_ref().map(|v| v.computed.clone())
                .expect("DATABASE_URL_UAT value not found in Doppler")
        };


        let config_clone = config.clone();
        let auth_url_future = async {
            let secret = default_api::secrets_get(&config_clone, project, config_name, "AUTH_BASE_URL").await
                .expect("Failed to get AUTH_BASE_URL from Doppler");
            secret.value.as_ref().map(|v| v.computed.clone())
                .expect("AUTH_BASE_URL value not found in Doppler")
        };

        let config_clone = config.clone();
        let access_token_future = async {
            let secret = default_api::secrets_get(&config_clone, project, config_name, "ACCESS_TOKEN_PUBLIC_KEY").await
                .expect("Failed to get ACCESS_TOKEN_PUBLIC_KEY from Doppler");
            secret.value.as_ref().map(|v| v.computed.clone())
                .expect("ACCESS_TOKEN_PUBLIC_KEY value not found in Doppler")
        };

        let (
            rust_log,
            host,
            port_str,
            app_env,
            database_url,
            prod_database_url,
            database_url_uat,
            auth_base_url,
            access_token_public_key
        ) = tokio::join!(
            rust_log_future,
            host_future,
            port_future,
            app_env_future,
            database_url_future,
            prod_db_future,
            uat_db_future,
            auth_url_future,
            access_token_future
        );


        ConfigService {
            rust_log: rust_log.unwrap(),
            host: host.unwrap(),
            port: port_str.expect("PORT NOT FOUND").parse::<u16>().unwrap(),
            app_env: app_env.unwrap(),
            database_url: database_url.unwrap(),
            prod_database_url: prod_database_url.unwrap(),
            database_url_uat: database_url_uat.unwrap(),
            auth_base_url: auth_base_url.unwrap(),
            access_token_public_key:access_token_public_key.unwrap()
        }
    }
}