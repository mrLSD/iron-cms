#[cfg(test)]
mod pages_test {
    use params::{Map, Value};
    use super::super::*;
    use super::super::pages::*;
    #[test]
    // Test validation for current model
    fn validate_title_test() {
        // Test valid result
        let mut map = Map::new();
        map.assign("pages[title]", Value::String("Test Title".into())).unwrap();
        let validator = validate(&map);
        assert!(validator.get_errors().is_none());

        // test unvalid result
        let mut map = Map::new();
        map.assign("title", Value::String("".into())).unwrap();
        let validator = validate(&map);
        assert!(validator.get_errors().is_some());

        // test unvalid result
        let mut map = Map::new();
        map.assign("title", Value::String("  ".into())).unwrap();
        let validator = validate(&map);
        assert!(validator.get_errors().is_some());
    }

    #[test]
    fn validate_published_test() {
        let mut map = Map::new();
        // Should setrequired field
        map.assign("pages[title]", Value::String("Test".into())).unwrap();
        map.assign("published", Value::Boolean(true)).unwrap();
        let validator = validate(&map);
        assert!(validator.get_errors().is_none());

        let mut map = Map::new();
        map.assign("pages[title]", Value::String("Test".into())).unwrap();
        map.assign("published", Value::String("on".into())).unwrap();
        let validator = validate(&map);
        assert!(validator.get_errors().is_none());

        let mut map = Map::new();
        map.assign("title", Value::String("Test".into())).unwrap();
        map.assign("published", Value::String("off".into())).unwrap();
        let validator = validate(&map);
        assert!(validator.get_errors().is_none());

        let mut map = Map::new();
        map.assign("title", Value::String("Test".into())).unwrap();
        map.assign("published", Value::I64(1)).unwrap();
        let validator = validate(&map);
        assert!(validator.get_errors().is_none());

        let mut map = Map::new();
        map.assign("title", Value::String("Test".into())).unwrap();
        map.assign("published", Value::I64(0)).unwrap();
        let validator = validate(&map);
        assert!(validator.get_errors().is_none());

        let mut map = Map::new();
        map.assign("title", Value::String("Test".into())).unwrap();
        map.assign("published", Value::String("test".into())).unwrap();
        let validator = validate(&map);
        assert!(validator.get_errors().is_some());

        let mut map = Map::new();
        map.assign("title", Value::String("Test".into())).unwrap();
        map.assign("published", Value::I64(-1)).unwrap();
        let validator = validate(&map);
        assert!(validator.get_errors().is_some());

        let mut map = Map::new();
        map.assign("title", Value::String("Test".into())).unwrap();
        map.assign("published", Value::I64(2)).unwrap();
        let validator = validate(&map);
        assert!(validator.get_errors().is_some());
    }

    #[test]
    fn validate_published_default_test() {
        let mut map = Map::new();
        // Should setrequired field
        map.assign("pages[title]", Value::String("Test".into())).unwrap();
        let validator = validate(&map);
        assert!(validator.get_errors().is_none());

        let page = init(validator.get_values());
        assert_eq!(page.published, false);
    }

    #[test]
    #[should_panic]
    fn wrong_type_test() {
        let mut map = Map::new();
        //map.assign("title", Value::String("Test Title".into())).unwrap();
        map.assign("title", Value::Boolean(true)).unwrap();

        // Weong type declaration
        let validator = ValidateResults(vec!(
            Validator::<bool>::new(btreemap! {
                "requiered".to_string() => true.to_json(),
                "vtype".to_string() => "bool".to_json(),
            }).validate("title".to_string(), map.find(&["title"])),
        ));
        // Should paniced
        assert!(validator.get_errors().is_none());
        // Should paniced
        // cause Page stuct `title` field is String not `bool`
        let _ = init(validator.get_values());
    }
}
