use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Schema {
    String,
    Number,
    Boolean,
    Null,
    Any,
    Array(Box<Schema>),
    Object(Vec<(String, Schema)>),
    Enum(Vec<String>), // Simple string enum
    Union(Vec<Schema>),
    Tuple(Vec<Schema>),
    Ref(String), // Reference to another type
    Record {
        key: Box<Schema>,
        value: Box<Schema>,
    }, // For HashMap/Record types
                 // For complex enums, we might need more structure, but let's start simple
                 // or maybe just use a custom "Type" definition
}

pub trait SchemaBridge {
    fn to_ts() -> String;
    fn to_schema() -> Schema;
}

// Implement for basic types
impl SchemaBridge for String {
    fn to_ts() -> String {
        "string".to_string()
    }
    fn to_schema() -> Schema {
        Schema::String
    }
}

impl SchemaBridge for i32 {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Number
    }
}

impl SchemaBridge for f64 {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Number
    }
}

impl SchemaBridge for bool {
    fn to_ts() -> String {
        "boolean".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Boolean
    }
}

// Implement for all integer types
impl SchemaBridge for i8 {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Number
    }
}

impl SchemaBridge for i16 {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Number
    }
}

impl SchemaBridge for i64 {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Number
    }
}

impl SchemaBridge for i128 {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Number
    }
}

impl SchemaBridge for isize {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Number
    }
}

impl SchemaBridge for u8 {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Number
    }
}

impl SchemaBridge for u16 {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Number
    }
}

impl SchemaBridge for u32 {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Number
    }
}

impl SchemaBridge for u64 {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Number
    }
}

impl SchemaBridge for u128 {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Number
    }
}

impl SchemaBridge for usize {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Number
    }
}

impl SchemaBridge for f32 {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Number
    }
}

// Implement for char
impl SchemaBridge for char {
    fn to_ts() -> String {
        "string".to_string()
    }
    fn to_schema() -> Schema {
        Schema::String
    }
}

// Implement for unit type
impl SchemaBridge for () {
    fn to_ts() -> String {
        "null".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Null
    }
}

impl<T: SchemaBridge> SchemaBridge for Option<T> {
    fn to_ts() -> String {
        format!("{} | null", T::to_ts())
    }
    fn to_schema() -> Schema {
        Schema::Union(vec![T::to_schema(), Schema::Null])
    }
}

impl<T: SchemaBridge> SchemaBridge for Vec<T> {
    fn to_ts() -> String {
        format!("{}[]", T::to_ts())
    }
    fn to_schema() -> Schema {
        Schema::Array(Box::new(T::to_schema()))
    }
}

impl SchemaBridge for PathBuf {
    fn to_ts() -> String {
        "string".to_string()
    }
    fn to_schema() -> Schema {
        Schema::String
    }
}

impl<K, V> SchemaBridge for HashMap<K, V>
where
    K: SchemaBridge,
    V: SchemaBridge,
{
    fn to_ts() -> String {
        format!("Record<{}, {}>", K::to_ts(), V::to_ts())
    }
    fn to_schema() -> Schema {
        Schema::Record {
            key: Box::new(K::to_schema()),
            value: Box::new(V::to_schema()),
        }
    }
}

impl<K, V> SchemaBridge for BTreeMap<K, V>
where
    K: SchemaBridge,
    V: SchemaBridge,
{
    fn to_ts() -> String {
        format!("Record<{}, {}>", K::to_ts(), V::to_ts())
    }
    fn to_schema() -> Schema {
        Schema::Record {
            key: Box::new(K::to_schema()),
            value: Box::new(V::to_schema()),
        }
    }
}

impl<T: SchemaBridge> SchemaBridge for HashSet<T> {
    fn to_ts() -> String {
        format!("{}[]", T::to_ts())
    }
    fn to_schema() -> Schema {
        Schema::Array(Box::new(T::to_schema()))
    }
}

impl<T: SchemaBridge> SchemaBridge for BTreeSet<T> {
    fn to_ts() -> String {
        format!("{}[]", T::to_ts())
    }
    fn to_schema() -> Schema {
        Schema::Array(Box::new(T::to_schema()))
    }
}

impl<T: SchemaBridge> SchemaBridge for Box<T> {
    fn to_ts() -> String {
        T::to_ts()
    }
    fn to_schema() -> Schema {
        T::to_schema()
    }
}

impl<T: SchemaBridge> SchemaBridge for Rc<T> {
    fn to_ts() -> String {
        T::to_ts()
    }
    fn to_schema() -> Schema {
        T::to_schema()
    }
}

impl<T: SchemaBridge> SchemaBridge for Arc<T> {
    fn to_ts() -> String {
        T::to_ts()
    }
    fn to_schema() -> Schema {
        T::to_schema()
    }
}

impl<T: SchemaBridge, E: SchemaBridge> SchemaBridge for Result<T, E> {
    fn to_ts() -> String {
        format!("{} | {}", T::to_ts(), E::to_ts())
    }
    fn to_schema() -> Schema {
        Schema::Union(vec![T::to_schema(), E::to_schema()])
    }
}

// Tuple implementations
impl<T: SchemaBridge> SchemaBridge for (T,) {
    fn to_ts() -> String {
        format!("[{}]", T::to_ts())
    }
    fn to_schema() -> Schema {
        Schema::Tuple(vec![T::to_schema()])
    }
}

impl<T1: SchemaBridge, T2: SchemaBridge> SchemaBridge for (T1, T2) {
    fn to_ts() -> String {
        format!("[{}, {}]", T1::to_ts(), T2::to_ts())
    }
    fn to_schema() -> Schema {
        Schema::Tuple(vec![T1::to_schema(), T2::to_schema()])
    }
}

impl<T1: SchemaBridge, T2: SchemaBridge, T3: SchemaBridge> SchemaBridge for (T1, T2, T3) {
    fn to_ts() -> String {
        format!("[{}, {}, {}]", T1::to_ts(), T2::to_ts(), T3::to_ts())
    }
    fn to_schema() -> Schema {
        Schema::Tuple(vec![T1::to_schema(), T2::to_schema(), T3::to_schema()])
    }
}

impl<T1: SchemaBridge, T2: SchemaBridge, T3: SchemaBridge, T4: SchemaBridge> SchemaBridge
    for (T1, T2, T3, T4)
{
    fn to_ts() -> String {
        format!(
            "[{}, {}, {}, {}]",
            T1::to_ts(),
            T2::to_ts(),
            T3::to_ts(),
            T4::to_ts()
        )
    }
    fn to_schema() -> Schema {
        Schema::Tuple(vec![
            T1::to_schema(),
            T2::to_schema(),
            T3::to_schema(),
            T4::to_schema(),
        ])
    }
}

impl<T1: SchemaBridge, T2: SchemaBridge, T3: SchemaBridge, T4: SchemaBridge, T5: SchemaBridge>
    SchemaBridge for (T1, T2, T3, T4, T5)
{
    fn to_ts() -> String {
        format!(
            "[{}, {}, {}, {}, {}]",
            T1::to_ts(),
            T2::to_ts(),
            T3::to_ts(),
            T4::to_ts(),
            T5::to_ts()
        )
    }
    fn to_schema() -> Schema {
        Schema::Tuple(vec![
            T1::to_schema(),
            T2::to_schema(),
            T3::to_schema(),
            T4::to_schema(),
            T5::to_schema(),
        ])
    }
}

impl<
        T1: SchemaBridge,
        T2: SchemaBridge,
        T3: SchemaBridge,
        T4: SchemaBridge,
        T5: SchemaBridge,
        T6: SchemaBridge,
    > SchemaBridge for (T1, T2, T3, T4, T5, T6)
{
    fn to_ts() -> String {
        format!(
            "[{}, {}, {}, {}, {}, {}]",
            T1::to_ts(),
            T2::to_ts(),
            T3::to_ts(),
            T4::to_ts(),
            T5::to_ts(),
            T6::to_ts()
        )
    }
    fn to_schema() -> Schema {
        Schema::Tuple(vec![
            T1::to_schema(),
            T2::to_schema(),
            T3::to_schema(),
            T4::to_schema(),
            T5::to_schema(),
            T6::to_schema(),
        ])
    }
}

// Helper to generate the full TS file content
pub fn generate_ts_file(types: Vec<(&str, String)>) -> String {
    let mut content = String::new();
    content.push_str("// This file is auto-generated by schema-bridge\n\n");

    for (name, ts_def) in types {
        content.push_str(&format!("export type {} = {};\n\n", name, ts_def));
    }

    content
}

/// Export types to a TypeScript file
pub fn export_to_file(types: Vec<(&str, String)>, path: &str) -> std::io::Result<()> {
    let content = generate_ts_file(types);
    std::fs::write(path, content)
}

/// Macro to easily export types to a file
#[macro_export]
macro_rules! export_types {
    ($path:expr, $($name:ident),+ $(,)?) => {{
        let types = vec![
            $((stringify!($name), $name::to_ts()),)+
        ];
        $crate::export_to_file(types, $path)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_ts() {
        assert_eq!(String::to_ts(), "string");
    }

    #[test]
    fn test_i32_to_ts() {
        assert_eq!(i32::to_ts(), "number");
    }

    #[test]
    fn test_f64_to_ts() {
        assert_eq!(f64::to_ts(), "number");
    }

    #[test]
    fn test_bool_to_ts() {
        assert_eq!(bool::to_ts(), "boolean");
    }

    #[test]
    fn test_option_to_ts() {
        assert_eq!(Option::<String>::to_ts(), "string | null");
        assert_eq!(Option::<i32>::to_ts(), "number | null");
    }

    #[test]
    fn test_vec_to_ts() {
        assert_eq!(Vec::<String>::to_ts(), "string[]");
        assert_eq!(Vec::<i32>::to_ts(), "number[]");
    }

    #[test]
    fn test_nested_vec() {
        assert_eq!(Vec::<Vec::<String>>::to_ts(), "string[][]");
    }

    #[test]
    fn test_optional_vec() {
        assert_eq!(Option::<Vec::<String>>::to_ts(), "string[] | null");
    }

    #[test]
    fn test_generate_ts_file() {
        let types = vec![
            ("User", "{ name: string; age: number; }".to_string()),
            ("Status", "'Active' | 'Inactive'".to_string()),
        ];

        let result = generate_ts_file(types);

        assert!(result.contains("// This file is auto-generated by schema-bridge"));
        assert!(result.contains("export type User = { name: string; age: number; };"));
        assert!(result.contains("export type Status = 'Active' | 'Inactive';"));
    }

    #[test]
    fn test_schema_enum() {
        let schema = Schema::String;
        assert_eq!(schema, Schema::String);

        let schema = Schema::Array(Box::new(Schema::Number));
        assert!(matches!(schema, Schema::Array(_)));
    }

    #[test]
    fn test_pathbuf_to_ts() {
        assert_eq!(PathBuf::to_ts(), "string");
    }

    #[test]
    fn test_pathbuf_to_schema() {
        assert_eq!(PathBuf::to_schema(), Schema::String);
    }

    #[test]
    fn test_hashmap_to_ts() {
        assert_eq!(HashMap::<String, i32>::to_ts(), "Record<string, number>");
        assert_eq!(HashMap::<String, String>::to_ts(), "Record<string, string>");
    }

    #[test]
    fn test_hashmap_to_schema() {
        let schema = HashMap::<String, i32>::to_schema();
        assert!(matches!(schema, Schema::Record { .. }));
        if let Schema::Record { key, value } = schema {
            assert_eq!(*key, Schema::String);
            assert_eq!(*value, Schema::Number);
        }
    }

    #[test]
    fn test_nested_hashmap() {
        assert_eq!(
            HashMap::<String, Vec::<String>>::to_ts(),
            "Record<string, string[]>"
        );
    }

    #[test]
    fn test_optional_hashmap() {
        assert_eq!(
            Option::<HashMap::<String, i32>>::to_ts(),
            "Record<string, number> | null"
        );
    }

    // Test numeric types
    #[test]
    fn test_numeric_types() {
        assert_eq!(i8::to_ts(), "number");
        assert_eq!(i16::to_ts(), "number");
        assert_eq!(i64::to_ts(), "number");
        assert_eq!(i128::to_ts(), "number");
        assert_eq!(isize::to_ts(), "number");
        assert_eq!(u8::to_ts(), "number");
        assert_eq!(u16::to_ts(), "number");
        assert_eq!(u32::to_ts(), "number");
        assert_eq!(u64::to_ts(), "number");
        assert_eq!(u128::to_ts(), "number");
        assert_eq!(usize::to_ts(), "number");
        assert_eq!(f32::to_ts(), "number");
    }

    #[test]
    fn test_char_to_ts() {
        assert_eq!(char::to_ts(), "string");
        assert_eq!(char::to_schema(), Schema::String);
    }

    #[test]
    fn test_unit_to_ts() {
        assert_eq!(<()>::to_ts(), "null");
        assert_eq!(<()>::to_schema(), Schema::Null);
    }

    // Test BTreeMap
    #[test]
    fn test_btreemap_to_ts() {
        assert_eq!(BTreeMap::<String, i32>::to_ts(), "Record<string, number>");
    }

    // Test HashSet and BTreeSet
    #[test]
    fn test_hashset_to_ts() {
        assert_eq!(HashSet::<String>::to_ts(), "string[]");
        assert_eq!(HashSet::<i32>::to_ts(), "number[]");
    }

    #[test]
    fn test_btreeset_to_ts() {
        assert_eq!(BTreeSet::<String>::to_ts(), "string[]");
        assert_eq!(BTreeSet::<i32>::to_ts(), "number[]");
    }

    // Test smart pointers
    #[test]
    fn test_box_to_ts() {
        assert_eq!(Box::<String>::to_ts(), "string");
        assert_eq!(Box::<i32>::to_ts(), "number");
        assert_eq!(Box::<String>::to_schema(), Schema::String);
    }

    #[test]
    fn test_rc_to_ts() {
        assert_eq!(Rc::<String>::to_ts(), "string");
        assert_eq!(Rc::<i32>::to_ts(), "number");
    }

    #[test]
    fn test_arc_to_ts() {
        assert_eq!(Arc::<String>::to_ts(), "string");
        assert_eq!(Arc::<i32>::to_ts(), "number");
    }

    // Test Result
    #[test]
    fn test_result_to_ts() {
        assert_eq!(Result::<String, String>::to_ts(), "string | string");
        assert_eq!(Result::<i32, String>::to_ts(), "number | string");
    }

    #[test]
    fn test_result_to_schema() {
        let schema = Result::<String, i32>::to_schema();
        assert!(matches!(schema, Schema::Union(_)));
        if let Schema::Union(types) = schema {
            assert_eq!(types.len(), 2);
            assert_eq!(types[0], Schema::String);
            assert_eq!(types[1], Schema::Number);
        }
    }

    // Test tuples
    #[test]
    fn test_tuple_1() {
        assert_eq!(<(String,)>::to_ts(), "[string]");
        let schema = <(String,)>::to_schema();
        assert!(matches!(schema, Schema::Tuple(_)));
    }

    #[test]
    fn test_tuple_2() {
        assert_eq!(<(String, i32)>::to_ts(), "[string, number]");
    }

    #[test]
    fn test_tuple_3() {
        assert_eq!(<(String, i32, bool)>::to_ts(), "[string, number, boolean]");
    }

    #[test]
    fn test_tuple_4() {
        assert_eq!(
            <(String, i32, bool, f64)>::to_ts(),
            "[string, number, boolean, number]"
        );
    }

    #[test]
    fn test_tuple_schema() {
        let schema = <(String, i32)>::to_schema();
        if let Schema::Tuple(types) = schema {
            assert_eq!(types.len(), 2);
            assert_eq!(types[0], Schema::String);
            assert_eq!(types[1], Schema::Number);
        } else {
            panic!("Expected Tuple schema");
        }
    }

    // Test complex combinations
    #[test]
    fn test_complex_types() {
        assert_eq!(Option::<Box::<String>>::to_ts(), "string | null");
        assert_eq!(Vec::<Arc::<String>>::to_ts(), "string[]");
        assert_eq!(
            HashMap::<String, Vec::<i32>>::to_ts(),
            "Record<string, number[]>"
        );
    }
}
