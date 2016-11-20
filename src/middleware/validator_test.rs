#[cfg(test)]
mod test {
    use super::super::*;
    use params::{Map, Value};

    #[test]
    /// Test Validator::new method
    fn new_test() {
        // Test for requiered equal
        let val_req = Validator::<String>::new(btreemap! {
            "requiered".to_string() => true.to_json(),
            "vtype".to_string() => "bool".to_json(),
        });
        assert_eq!(val_req.requiered, Some(true));

        // Test for non-panic
        Validator::<bool>::new(btreemap! {
            "default".to_string() => false.to_json(),
            "vtype".to_string() => "bool".to_json(),
        });

        let val_def = Validator::<bool>::new(btreemap! {
            "default".to_string() => false.to_json(),
            "vtype".to_string() => "bool".to_json(),
        });
        assert_eq!(val_def.default, Some(false));

        let val_def = Validator::<i32>::new(btreemap! {
            "default".to_string() => 100i32.to_json(),
            "vtype".to_string() => "bool".to_json(),
        });
        assert_eq!(val_def.default, Some(100i32));
    }

    #[test]
    #[should_panic]
    fn new_with_wrong_type_test() {
        // It should be: Validator::<bool>
        Validator::<String>::new(btreemap! {
            "default".to_string() => false.to_json(),
            "vtype".to_string() => "bool".to_json(),
        });
    }

    #[test]
    /// Test rules - bouth rules set
    fn validator_two_rules_test() {
        let mut values = Map::new();
        values.assign("pages[title]", Value::String("Test".into())).unwrap();
        values.assign("pages[published]", Value::Boolean(true)).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "requiered".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("title".to_string(), values.find(&["pages", "title"])),
            Validator::<bool>::new(btreemap! {
                "default".to_string() => false.to_json(),
                "vtype".to_string() => "bool".to_json(),
            }).validate("published".to_string(), values.find(&["published"])),
        ));
        assert!(validator.get_errors().is_none());
    }

    #[test]
    /// Test validator: requiered
    fn requiered_validator_test() {
        // Field is set
        let mut values = Map::new();
        values.assign("pages[title]", Value::String("Test".into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "requiered".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("title".to_string(), values.find(&["pages", "title"])),
        ));
        assert!(validator.get_errors().is_none());

        // Field is not set
        let values = Map::new();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "requiered".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("title".to_string(), values.find(&["pages", "title"])),
        ));
        assert!(validator.get_errors().is_some());

        // Field is not empty
        let mut values = Map::new();
        values.assign("pages[title]", Value::String("".into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "requiered".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("title".to_string(), values.find(&["pages", "title"])),
        ));
        assert!(validator.get_errors().is_some());

        // Field with int type
        let mut values = Map::new();
        values.assign("age", Value::I64(23)).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<i64>::new(btreemap! {
                "requiered".to_string() => true.to_json(),
                "vtype".to_string() => "i64".to_json(),
            }).validate("man_age".to_string(), values.find(&["age"])),
        ));
        assert!(validator.get_errors().is_none());

        // Field with wrong type
        let mut values = Map::new();
        values.assign("age", Value::String("Test".into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<i64>::new(btreemap! {
                "requiered".to_string() => true.to_json(),
                "vtype".to_string() => "i64".to_json(),
            }).validate("man_age".to_string(), values.find(&["age"])),
        ));
        assert!(validator.get_errors().is_some());
    }

    #[test]
    /// Test validator: default
    fn default_validator_test() {
        // Field is set
        let mut values = Map::new();
        values.assign("pages[title]", Value::String("Test".into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "default".to_string() => "Default text".to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("title".to_string(), values.find(&["pages", "title"])),
        ));
        assert!(validator.get_errors().is_none());
        assert_eq!(validator.get_values()["title"], "Test".to_json());

        // Field is not set
        let values = Map::new();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "default".to_string() => "Default text".to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("title".to_string(), values.find(&["pages", "title"])),
        ));
        assert!(validator.get_errors().is_none());
        assert_eq!(validator.get_values()["title"], "Default text".to_json());
    }
}
