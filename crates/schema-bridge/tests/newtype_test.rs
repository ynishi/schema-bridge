use schema_bridge::SchemaBridge;
use serde::{Deserialize, Serialize};

// Simulate an external type
#[derive(Serialize, Deserialize, SchemaBridge)]
enum ExternalStatus {
    Active,
    Inactive,
    Pending,
}

// Newtype pattern - wrapping external type
#[derive(Serialize, Deserialize, SchemaBridge)]
struct MyStatus(ExternalStatus);

// Another newtype example
#[derive(Serialize, Deserialize, SchemaBridge)]
struct UserId(String);

// Multi-field tuple struct
#[derive(Serialize, Deserialize, SchemaBridge)]
struct Coordinates(f64, f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_newtype_enum() {
        let ts = MyStatus::to_ts();
        // Should delegate to ExternalStatus
        assert!(ts.contains("'Active'"));
        assert!(ts.contains("'Inactive'"));
        assert!(ts.contains("'Pending'"));
    }

    #[test]
    fn test_newtype_string() {
        let ts = UserId::to_ts();
        assert_eq!(ts, "string");
    }

    #[test]
    fn test_tuple_struct() {
        let ts = Coordinates::to_ts();
        assert!(ts.contains("number"));
        assert!(ts.contains(","));
    }
}
