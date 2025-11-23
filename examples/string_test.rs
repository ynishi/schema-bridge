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

fn main() {
    // Display test
    let style = TalkStyle::Brainstorm;
    println!("Display: {}", style);
    println!("to_string(): {}", style.to_string());
    
    // FromStr test
    let parsed: TalkStyle = "casual".parse().unwrap();
    println!("Parsed 'casual': {:?}", parsed);
    
    let parsed2: TalkStyle = "decision_making".parse().unwrap();
    println!("Parsed 'decision_making': {:?}", parsed2);
    
    // Error test
    match "invalid".parse::<TalkStyle>() {
        Ok(_) => println!("Should not reach here"),
        Err(e) => println!("Error (expected): {}", e),
    }
}
