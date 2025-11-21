use schema_bridge::{export_types, SchemaBridge};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SchemaBridge)]
struct MyStruct {
    name: String,
    age: i32,
    is_active: bool,
    tags: Vec<String>,
    meta: Option<String>,
}

#[derive(Serialize, Deserialize, SchemaBridge)]
enum MyEnum {
    OptionA,
    OptionB,
}

// Newtype pattern - wrapping external types
#[derive(Serialize, Deserialize, SchemaBridge)]
struct UserId(String);

#[derive(Serialize, Deserialize, SchemaBridge)]
enum ExternalStatus {
    Active,
    Inactive,
}

// Wrapping external enum
#[derive(Serialize, Deserialize, SchemaBridge)]
struct AppStatus(ExternalStatus);

fn main() {
    // Simple way: use the export_types! macro
    export_types!(
        "bindings.ts",
        MyStruct,
        MyEnum,
        UserId,
        ExternalStatus,
        AppStatus
    )
    .expect("Failed to write TS file");

    println!("TypeScript definitions written to bindings.ts");
}
