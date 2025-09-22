/// This function retrieves the value of an environment variable and panics if it is not set.
///
/// # Panics
///
/// This function will panic if the specified environment variable `var_name` is not set.
fn get_env_var(var_name: &str) -> String {
    std::env::var(var_name).unwrap_or_else(|_| panic!("{} must be set", var_name))
}

/// A struct that holds the application's configuration settings.
///
/// This struct is used to manage all environment-specific variables
/// and application settings in a centralized location.
#[derive(Debug, Clone)]
pub struct ConfigService {
    /// The logging level for the application, typically set via the `RUST_LOG` environment variable.
    pub rust_log: String,
    /// A boolean indicating whether the database schema should be synchronized on startup.
    /// This is typically used in development environments.
    pub schema_synchronize: bool,
    /// The host address the application will bind to.
    pub host: String,
    /// The network port the application will listen on.
    pub port: u16,
    /// The current application environment, such as "development," "production," or "UAT."
    pub app_env: String,
    /// The URL for the primary database connection.
    pub database_url: String,
    /// The database URL for the production environment.
    pub prod_database_url: String,
    /// The database URL for the UAT (User Acceptance Testing) environment.
    pub database_url_uat: String,
    /// A boolean to control database synchronization.
    pub synchronize: bool,
    /// A boolean to enable or disable automatic database migrations on startup.
    pub auto_migrate: bool,
    /// The base URL for the authentication service.
    pub auth_base_url: String,
    /// The public key used to verify and validate access tokens.
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
    pub fn new() -> Self {
        let rust_log = get_env_var("RUST_LOG");
        let schema_synchronize = get_env_var("SCHEMA_SYNCHRONIZE").parse::<bool>().unwrap();
        let host = get_env_var("HOST");
        let port = get_env_var("PORT").parse::<u16>().unwrap();
        let app_env = get_env_var("APP_ENV");
        let database_url = get_env_var("DATABASE_URL");
        let prod_database_url = get_env_var("PROD_DATABASE_URL");
        let database_url_uat = get_env_var("DATABASE_URL_UAT");
        let synchronize = get_env_var("SYNCHRONIZE").parse::<bool>().unwrap();
        let auto_migrate = get_env_var("AUTO_MIGRATE").parse::<bool>().unwrap();
        let auth_base_url = get_env_var("AUTH_BASE_URL");
        let access_token_public_key = get_env_var("ACCESS_TOKEN_PUBLIC_KEY");

        ConfigService {
            rust_log,
            schema_synchronize,
            host,
            port,
            app_env,
            database_url,
            prod_database_url,
            database_url_uat,
            synchronize,
            auto_migrate,
            auth_base_url,
            access_token_public_key
        }
    }
}