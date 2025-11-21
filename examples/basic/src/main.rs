use schema_bridge::{SchemaBridge, export_types};
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

fn main() {
    // Simple way: use the export_types! macro
    export_types!("bindings.ts", MyStruct, MyEnum).expect("Failed to write TS file");
    
    println!("TypeScript definitions written to bindings.ts");
}
