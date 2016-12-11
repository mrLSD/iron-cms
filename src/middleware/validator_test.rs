#[cfg(test)]
mod test {
    use super::super::*;
    use params::{Map, Value};

    // Convert Json to Value
    fn to_value(json_value: Json) -> Value {
        match json_value {
            Json::I64(value) => Value::I64(value),
            Json::U64(value) => Value::U64(value),
            Json::F64(value) => Value::F64(value),
            Json::String( ref value) => Value::String(value.clone()),
            Json::Boolean(value) => Value::Boolean(value),
            _ => Value::Null,
        }
    }

    /// Basic test function
    macro_rules! test {
        ($func:ident = $body:expr) => {
            #[test]
            fn $func() {
                $body
            }
        };
    }

    /// Validator test macros
    macro_rules! validate {
        // Test empty value for validator
        ($validator:ident [$assert:expr] $eq:expr => $t:ident) => {
            let validator_type = stringify!($t).to_lowercase();
            let values = Map::new();

            let validator = ValidateResults(vec!(
                Validator::<$t>::new(btreemap! {
                    stringify!($validator).to_string() => ($eq).to_json(),
                    "vtype".to_string() => validator_type.to_json(),
                }).validate("some_field".to_string(), values.find(&["some", "field"])),
            ));
            if $assert {
                assert!(validator.get_errors().is_some());
            } else {
                assert!(validator.get_errors().is_none());
            }
        };
        // Test validator and multiple values
        ($validator:ident [$assert:expr] $eq:expr => $($t:ident $val:expr),+ ) => {
            $(
                let validator_type = stringify!($t).to_lowercase();
                let mut value = $val;
                if validator_type == "string" {
                    value = value.into()
                }
                let mut values = Map::new();
                values.assign("some[field]", to_value(value.to_json())).unwrap();

                let validator = ValidateResults(vec!(
                    Validator::<$t>::new(btreemap! {
                        stringify!($validator).to_string() => ($eq).to_json(),
                        "vtype".to_string() => validator_type.to_json(),
                    }).validate("some_field".to_string(), values.find(&["some", "field"])),
                ));
                if $assert {
                    assert!(validator.get_errors().is_some());
                } else {
                    assert!(validator.get_errors().is_none());
                }
            )+
        }
    }

    /// Test "test macros"
    test!(test_macros = {
        validate!(eq [false] 100.3 => f64);
        validate!(eq [false] 100.3 => f64 100.3, f64 100.3);
    });

    #[test]
    /// Test Validator::new method
    fn new_test() {
        // Test for required equal
        let val_req = Validator::<String>::new(btreemap! {
            "required".to_string() => true.to_json(),
            "vtype".to_string() => "bool".to_json(),
        });
        assert_eq!(val_req.required, Some(true));

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
                "required".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("title".to_string(), values.find(&["pages", "title"])),
            Validator::<bool>::new(btreemap! {
                "default".to_string() => false.to_json(),
                "vtype".to_string() => "bool".to_json(),
            }).validate("published".to_string(), values.find(&["published"])),
        ));
        assert!(validator.get_errors().is_none());
    }

    /// Test validator: required
    test!(required_validator_test = {
        // Field is set
        validate!(required [false] true => String "Test");
        // Field is not set
        validate!(required [true] true => String);
        // Field is empty
        validate!(required [true] true => String "");
        // Field with int type
        validate!(required [false] true => i64 23);

        // Field with wrong type
        let mut values = Map::new();
        values.assign("age", Value::String("Test".into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<i64>::new(btreemap! {
                "required".to_string() => true.to_json(),
                "vtype".to_string() => "i64".to_json(),
            }).validate("man_age".to_string(), values.find(&["age"])),
        ));
        assert!(validator.get_errors().is_some());
    });

    /// Test validator: default
    test!(default_validator_test = {
        // Field is set
        validate!(default [false] "Default text" => String "Test");

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
    });

    /// Test validator: not_empty
    test!(not_empty_validator_test = {
        // Field is set
        validate!(not_empty [false] true => String "Test");
        // Field is empty
        validate!(not_empty [true] true => String "");
        // Field is not set
        validate!(not_empty [false] true => String);
    });

    /// Test validator: max
    test!(max_validator_test = {
        // Field is set as valid
        validate!(max [false] 10 => String "Test");
        // Field is set as not valid
        validate!(max [true] 3 => String "Test");
        // Field is set as valid - UTF8
        validate!(max [false] 16 => String "Test Тест délice");
        // Field is not set
        validate!(max [false] 16 => String);
        // Valid
        validate!(max [false] 12 => i64 10);
        // Not valid
        validate!(max [true] 9 => i64 10);
    });

    /// Test validator: min
    test!(min_validator_test = {
        // Field is set as valid
        validate!(min [false] 4 => String "Test");
        // Field is set as valid
        validate!(min [false] 0 => String "Test");
        // Field is set as valid - UTF8
        validate!(min [false] 16 => String "Test Тест délice");
        // Field is not set
        validate!(min [false] 10 => String);
        // Field is set as not valid
        validate!(min [true] 5 => String "Test");
        // Not valid
        validate!(min [true] -10 => i64 -20);
        // Valid
        validate!(min [false] -20 => i64 -20);

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
    });

    /// Test validator: min - type test
    test!(min_validator_type_test = {
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
        validate!(min [false] 0 => f64 5.1);
    });

    /// Test validator: email
    test!(email_validator_test = {
        // Valid value and type
        validate!(email [false] true => String "1W.B.c-D.E_f@B-b.C.d.easD");
        // Valid value and type
        validate!(email [false] true => String "the.test@google.com");

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
        let invalid_emails = vec!(
            "http://www.google.com",
            "the.test.@google.com",
            ".test.@google.com",
            "the.test.google.com",
            "the.test@google.com.",
            "the.test@googlecom",
            "the@test",
        );
        for email in invalid_emails {
            validate!(email [true] true => String email);
        }
        // Value is not set
        validate!(email [false] true => String);
    });

    /// Test validator: url
    /// Only Valid URL's
    /// Author of URL's: https://mathiasbynens.be/demo/url-regex
    test!(valid_url_validator_test = {
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
            validate!(url [false] true => String url);
        }
    });

    /// Test validator: url
    /// Only Invalid URL's
    /// Author of URL's: https://mathiasbynens.be/demo/url-regex
    test!(invalid_url_validator_test = {
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
            validate!(url [true] true => String url);
        }
    });

    /// Test validator: regexp
    test!(regexp_validator_test = {
        let rule = r"\A(?i)[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\z";
        // Value is not set
        validate!(regexp [false] rule => String);
        // Valid value
        validate!(regexp [false] rule => String "test@google.com");
        // Not valid value
        validate!(regexp [true] rule => String "test@google.com.");
    });

    /// Test validator: equals
    test!(eq_validator_test = {
        let rule = "test@google.com";
        // Value is not set
        validate!(eq [false] rule => String);
        // Valid value
        validate!(eq [false] rule => String "test@google.com");
        // Not valid value
        validate!(eq [true] rule => String "test");
        // Valid value- Valid type
        validate!(eq [false] 100.3 => f64 100.3);

        // Invalid value and invalid type
        let mut values = Map::new();
        values.assign("user[email]", Value::String(("test@google.com").into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<i64>::new(btreemap! {
                "eq".to_string() => 100.to_json(),
                "vtype".to_string() => "f64".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
        ));
        assert!(validator.get_errors().is_some());

        // Valid value- invalid type
        let mut values = Map::new();
        values.assign("user[email]", Value::F64(100.)).unwrap();

        let validator = ValidateResults(vec!(
            // Type should be f64
            Validator::<i64>::new(btreemap! {
                "eq".to_string() => 100.to_json(),
                "vtype".to_string() => "f64".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
        ));
        assert!(validator.get_errors().is_some());

        // Valid value- invalid type
        let mut values = Map::new();
        values.assign("user[email]", Value::String(("test@google.com").into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<f64>::new(btreemap! {
                "eq".to_string() => 100.to_json(),
                "vtype".to_string() => "f64".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
        ));
        assert!(validator.get_errors().is_some());
    });

}
