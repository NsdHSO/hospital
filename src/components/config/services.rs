fn get_env_var(var_name: &str) -> String {
    std::env::var(var_name).unwrap_or_else(|_| panic!("{} must be set", var_name))
}

#[derive(Debug, Clone)]
pub struct ConfigService {
    pub rust_log: String,
    pub schema_synchronize: bool,
    pub host: String,
    pub port: u16,
    pub app_env: String,
    pub database_url: String,
    pub prod_database_url: String,
    pub database_url_uat: String,
    pub synchronize: bool,
    pub auto_migrate: bool,
    pub auth_base_url: String,
    pub access_token_public_key: String,
}

impl ConfigService {
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