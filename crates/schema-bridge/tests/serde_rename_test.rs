use schema_bridge::SchemaBridge;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SchemaBridge)]
#[serde(rename_all = "snake_case")]
enum ConversationMode {
    Normal,
    Concise,
    Creative,
    Academic,
}

#[derive(Serialize, Deserialize, SchemaBridge)]
#[serde(rename_all = "camelCase")]
enum Status {
    Active,
    Pending,
    Complete,
}

#[derive(Serialize, Deserialize, SchemaBridge)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum Priority {
    LowPriority,
    HighPriority,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snake_case_rename() {
        let ts = ConversationMode::to_ts();
        assert!(ts.contains("'normal'"));
        assert!(ts.contains("'concise'"));
        assert!(ts.contains("'creative'"));
        assert!(ts.contains("'academic'"));
        assert!(!ts.contains("Normal")); // Should not have PascalCase
    }

    #[test]
    fn test_camel_case_rename() {
        let ts = Status::to_ts();
        assert!(ts.contains("'active'"));
        assert!(ts.contains("'pending'"));
        assert!(ts.contains("'complete'"));
    }

    #[test]
    fn test_screaming_snake_case_rename() {
        let ts = Priority::to_ts();
        assert!(ts.contains("'LOW_PRIORITY'"));
        assert!(ts.contains("'HIGH_PRIORITY'"));
    }
}
