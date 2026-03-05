use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Schema {
    String,
    Number,
    Integer,
    Boolean,
    Null,
    Any,
    Array(Box<Schema>),
    Object(Vec<Field>),
    Enum(Vec<String>),
    Union(Vec<Schema>),
    Tuple(Vec<Schema>),
    Ref(String),
    Record {
        key: Box<Schema>,
        value: Box<Schema>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field {
    pub name: String,
    pub schema: Schema,
    pub required: bool,
    pub constraints: Constraints,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Constraints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_len: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_len: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_of: Option<Vec<String>>,
}

impl Field {
    pub fn new(name: impl Into<String>, schema: Schema) -> Self {
        Self {
            name: name.into(),
            schema,
            required: true,
            constraints: Constraints::default(),
        }
    }

    pub fn optional(name: impl Into<String>, schema: Schema) -> Self {
        Self {
            name: name.into(),
            schema,
            required: false,
            constraints: Constraints::default(),
        }
    }
}

impl Schema {
    pub fn type_name(&self) -> &'static str {
        match self {
            Schema::String => "string",
            Schema::Number => "number",
            Schema::Integer => "integer",
            Schema::Boolean => "boolean",
            Schema::Null => "nil",
            Schema::Any => "any",
            Schema::Array(_) => "table",
            Schema::Object(_) => "table",
            Schema::Enum(_) => "string",
            Schema::Union(_) => "any",
            Schema::Tuple(_) => "table",
            Schema::Ref(_) => "table",
            Schema::Record { .. } => "table",
        }
    }
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
        Schema::Integer
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

impl SchemaBridge for i8 {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Integer
    }
}

impl SchemaBridge for i16 {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Integer
    }
}

impl SchemaBridge for i64 {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Integer
    }
}

impl SchemaBridge for i128 {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Integer
    }
}

impl SchemaBridge for isize {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Integer
    }
}

impl SchemaBridge for u8 {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Integer
    }
}

impl SchemaBridge for u16 {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Integer
    }
}

impl SchemaBridge for u32 {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Integer
    }
}

impl SchemaBridge for u64 {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Integer
    }
}

impl SchemaBridge for u128 {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Integer
    }
}

impl SchemaBridge for usize {
    fn to_ts() -> String {
        "number".to_string()
    }
    fn to_schema() -> Schema {
        Schema::Integer
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

// --- mlua integration ---

#[cfg(feature = "mlua")]
mod lua {
    use super::*;
    use mlua::prelude::*;

    impl Schema {
        /// Convert this schema to a Lua table compatible with
        /// `mlua_batteries::validate::check()`.
        ///
        /// For `Schema::Object`, produces a table where each key maps to
        /// either a type-name string (shorthand) or a full constraint table.
        pub fn to_lua_table(&self, lua: &Lua) -> LuaResult<LuaValue> {
            match self {
                Schema::Object(fields) => {
                    let t = lua.create_table()?;
                    for field in fields {
                        let value = field_to_lua_value(lua, field)?;
                        t.set(field.name.as_str(), value)?;
                    }
                    Ok(LuaValue::Table(t))
                }
                _ => {
                    // Non-object schemas: return the type name string
                    Ok(LuaValue::String(lua.create_string(self.type_name())?))
                }
            }
        }
    }

    fn field_to_lua_value(lua: &Lua, field: &Field) -> LuaResult<LuaValue> {
        let has_constraints = field.constraints.min.is_some()
            || field.constraints.max.is_some()
            || field.constraints.min_len.is_some()
            || field.constraints.max_len.is_some()
            || field.constraints.one_of.is_some();

        // Use shorthand format when: not required AND no constraints
        // (shorthand means the field is optional with just a type check)
        if !field.required && !has_constraints {
            return Ok(LuaValue::String(
                lua.create_string(field.schema.type_name())?,
            ));
        }

        // Full format: { type = "...", required = true/false, ... }
        let t = lua.create_table()?;
        t.set("type", field.schema.type_name())?;

        if field.required {
            t.set("required", true)?;
        }

        if let Some(min) = field.constraints.min {
            t.set("min", min)?;
        }
        if let Some(max) = field.constraints.max {
            t.set("max", max)?;
        }
        if let Some(min_len) = field.constraints.min_len {
            t.set("min_len", min_len as i64)?;
        }
        if let Some(max_len) = field.constraints.max_len {
            t.set("max_len", max_len as i64)?;
        }
        if let Some(ref one_of) = field.constraints.one_of {
            let arr = lua.create_table()?;
            for (i, val) in one_of.iter().enumerate() {
                arr.set(i + 1, val.as_str())?;
            }
            t.set("one_of", arr)?;
        }

        Ok(LuaValue::Table(t))
    }
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
    fn test_integer_schema() {
        assert_eq!(i32::to_schema(), Schema::Integer);
        assert_eq!(u64::to_schema(), Schema::Integer);
        assert_eq!(i8::to_schema(), Schema::Integer);
        assert_eq!(usize::to_schema(), Schema::Integer);
    }

    #[test]
    fn test_float_schema() {
        assert_eq!(f32::to_schema(), Schema::Number);
        assert_eq!(f64::to_schema(), Schema::Number);
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
            assert_eq!(*value, Schema::Integer);
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
            assert_eq!(types[1], Schema::Integer);
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
            assert_eq!(types[1], Schema::Integer);
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

    // Test Field and Constraints
    #[test]
    fn test_field_new() {
        let f = Field::new("name", Schema::String);
        assert_eq!(f.name, "name");
        assert!(f.required);
        assert_eq!(f.constraints, Constraints::default());
    }

    #[test]
    fn test_field_optional() {
        let f = Field::optional("email", Schema::String);
        assert!(!f.required);
    }

    #[test]
    fn test_schema_type_name() {
        assert_eq!(Schema::String.type_name(), "string");
        assert_eq!(Schema::Number.type_name(), "number");
        assert_eq!(Schema::Integer.type_name(), "integer");
        assert_eq!(Schema::Boolean.type_name(), "boolean");
        assert_eq!(Schema::Null.type_name(), "nil");
        assert_eq!(Schema::Any.type_name(), "any");
    }

    #[test]
    fn test_object_schema() {
        let schema = Schema::Object(vec![
            Field::new("name", Schema::String),
            Field::optional("age", Schema::Integer),
        ]);
        if let Schema::Object(fields) = &schema {
            assert_eq!(fields.len(), 2);
            assert_eq!(fields[0].name, "name");
            assert!(fields[0].required);
            assert_eq!(fields[1].name, "age");
            assert!(!fields[1].required);
        } else {
            panic!("Expected Object schema");
        }
    }

    #[test]
    fn test_constraints_with_values() {
        let c = Constraints {
            min: Some(0.0),
            max: Some(100.0),
            min_len: None,
            max_len: Some(255),
            one_of: None,
        };
        assert_eq!(c.min, Some(0.0));
        assert_eq!(c.max, Some(100.0));
        assert_eq!(c.max_len, Some(255));
    }
}

#[cfg(all(test, feature = "mlua"))]
mod lua_tests {
    use super::*;
    use mlua::prelude::*;

    #[test]
    fn to_lua_table_simple_object() {
        let lua = Lua::new();
        let schema = Schema::Object(vec![
            Field::new("name", Schema::String),
            Field::optional("bio", Schema::String),
        ]);

        let value = schema.to_lua_table(&lua).unwrap();
        let table = value.as_table().unwrap();

        // "name" is required → full format
        let name_val: LuaTable = table.get("name").unwrap();
        let name_type: String = name_val.get("type").unwrap();
        assert_eq!(name_type, "string");
        let name_req: bool = name_val.get("required").unwrap();
        assert!(name_req);

        // "bio" is optional, no constraints → shorthand
        let bio_val: String = table.get("bio").unwrap();
        assert_eq!(bio_val, "string");
    }

    #[test]
    fn to_lua_table_with_constraints() {
        let lua = Lua::new();
        let schema = Schema::Object(vec![Field {
            name: "age".into(),
            schema: Schema::Integer,
            required: true,
            constraints: Constraints {
                min: Some(0.0),
                max: Some(150.0),
                ..Default::default()
            },
        }]);

        let value = schema.to_lua_table(&lua).unwrap();
        let table = value.as_table().unwrap();

        let age: LuaTable = table.get("age").unwrap();
        let age_type: String = age.get("type").unwrap();
        assert_eq!(age_type, "integer");
        let age_min: f64 = age.get("min").unwrap();
        assert!((age_min - 0.0).abs() < f64::EPSILON);
        let age_max: f64 = age.get("max").unwrap();
        assert!((age_max - 150.0).abs() < f64::EPSILON);
    }

    #[test]
    fn to_lua_table_with_one_of() {
        let lua = Lua::new();
        let schema = Schema::Object(vec![Field {
            name: "status".into(),
            schema: Schema::String,
            required: true,
            constraints: Constraints {
                one_of: Some(vec!["active".into(), "inactive".into()]),
                ..Default::default()
            },
        }]);

        let value = schema.to_lua_table(&lua).unwrap();
        let table = value.as_table().unwrap();

        let status: LuaTable = table.get("status").unwrap();
        let one_of: LuaTable = status.get("one_of").unwrap();
        let v1: String = one_of.get(1).unwrap();
        let v2: String = one_of.get(2).unwrap();
        assert_eq!(v1, "active");
        assert_eq!(v2, "inactive");
    }

    #[test]
    fn to_lua_table_non_object_returns_string() {
        let lua = Lua::new();
        let value = Schema::String.to_lua_table(&lua).unwrap();
        let s = value.as_string().map(|s| s.to_string_lossy()).unwrap();
        assert_eq!(s, "string");
    }
}
