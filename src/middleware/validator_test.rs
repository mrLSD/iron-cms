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
            "vtype".to_string() => "i32".to_json(),
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

    #[test]
    /// Test validator: not_empty
    fn not_empty_validator_test() {
        // Field is set
        let mut values = Map::new();
        values.assign("pages[title]", Value::String("Test".into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "not_empty".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("title".to_string(), values.find(&["pages", "title"])),
        ));
        assert!(validator.get_errors().is_none());

        // Field is empty
        let mut values = Map::new();
        values.assign("pages[title]", Value::String("".into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "not_empty".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("title".to_string(), values.find(&["pages", "title"])),
        ));
        assert!(validator.get_errors().is_some());

        // Field is not set
        let values = Map::new();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "not_empty".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("title".to_string(), values.find(&["pages", "title"])),
        ));
        assert!(validator.get_errors().is_none());
    }

    #[test]
    /// Test validator: max
    fn max_validator_test() {
        // Field is set as valid
        let mut values = Map::new();
        values.assign("pages[title]", Value::String("Test".into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "max".to_string() => 10.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("title".to_string(), values.find(&["pages", "title"])),
        ));
        assert!(validator.get_errors().is_none());

        // Field is set as not valid
        let mut values = Map::new();
        values.assign("pages[title]", Value::String("Test".into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "max".to_string() => 3.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("title".to_string(), values.find(&["pages", "title"])),
        ));
        assert!(validator.get_errors().is_some());

        // Field is set as valid - UTF8
        let mut values = Map::new();
        values.assign("pages[title]", Value::String("Test Тест délice".into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "max".to_string() => 16.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("title".to_string(), values.find(&["pages", "title"])),
        ));
        assert!(validator.get_errors().is_none());

        // Field is not set
        let values = Map::new();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "max".to_string() => 16.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("title".to_string(), values.find(&["pages", "title"])),
        ));
        assert!(validator.get_errors().is_none());
    }

    #[test]
    /// Test validator: min
    fn min_validator_test() {
        // Field is set as valid
        let mut values = Map::new();
        values.assign("pages[title]", Value::String("Test".into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "min".to_string() => 4.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("title".to_string(), values.find(&["pages", "title"])),
        ));
        assert!(validator.get_errors().is_none());

        // Field is set as valid
        let mut values = Map::new();
        values.assign("pages[title]", Value::String("Test".into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "min".to_string() => 0.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("title".to_string(), values.find(&["pages", "title"])),
        ));
        assert!(validator.get_errors().is_none());

        // Field is set as valid - UTF8
        let mut values = Map::new();
        values.assign("pages[title]", Value::String("Test Тест délice".into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "min".to_string() => 16.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("title".to_string(), values.find(&["pages", "title"])),
        ));
        assert!(validator.get_errors().is_none());

        // Field is not set
        let values = Map::new();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "min".to_string() => 10.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("title".to_string(), values.find(&["pages", "title"])),
        ));
        assert!(validator.get_errors().is_none());

        // Field is set as not valid
        let mut values = Map::new();
        values.assign("pages[title]", Value::String("Test".into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "min".to_string() => 5.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("title".to_string(), values.find(&["pages", "title"])),
        ));
        assert!(validator.get_errors().is_some());

        // Field is set as not valid
        let mut values = Map::new();
        values.assign("temperature", Value::I64(-20)).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<i64>::new(btreemap! {
                "min".to_string() => (-10).to_json(),
                "vtype".to_string() => "i64".to_json(),
            }).validate("temperature".to_string(), values.find(&["temperature"])),
        ));
        assert!(validator.get_errors().is_some());

        let validator = ValidateResults(vec!(
            Validator::<i64>::new(btreemap! {
                "min".to_string() => (-20).to_json(),
                "vtype".to_string() => "i64".to_json(),
            }).validate("temperature".to_string(), values.find(&["temperature"])),
        ));
        assert!(validator.get_errors().is_none());

        // Test max + min, whare max <= min
        let mut values = Map::new();
        values.assign("temperature", Value::I64(10)).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<i64>::new(btreemap! {
                "max".to_string() => 5.to_json(),
                "min".to_string() => 10.to_json(),
                "vtype".to_string() => "i64".to_json(),
            }).validate("temperature".to_string(), values.find(&["temperature"])),
        ));
        assert!(validator.get_errors().is_some());

        // Test max + min, whare max > min
        let validator = ValidateResults(vec!(
            Validator::<i64>::new(btreemap! {
                "max".to_string() => 20.to_json(),
                "min".to_string() => 10.to_json(),
                "vtype".to_string() => "i64".to_json(),
            }).validate("temperature".to_string(), values.find(&["temperature"])),
        ));
        assert!(validator.get_errors().is_none());
    }

    #[test]
    /// Test validator: min - type test
    fn min_validator_type_test() {
        // Invalid value type
        let mut values = Map::new();
        values.assign("temperature", Value::String("Test".into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "min".to_string() => 0.to_json(),
                "vtype".to_string() => "f64".to_json(),
            }).validate("temperature".to_string(), values.find(&["temperature"])),
        ));
        assert!(validator.get_errors().is_some());

        // Valid value and type
        let mut values = Map::new();
        values.assign("temperature", Value::F64(5.1)).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "min".to_string() => 0.to_json(),
                "vtype".to_string() => "f64".to_json(),
            }).validate("temperature".to_string(), values.find(&["temperature"])),
        ));
        assert!(validator.get_errors().is_none());
    }

    #[test]
    /// Test validator: email
    fn email_validator_test() {
        // Valid value and type
        let mut values = Map::new();
        values.assign("user[email]", Value::String(("1W.B.c-D.E_f@B-b.C.d.easD").into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "email".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
        ));
        assert!(validator.get_errors().is_none());

        // Valid value and type
        let mut values = Map::new();
        values.assign("user[email]", Value::String(("the.test@google.com").into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "email".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
        ));
        assert!(validator.get_errors().is_none());

        // Not valid value and type
        let mut values = Map::new();
        values.assign("user[email]", Value::I64((100).into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "email".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
        ));
        assert!(validator.get_errors().is_some());

        // Not valid value
        let mut values = Map::new();
        values.assign("user[email]", Value::String(("the.test.@google.com").into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "email".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
        ));
        assert!(validator.get_errors().is_some());

        // Not valid value
        let mut values = Map::new();
        values.assign("user[email]", Value::String((".test.@google.com").into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "email".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
        ));
        assert!(validator.get_errors().is_some());

        // Not valid value
        let mut values = Map::new();
        values.assign("user[email]", Value::String(("the.test.google.com").into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "email".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
        ));
        assert!(validator.get_errors().is_some());

        // Not valid value
        let mut values = Map::new();
        values.assign("user[email]", Value::String(("the.test@google.com.").into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "email".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
        ));
        assert!(validator.get_errors().is_some());

        // Not valid value
        let mut values = Map::new();
        values.assign("user[email]", Value::String(("the.test@googlecom").into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "email".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
        ));
        assert!(validator.get_errors().is_some());

        // Not valid value
        let mut values = Map::new();
        values.assign("user[email]", Value::String(("the@test").into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "email".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
        ));
        assert!(validator.get_errors().is_some());


        // Value is not set
        let values = Map::new();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "email".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
        ));
        assert!(validator.get_errors().is_none());
    }

    #[test]
    /// Test validator: url
    fn url_validator_test() {
        //===========================================
        // Valid URLS
        let mut values = Map::new();
        values.assign("user[url]", Value::String(("http://www.google.com").into())).unwrap();
        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "url".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_url".to_string(), values.find(&["user", "url"])),
        ));
        assert!(validator.get_errors().is_none());

        let mut values = Map::new();
        values.assign("user[url]", Value::String(("http://www.google.com/").into())).unwrap();
        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "url".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_url".to_string(), values.find(&["user", "url"])),
        ));
        assert!(validator.get_errors().is_none());

        let mut values = Map::new();
        values.assign("user[url]", Value::String(("http://foo.com/blah_blah").into())).unwrap();
        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "url".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_url".to_string(), values.find(&["user", "url"])),
        ));
        assert!(validator.get_errors().is_none());

        let mut values = Map::new();
        values.assign("user[url]", Value::String(("http://foo.com/blah_blah.json").into())).unwrap();
        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "url".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_url".to_string(), values.find(&["user", "url"])),
        ));
        assert!(validator.get_errors().is_none());

        let mut values = Map::new();
        values.assign("user[url]", Value::String(("http://foo.com/blah_blah/").into())).unwrap();
        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "url".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_url".to_string(), values.find(&["user", "url"])),
        ));
        assert!(validator.get_errors().is_none());

        let mut values = Map::new();
        values.assign("user[url]", Value::String(("http://foo.com/blah_blah_(wikipedia)").into())).unwrap();
        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "url".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_url".to_string(), values.find(&["user", "url"])),
        ));
        assert!(validator.get_errors().is_none());

        let mut values = Map::new();
        values.assign("user[url]", Value::String(("http://foo.com/blah_blah_(wikipedia)_(again)").into())).unwrap();
        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "url".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_url".to_string(), values.find(&["user", "url"])),
        ));
        assert!(validator.get_errors().is_none());

        let mut values = Map::new();
        values.assign("user[url]", Value::String(("http://www.example.com/wpstyle/?p=364").into())).unwrap();
        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "url".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_url".to_string(), values.find(&["user", "url"])),
        ));
        assert!(validator.get_errors().is_none());

        let mut values = Map::new();
        values.assign("user[url]", Value::String(("https://www.example.com/foo/?bar=baz&inga=42&quux").into())).unwrap();
        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "url".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_url".to_string(), values.find(&["user", "url"])),
        ));
        assert!(validator.get_errors().is_none());
    }

}
