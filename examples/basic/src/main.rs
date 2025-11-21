use schema_bridge::{generate_ts_file, SchemaBridge};
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
    let types = vec![("MyStruct", MyStruct::to_ts()), ("MyEnum", MyEnum::to_ts())];

    let ts_content = generate_ts_file(types);
    println!("{}", ts_content);
}
