use schema_bridge::{Schema, SchemaBridge};

#[derive(SchemaBridge)]
struct User {
    name: String,
    #[schema(min = 0, max = 150)]
    age: i32,
    email: Option<String>,
}

#[derive(SchemaBridge)]
struct Profile {
    #[schema(min_len = 1, max_len = 50)]
    username: String,
    #[schema(one_of("active", "inactive", "banned"))]
    status: String,
}

#[derive(SchemaBridge)]
enum Role {
    Admin,
    User,
    Guest,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn struct_to_schema_produces_object() {
        let schema = User::to_schema();
        if let Schema::Object(fields) = &schema {
            assert_eq!(fields.len(), 3);

            assert_eq!(fields[0].name, "name");
            assert_eq!(fields[0].schema, Schema::String);
            assert!(fields[0].required);

            assert_eq!(fields[1].name, "age");
            assert_eq!(fields[1].schema, Schema::Integer);
            assert!(fields[1].required);
            assert_eq!(fields[1].constraints.min, Some(0.0));
            assert_eq!(fields[1].constraints.max, Some(150.0));

            assert_eq!(fields[2].name, "email");
            assert_eq!(fields[2].schema, Schema::String);
            assert!(!fields[2].required, "Option<T> fields should be optional");
        } else {
            panic!("Expected Schema::Object, got: {schema:?}");
        }
    }

    #[test]
    fn schema_constraints_min_len_max_len() {
        let schema = Profile::to_schema();
        if let Schema::Object(fields) = &schema {
            let username = &fields[0];
            assert_eq!(username.name, "username");
            assert_eq!(username.constraints.min_len, Some(1));
            assert_eq!(username.constraints.max_len, Some(50));
        } else {
            panic!("Expected Schema::Object");
        }
    }

    #[test]
    fn schema_one_of_attribute() {
        let schema = Profile::to_schema();
        if let Schema::Object(fields) = &schema {
            let status = &fields[1];
            assert_eq!(status.name, "status");
            assert_eq!(
                status.constraints.one_of,
                Some(vec![
                    "active".to_string(),
                    "inactive".to_string(),
                    "banned".to_string()
                ])
            );
        } else {
            panic!("Expected Schema::Object");
        }
    }

    #[test]
    fn enum_to_schema_produces_enum() {
        let schema = Role::to_schema();
        if let Schema::Enum(variants) = &schema {
            assert_eq!(variants, &["Admin", "User", "Guest"]);
        } else {
            panic!("Expected Schema::Enum, got: {schema:?}");
        }
    }

    #[test]
    fn option_field_uses_inner_type_schema() {
        let schema = User::to_schema();
        if let Schema::Object(fields) = &schema {
            // email: Option<String> → schema should be String (not Union(String, Null))
            assert_eq!(fields[2].schema, Schema::String);
        } else {
            panic!("Expected Schema::Object");
        }
    }
}
