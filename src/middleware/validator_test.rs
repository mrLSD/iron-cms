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
    /// Only Valid URL's
    /// Author of URL's: https://mathiasbynens.be/demo/url-regex
    fn valid_url_validator_test() {
        let valid_urls = vec!(
            "http://www.google.com",
            "http://www.google.com/",
            "http://foo.com/blah_blah",
            "http://foo.com/blah_blah.json",
            "http://foo.com/blah_blah/",
            "http://foo.com/blah_blah_(wikipedia)",
            "http://foo.com/blah_blah_(wikipedia)_(again)",
            "http://www.example.com/wpstyle/?p=364",
            "https://www.example.com/foo/?bar=baz&inga=42&quux",
            "http://✪df.ws/123",
            "http://userid:password@example.com:8080",
            "http://userid:password@example.com:8080/",
            "http://userid@example.com",
            "http://userid@example.com/",
            "http://userid@example.com:8080",
            "http://userid@example.com:8080/",
            "http://userid:password@example.com",
            "http://userid:password@example.com/",
            "http://142.42.1.1/",
            "http://142.42.1.1:8080/",
            "http://➡.ws/䨹",
            "http://⌘.ws",
            "http://⌘.ws/",
            "http://foo.com/blah_(wikipedia)#cite-1",
            "http://foo.com/blah_(wikipedia)_blah#cite-1",
            "http://foo.com/unicode_(✪)_in_parens",
            "http://foo.com/(something)?after=parens",
            "http://☺.damowmow.com/",
            "http://code.google.com/events/#&product=browser",
            "http://j.mp",
            "ftp://foo.bar/baz",
            "http://foo.bar/?q=Test%20URL-encoded%20stuff",
            "http://مثال.إختبار",
            "http://例子.测试",
            "http://उदाहरण.परीक्षा",
            "http://-.~_!$&'()*+,;=:%40:80%2f::::::@example.com",
            "http://1337.net",
            "http://223.255.255.254"
        );
        for url in valid_urls {
            let mut values = Map::new();
            values.assign("user[url]", Value::String((url).into())).unwrap();
            let validator = ValidateResults(vec!(
                Validator::<String>::new(btreemap! {
                    "url".to_string() => true.to_json(),
                    "vtype".to_string() => "string".to_json(),
                }).validate("user_url".to_string(), values.find(&["user", "url"])),
            ));
            assert!(validator.get_errors().is_none());
        }
    }

    #[test]
    /// Test validator: url
    /// Only Invalid URL's
    /// Author of URL's: https://mathiasbynens.be/demo/url-regex
    fn invalid_url_validator_test() {
        let invalid_urls = vec!(
            "http://",
            "http:// ",
            "http://.",
            "http://..",
            "http://../",
            "http://?",
            "http://??",
            "http://??/",
            "http://#",
            "http://##",
            "http://##/",
            "htp://google,com/",
            "http://foo.bar?q=Spaces should be encoded",
            "//",
            "//a",
            "///a",
            "///",
            "http:///a",
            "foo.com",
            "rdar://1234",
            "h://test",
            "http:// shouldfail.com",
            ":// should fail",
            "http://foo.bar/foo(bar)baz quux",
            "ftps://foo.bar/",
            "http://-error-.invalid/",
            "http://a.b--c.de/",
            "http://-a.b.co",
            "http://a.b-.co",
            "http://.www.foo.bar/",
        );
        for url in invalid_urls {
            let mut values = Map::new();
            values.assign("user[url]", Value::String((url).into())).unwrap();
            let validator = ValidateResults(vec!(
                Validator::<String>::new(btreemap! {
                    "url".to_string() => true.to_json(),
                    "vtype".to_string() => "string".to_json(),
                }).validate("user_url".to_string(), values.find(&["user", "url"])),
            ));
            assert!(validator.get_errors().is_some());
        }
    }

}
