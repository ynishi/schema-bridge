use schema_bridge::SchemaBridge;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SchemaBridge, Debug, PartialEq)]
#[schema_bridge(string_conversion)]
#[serde(rename_all = "snake_case")]
enum TalkStyle {
    Brainstorm,
    Casual,
    DecisionMaking,
}

#[derive(Serialize, Deserialize, SchemaBridge, Debug, PartialEq)]
#[schema_bridge(string_conversion)]
#[serde(rename_all = "PascalCase")]
enum Status {
    Active,
    Pending,
    Completed,
}

#[derive(Serialize, Deserialize, SchemaBridge, Debug, PartialEq)]
#[schema_bridge(string_conversion)]
#[serde(rename_all = "camelCase")]
enum Priority {
    HighPriority,
    LowPriority,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_snake_case() {
        assert_eq!(TalkStyle::Brainstorm.to_string(), "brainstorm");
        assert_eq!(TalkStyle::Casual.to_string(), "casual");
        assert_eq!(TalkStyle::DecisionMaking.to_string(), "decision_making");
    }

    #[test]
    fn test_fromstr_snake_case() {
        assert_eq!("brainstorm".parse::<TalkStyle>().unwrap(), TalkStyle::Brainstorm);
        assert_eq!("casual".parse::<TalkStyle>().unwrap(), TalkStyle::Casual);
        assert_eq!("decision_making".parse::<TalkStyle>().unwrap(), TalkStyle::DecisionMaking);
    }

    #[test]
    fn test_fromstr_error() {
        let result = "invalid".parse::<TalkStyle>();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unknown TalkStyle: invalid");
    }

    #[test]
    fn test_display_pascal_case() {
        assert_eq!(Status::Active.to_string(), "Active");
        assert_eq!(Status::Pending.to_string(), "Pending");
        assert_eq!(Status::Completed.to_string(), "Completed");
    }

    #[test]
    fn test_fromstr_pascal_case() {
        assert_eq!("Active".parse::<Status>().unwrap(), Status::Active);
        assert_eq!("Pending".parse::<Status>().unwrap(), Status::Pending);
        assert_eq!("Completed".parse::<Status>().unwrap(), Status::Completed);
    }

    #[test]
    fn test_display_camel_case() {
        assert_eq!(Priority::HighPriority.to_string(), "highPriority");
        assert_eq!(Priority::LowPriority.to_string(), "lowPriority");
    }

    #[test]
    fn test_fromstr_camel_case() {
        assert_eq!("highPriority".parse::<Priority>().unwrap(), Priority::HighPriority);
        assert_eq!("lowPriority".parse::<Priority>().unwrap(), Priority::LowPriority);
    }

    #[test]
    fn test_roundtrip() {
        let style = TalkStyle::DecisionMaking;
        let as_string = style.to_string();
        let parsed: TalkStyle = as_string.parse().unwrap();
        assert_eq!(parsed, style);
    }
}
