use schema_bridge::SchemaBridge;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SchemaBridge)]
#[serde(rename_all = "camelCase")]
struct UserConfig {
    user_name: String,
    max_retries: i32,
    is_active: bool,
}

#[derive(Serialize, Deserialize, SchemaBridge)]
#[serde(rename_all = "snake_case")]
struct ApiResponse {
    status_code: i32,
    error_message: Option<String>,
}

#[derive(Serialize, Deserialize, SchemaBridge)]
#[serde(rename_all = "PascalCase")]
struct SystemSettings {
    database_url: String,
    cache_enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_camel_case_rename() {
        let ts = UserConfig::to_ts();
        println!("UserConfig TS: {}", ts);
        assert!(ts.contains("userName:"), "Should contain 'userName:'");
        assert!(ts.contains("maxRetries:"), "Should contain 'maxRetries:'");
        assert!(ts.contains("isActive:"), "Should contain 'isActive:'");
        assert!(
            !ts.contains("user_name:"),
            "Should not contain 'user_name:'"
        );
    }

    #[test]
    fn test_struct_snake_case_rename() {
        let ts = ApiResponse::to_ts();
        println!("ApiResponse TS: {}", ts);
        // When fields are already in snake_case, rename_all = "snake_case" keeps them as-is
        assert!(ts.contains("status_code:"), "Should contain 'status_code:'");
        assert!(
            ts.contains("error_message:"),
            "Should contain 'error_message:'"
        );
    }

    #[test]
    fn test_struct_pascal_case_rename() {
        let ts = SystemSettings::to_ts();
        println!("SystemSettings TS: {}", ts);
        assert!(ts.contains("DatabaseUrl:"), "Should contain 'DatabaseUrl:'");
        assert!(
            ts.contains("CacheEnabled:"),
            "Should contain 'CacheEnabled:'"
        );
        assert!(
            !ts.contains("database_url:"),
            "Should not contain 'database_url:'"
        );
    }
}
