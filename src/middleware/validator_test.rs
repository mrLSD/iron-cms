#[cfg(test)]
mod test {
    use super::super::*;
    use params::{Map, Value};

    // Convert Json to Value
    fn to_value(json_value: Json) -> Value {
        match json_value {
            Json::U64(value) => Value::U64(value),
            Json::I64(value) => Value::I64(value),
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

    /// Valid macrod validator - valid asserions
    macro_rules! valid {
        // Valid without expression
        ($validator:ident => $($t:ident $val:expr),+ ) => {
            $(
                validate! ($validator [false] true => $t $val);
            )+
        };
        // Valid without expression and without value
        ($validator:ident => $t:ident ) => {
            validate! ($validator [false] true => $t);
        };
        // Valid with equal expression
        ($validator:ident $eq:expr => $($t:ident $val:expr),+ ) => {
            $(
                validate!($validator [false] $eq => $t $val);
            )+
        };
        // Valid with equal expression and without value
        ($validator:ident $eq:expr => $t:ident) => {
            validate!($validator [false] $eq => $t);
        }
    }

    /// Invalid macrod validator - failed asserions
    macro_rules! invalid {
        // Valid without expression
        ($validator:ident => $($t:ident $val:expr),+ ) => {
            $(
                validate! ($validator [true] true => $t $val);
            )+
        };
        // Valid without expression and without value
        ($validator:ident => $t:ident ) => {
            validate! ($validator [true] true => $t);
        };
        // Valid with equal expression
        ($validator:ident $eq:expr => $($t:ident $val:expr),+ ) => {
            $(
                validate!($validator [true] $eq => $t $val);
            )+
        };
        // Valid with equal expression and without value
        ($validator:ident $eq:expr => $t:ident) => {
            $(
                validate!($validator [true] $eq => $t);
            )+
        }
    }

    /// Test JSON to Values for Tests
    test!(test_json_to_value_for_tests = {
        assert_eq!( to_value(Json::U64(10)), Value::U64(10) );
        assert_eq!( to_value(Json::I64(-10)), Value::I64(-10) );
        assert_eq!( to_value(Json::F64(-10.3)), Value::F64(-10.3) );
        assert_eq!( to_value(Json::Null), Value::Null );
        assert_eq!( to_value(Json::Boolean(false)), Value::Boolean(false) );
        assert_eq!( to_value(Json::String("test".into())), Value::String("test".into()) );
    });

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

        //================================
        // Field with wrong types
        let mut values = Map::new();
        values.assign("age", Value::String("test".into())).unwrap();
        let validator = ValidateResults(vec!(
            Validator::<i64>::new(btreemap! {
                "required".to_string() => true.to_json(),
                "vtype".to_string() => "i64".to_json(),
            }).validate("man_age".to_string(), values.find(&["age"])),
        ));
        assert!(validator.get_errors().is_some());

        let mut values = Map::new();
        values.assign("age", Value::String("Test".into())).unwrap();
        let validator = ValidateResults(vec!(
            Validator::<u64>::new(btreemap! {
                "required".to_string() => true.to_json(),
                "vtype".to_string() => "u64".to_json(),
            }).validate("man_age".to_string(), values.find(&["age"])),
        ));
        assert!(validator.get_errors().is_some());

        let mut values = Map::new();
        values.assign("age", Value::Array(vec![Value::F64(0.3), Value::F64(-20.4)])).unwrap();
        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "required".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("man_age".to_string(), values.find(&["age"])),
        ));
        assert!(validator.get_errors().is_some());

        let mut values = Map::new();
        values.assign("age", Value::Array(vec![Value::F64(0.3), Value::F64(-20.4)])).unwrap();
        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "required".to_string() => true.to_json(),
                "vtype".to_string() => "str1".to_json(),
            }).validate("man_age".to_string(), values.find(&["age"])),
        ));
        assert!(validator.get_errors().is_some());
    });

    /// Test to_value conversion
    test!(json_to_value_test = {
        let validator = Validator::<i64>::new(btreemap! {
            "required".to_string() => true.to_json(),
            "vtype".to_string() => "i64".to_json(),
        });
        assert_eq! (validator.to_value(Json::I64(10)), Some(Value::I64(10)));
        assert_eq! (validator.to_value(Json::U64(10)), Some(Value::U64(10)));
        assert_eq! (validator.to_value(Json::F64(10.)), Some(Value::F64(10.)));
        assert_eq! (validator.to_value(Json::Boolean(true)), Some(Value::Boolean(true)));
        assert_eq! (validator.to_value(Json::String("test".into())), Some(Value::String("test".into())));
        assert_eq! (validator.to_value(Json::Null), None);
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

        //===================
        // Field is not set
        let values = Map::new();
        let validator = ValidateResults(vec!(
            Validator::<u64>::new(btreemap! {
                "default".to_string() => 10u64.to_json(),
                "vtype".to_string() => "u64".to_json(),
            }).validate("title".to_string(), values.find(&["pages", "title"])),
        ));
        assert!(validator.get_errors().is_none());
        assert_eq!(validator.get_values()["title"], 10u64.to_json());

        let values = Map::new();
        let validator = ValidateResults(vec!(
            Validator::<i64>::new(btreemap! {
                "default".to_string() => (-10).to_json(),
                "vtype".to_string() => "i64".to_json(),
            }).validate("title".to_string(), values.find(&["pages", "title"])),
        ));
        assert!(validator.get_errors().is_none());
        assert_eq!(validator.get_values()["title"], (-10).to_json());

        let values = Map::new();
        let validator = ValidateResults(vec!(
            Validator::<f64>::new(btreemap! {
                "default".to_string() => (10.3).to_json(),
                "vtype".to_string() => "f64".to_json(),
            }).validate("title".to_string(), values.find(&["pages", "title"])),
        ));
        assert!(validator.get_errors().is_none());
        assert_eq!(validator.get_values()["title"], (10.3).to_json());

        let values = Map::new();
        let validator = ValidateResults(vec!(
            Validator::<bool>::new(btreemap! {
                "default".to_string() => true.to_json(),
                "vtype".to_string() => "bool".to_json(),
            }).validate("title".to_string(), values.find(&["pages", "title"])),
        ));
        assert!(validator.get_errors().is_none());
        assert_eq!(validator.get_values()["title"], true.to_json());

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
        validate!(not_empty [false] true => i64 10);
        // Field is empty
        validate!(not_empty [true] true => String "");
        // Field is not set
        validate!(not_empty [false] true => String);
        validate!(not_empty [false] true => i64);
    });

    /// Test validator: max
    test!(max_validator_test = {
        // Field is set as valid
        valid! (max 10 => String "Test");
        // Field is set as not valid
        invalid! (max 3 => String "Test");
        invalid! (max -3 => String "Test");
        invalid! (max -12 => u64 12u64);
        // Field is set as valid - UTF8
        valid! (max 16 => String "Test Тест délice");
        // Field is not set
        valid! (max 16 => String);
        // Valid
        valid! (max 12 => i64 10);
        valid! (max 12 => u64 8u64);
        valid! (max 12 => f64 8.0f64);
        valid! (max 12 => bool true);
        // Not valid
        invalid! (max 9 => i64 10);

        let mut values = Map::new();
        values.assign("pages[title]", Value::Null).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "max".to_string() => 2.to_json(),
                "vtype".to_string() => "vec".to_json(),
            }).validate("title".to_string(), values.find(&["pages", "title"])),
        ));
        assert!(validator.get_errors().is_some());

        let mut values = Map::new();
        values.assign("item", Value::Array(vec![Value::U64(2)])).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "max".to_string() => 2.to_json(),
                "vtype".to_string() => "vec".to_json(),
            }).validate("item".to_string(), values.find(&["item"])),
        ));
        assert!(validator.get_errors().is_some());
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

        validate!(min [false] 0 => bool);
        validate!(min [true] 0 => bool true);

        // Not valid
        validate!(min [true] -10 => i64 -20);
        // Valid
        validate!(min [false] -20 => i64 -20);
        validate!(min [false] 20 => u64 20);
        validate!(min [false] 20 => u64 20u64);

        // Test max + min, whare max <= min
        let mut values = Map::new();
        values.assign("temperature", Value::U64(10)).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<u64>::new(btreemap! {
                "max".to_string() => 5.to_json(),
                "min".to_string() => 10.to_json(),
                "vtype".to_string() => "u64".to_json(),
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
        invalid! (url => i64 10);
    });

    /// Test validator: regexp
    test!(regexp_validator_test = {
        let rule = r"\A(?i)[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\z";
        // Value is not set
        valid! (regexp rule => String);
        // Valid value
        valid! (regexp rule => String "test@google.com");
        // Not valid value
        invalid! (regexp rule => String "test@google.com.");
        invalid! (regexp rule => u64 10u64);
    });

    /// Test validator: equals
    test!(eq_validator_test = {
        let rule = "test@google.com";
        // Value is not set
        validate!(eq [false] rule => String);
        invalid! (eq true => bool false);
        // Valid value
        validate!(eq [false] rule => String "test@google.com");
        valid! (eq 10 => u64 10u64);
        valid! (eq -10 => i64 -10);
        valid! (eq -10.0 => f64 -10.0);
        valid! (eq true => bool true);
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

        // Invalid value and invalid type
        let mut values = Map::new();
        values.assign("user[email]", Value::Null).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<i64>::new(btreemap! {
                "eq".to_string() => Json::Null,
                "vtype".to_string() => "i64".to_json(),
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

    /// Test validator: uuid5
    test!(uuid5_validator_test = {
        // Invalid
        validate!(uuid5 [true] true => f64 100.3);
        validate!(uuid5 [true] true => String "");
        validate!(uuid5 [true] true => String "test");
        validate!(uuid5 [true] true => String "xxxa987fbc9-4bed-3078-cf07-9141ba07c9f3");
        validate!(uuid5 [true] true => String "9c858901-8a57-4791-81fe-4c455b099bc9");
        validate!(uuid5 [true] true => String "a987fbc9-4bed-3078-cf07-9141ba07c9f3");
        validate!(uuid5 [true] true => i64 "987fbc97-4bed-5078-af07-9141ba07c9f3");
        // Valid
        validate!(uuid5 [false] true => String "987fbc97-4bed-5078-af07-9141ba07c9f3");
        validate!(uuid5 [false] true => String "987fbc97-4bed-5078-9f07-9141ba07c9f3");
        // Value not set
        validate!(uuid5 [false] true => String);
    });

    /// Test validator: uuid4
    test!(uuid4_validator_test = {
        // Invalid
        validate!(uuid4 [true] true => f64 100.3);
        validate!(uuid4 [true] true => String "");
        validate!(uuid4 [true] true => String "test");
        validate!(uuid4 [true] true => String "xxxa987fbc9-4bed-3078-cf07-9141ba07c9f3");
        validate!(uuid4 [true] true => String "a987fbc9-4bed-5078-af07-9141ba07c9f3");
        validate!(uuid4 [true] true => String "934859");
        validate!(uuid4 [true] true => i64 "57b73598-8764-4ad0-a76a-679bb6640eb1");
        // Valid
        validate!(uuid4 [false] true => String "57b73598-8764-4ad0-a76a-679bb6640eb1");
        validate!(uuid4 [false] true => String "625e63f3-58f5-40b7-83a1-a72ad31acffb");
        // Value not set
        validate!(uuid4 [false] true => String);
    });

    /// Test validator: uuid3
    test!(uuid3_validator_test = {
        // Invalid
        validate!(uuid3 [true] true => f64 100.3);
        validate!(uuid3 [true] true => String "");
        validate!(uuid3 [true] true => String "test");
        validate!(uuid3 [true] true => String "412452646");
        validate!(uuid3 [true] true => String "xxxa987fbc9-4bed-3078-cf07-9141ba07c9f3");
        validate!(uuid3 [true] true => String "a987fbc9-4bed-4078-8f07-9141ba07c9f3");
        validate!(uuid3 [true] true => i64 "a987fbc9-4bed-3078-cf07-9141ba07c9f3");
        // Valid
        validate!(uuid3 [false] true => String "a987fbc9-4bed-3078-cf07-9141ba07c9f3");
        // Value not set
        validate!(uuid3 [false] true => String);
    });

    /// Test validator: uuid
    test!(uuid_validator_test = {
        // Invalid
        validate!(uuid [true] true => f64 100.3);
        validate!(uuid [true] true => String "");
        validate!(uuid [true] true => String "test");
        validate!(uuid [true] true => String "412452646");
        validate!(uuid [true] true => String "934859");
        validate!(uuid [true] true => String "xxxa987fbc9-4bed-3078-cf07-9141ba07c9f3");
        validate!(uuid [true] true => String "a987fbc9-4bed-3078-cf07-9141ba07c9f3xxx");
        validate!(uuid [true] true => String "a987fbc94bed3078cf079141ba07c9f3");
        validate!(uuid [true] true => String "987fbc9-4bed-3078-cf07a-9141ba07c9f3");
        validate!(uuid [true] true => String "aaaaaaaa-1111-1111-aaag-111111111111");
        validate!(uuid [true] true => i64 "a987fbc9-4bed-3078-cf07-9141ba07c9f3");
        // Valid
        validate!(uuid [false] true => String "a987fbc9-4bed-3078-cf07-9141ba07c9f3");
        validate!(uuid [false] true => String "a987fbc9-4bed-4078-8f07-9141ba07c9f3");
        // Value not set
        validate!(uuid [false] true => String);
    });

    /// Test validator: asciiprintable
    test!(asciiprintable_validator_test = {
        // Invalid
        validate!(asciiprintable [true] true => f64 100.3);
        validate!(asciiprintable [true] true => String "ｆｏｏbar");
        validate!(asciiprintable [true] true => String "ｘｙｚ０９８");
        validate!(asciiprintable [true] true => String "１２３456");
        validate!(asciiprintable [true] true => String "ｶﾀｶﾅ");
        validate!(asciiprintable [true] true => String "newline\n");
        validate!(asciiprintable [true] true => String "\x19test\x7F");
        // Valid
        validate!(asciiprintable [false] true => String "");
        validate!(asciiprintable [false] true => String " ");
        validate!(asciiprintable [false] true => String "foobar");
        validate!(asciiprintable [false] true => String "0987654321");
        validate!(asciiprintable [false] true => String "test@example.com");
        validate!(asciiprintable [false] true => String "1234abcDEF");
        // Value not set
        validate!(asciiprintable [false] true => String);
    });

    /// Test validator: ascii
    test!(ascii_validator_test = {
        // Invalid
        validate!(ascii [true] true => f64 100.3);
        validate!(ascii [true] true => String "ｆｏｏbar");
        validate!(ascii [true] true => String "ｘｙｚ０９８");
        validate!(ascii [true] true => String "１２３456");
        validate!(ascii [true] true => String "ｶﾀｶﾅ");
        validate!(ascii [true] true => i64 "foobar");
        // Valid
        validate!(ascii [false] true => String "");
        validate!(ascii [false] true => String " ");
        validate!(ascii [false] true => String "foobar");
        validate!(ascii [false] true => String "0987654321");
        validate!(ascii [false] true => String "test@example.com");
        validate!(ascii [false] true => String "1234abcDEF");
        validate!(ascii [false] true => String "\x19test\x7F");
        validate!(ascii [false] true => String "newline\n");
        // Value not set
        validate!(ascii [false] true => String);
    });

    /// Test validator: latitude
    test!(latitude_validator_test = {
        // Invalid
        validate!(latitude [true] true => String "");
        validate!(latitude [true] true => f64 100.3);
        validate!(latitude [true] true => String "test");
        validate!(latitude [true] true => String "+99.9");
        validate!(latitude [true] true => f64 99.9);
        validate!(latitude [true] true => i64 10);
        // Valid
        validate!(latitude [false] true => String "-90.000");
        validate!(latitude [false] true => String "+90");
        validate!(latitude [false] true => String "47.123123");
        validate!(latitude [false] true => String "-47.123123");
        // Value not set
        validate!(latitude [false] true => String);
    });

    /// Test validator: longitude
    test!(longitude_validator_test = {
        // Invalid
        validate!(longitude [true] true => String "");
        validate!(longitude [true] true => f64 100.3);
        validate!(longitude [true] true => String "test");
        validate!(longitude [true] true => String "180.1");
        validate!(longitude [true] true => String "+382.3811");
        validate!(longitude [true] true => String "23.11111111");
        // Valid
        validate!(longitude [false] true => String "-180.000");
        validate!(longitude [false] true => String "+99.9");
        validate!(longitude [false] true => String "+19.9");
        validate!(longitude [false] true => String "+73.234");
        validate!(longitude [false] true => String "23.111111");
        // Value not set
        validate!(longitude [false] true => String);
    });

    /// Test validator: ssn
    test!(ssn_validator_test = {
        // Invalid
        invalid! (ssn => String "");
        invalid! (ssn => f64 100.3);
        invalid! (ssn => String "test");
        invalid! (ssn => String "00-90-8787");
        invalid! (ssn => String "66690-76");
        // Valid
        valid! (ssn => String "191 60 2869");
        valid! (ssn => String "191-60-2869");
        // Value not set
        valid! (ssn => String);
    });

    /// /// Test validator: fields_equals
    test!(eq_field_validator_test = {
        let mut values = Map::new();
        values.assign("user[email]", Value::String(("test@google.com").into())).unwrap();
        values.assign("new[email]", Value::String(("test@google.com").into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "eq_field".to_string() => CompareField(values.find(&["new", "email"])).to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
        ));
        assert!(validator.get_errors().is_none());

        let mut values = Map::new();
        values.assign("user[email]", Value::String(("test@google.com").into())).unwrap();
        values.assign("new[email]", Value::String(("test@google.com.").into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "eq_field".to_string() => CompareField(values.find(&["new", "email"])).to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
        ));
        assert!(validator.get_errors().is_some());

        //=========================
        // Test types
        let mut values = Map::new();
        values.assign("item", Value::I64(-10)).unwrap();
        values.assign("new_item", Value::I64(-10)).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<i64>::new(btreemap! {
                "eq_field".to_string() => CompareField(values.find(&["new_item"])).to_json(),
                "vtype".to_string() => "i64".to_json(),
            }).validate("item".to_string(), values.find(&["item"])),
        ));
        assert!(validator.get_errors().is_none());

        let mut values = Map::new();
        values.assign("item", Value::U64(10)).unwrap();
        values.assign("new_item", Value::U64(20)).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<u64>::new(btreemap! {
                "eq_field".to_string() => CompareField(values.find(&["new_item"])).to_json(),
                "vtype".to_string() => "u64".to_json(),
            }).validate("item".to_string(), values.find(&["item"])),
        ));
        assert!(validator.get_errors().is_some());

        let mut values = Map::new();
        values.assign("item", Value::F64(-20.1)).unwrap();
        values.assign("new_item", Value::F64(30.4)).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<f64>::new(btreemap! {
                "eq_field".to_string() => CompareField(values.find(&["new_item"])).to_json(),
                "vtype".to_string() => "f64".to_json(),
            }).validate("item".to_string(), values.find(&["item"])),
        ));
        assert!(validator.get_errors().is_some());

        let mut values = Map::new();
        values.assign("item", Value::Boolean(true)).unwrap();
        values.assign("new_item", Value::Boolean(true)).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<bool>::new(btreemap! {
                "eq_field".to_string() => CompareField(values.find(&["new_item"])).to_json(),
                "vtype".to_string() => "bool".to_json(),
            }).validate("item".to_string(), values.find(&["item"])),
        ));
        assert!(validator.get_errors().is_none());

        // Valid - Value set to Null - no set
        // values should set as required
        let mut values = Map::new();
        values.assign("item", Value::Boolean(true)).unwrap();
        values.assign("new_item", Value::Null).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<bool>::new(btreemap! {
                "eq_field".to_string() => CompareField(values.find(&["new_item"])).to_json(),
                "vtype".to_string() => "bool".to_json(),
            }).validate("item".to_string(), values.find(&["item"])),
        ));
        assert!(validator.get_errors().is_none());

        // Value - new_item - explicitely not set
        // values should set as required
        let mut values = Map::new();
        values.assign("item", Value::Boolean(true)).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<bool>::new(btreemap! {
                "eq_field".to_string() => CompareField(values.find(&["new_item"])).to_json(),
                "vtype".to_string() => "bool".to_json(),
            }).validate("item".to_string(), values.find(&["item"])),
        ));
        assert!(validator.get_errors().is_none());

        // Base value set as Null - not set
        let mut values = Map::new();
        values.assign("item", Value::Null).unwrap();
        values.assign("new_item", Value::Boolean(true)).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<bool>::new(btreemap! {
                "eq_field".to_string() => CompareField(values.find(&["new_item"])).to_json(),
                "vtype".to_string() => "bool".to_json(),
            }).validate("item".to_string(), values.find(&["item"])),
        ));
        assert!(validator.get_errors().is_some());

        // Base value explicitely not set
        let mut values = Map::new();
        values.assign("new_item", Value::Boolean(true)).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<bool>::new(btreemap! {
                "eq_field".to_string() => CompareField(values.find(&["new_item"])).to_json(),
                "vtype".to_string() => "bool".to_json(),
            }).validate("item".to_string(), values.find(&["item"])),
        ));
        assert!(validator.get_errors().is_some());
        //=========================

        let mut values = Map::new();
        values.assign("new[email]", Value::String(("test@google.com.").into())).unwrap();

        // First field not set, then rool is not invoked
        // If we want added rool invokation we should added
        // additional validator - required
        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "eq_field".to_string() => CompareField(values.find(&["new", "email"])).to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
        ));
        assert!(validator.get_errors().is_some());

        let mut values = Map::new();
        values.assign("user[email]", Value::String(("test@google.com").into())).unwrap();

        // We should added required validator for 2-d field
        // if we want check fields equal
        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "eq_field".to_string() => CompareField(values.find(&["new", "email"])).to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
            Validator::<String>::new(btreemap! {
                "required".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("new_email".to_string(), values.find(&["new", "email"])),
        ));
        assert!(validator.get_errors().is_some());
        //=======
        let mut values = Map::new();
        values.assign("user[email]", Value::String(("test@google.com").into())).unwrap();
        values.assign("new[email]", Value::String(("test@google.com").into())).unwrap();

        // We should added required validator for 2-d field
        // if we want check fields equal
        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "eq_field".to_string() => CompareField(values.find(&["new", "email"])).to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
            Validator::<String>::new(btreemap! {
                "required".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("new_email".to_string(), values.find(&["new", "email"])),
        ));
        assert!(validator.get_errors().is_none());
    });

    /// Test validator: nor equals
    test!(ne_validator_test = {
        let rule = "test@google.com";
        // Value is not set
        validate!(ne [false] rule => String);
        // Valid value
        validate!(ne [false] rule => String "test");
        validate!(ne [false] 100.3 => f64 100.4);
        // Not valid value
        validate!(ne [true] rule => String "test@google.com");
        // Valid value - Valid type
        validate!(ne [true] 100.3 => f64 100.3);

        // Invalid value and invalid type
        let mut values = Map::new();
        values.assign("user[email]", Value::String(("test@google.com").into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<i64>::new(btreemap! {
                "ne".to_string() => 100.to_json(),
                "vtype".to_string() => "f64".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
        ));
        assert!(validator.get_errors().is_some());

        // Valid value- invalid type
        let mut values = Map::new();
        values.assign("user[email]", Value::F64(100.)).unwrap();

        let validator = ValidateResults(vec!(
            // Type should be f64 - but that is not error
            // valued different
            Validator::<i64>::new(btreemap! {
                "ne".to_string() => 100.to_json(),
                "vtype".to_string() => "f64".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
        ));
        assert!(validator.get_errors().is_none());

        // Valid value- invalid type
        let mut values = Map::new();
        values.assign("user[email]", Value::String(("test@google.com").into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<f64>::new(btreemap! {
                "ne".to_string() => 100.to_json(),
                "vtype".to_string() => "f64".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
        ));
        assert!(validator.get_errors().is_some());
    });

    /// Test validator: fields not equals
    test!(ne_field_validator_test = {
        let mut values = Map::new();
        values.assign("user[email]", Value::String(("test@google.com").into())).unwrap();
        values.assign("new[email]", Value::String(("test@google.com").into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "ne_field".to_string() => CompareField(values.find(&["new", "email"])).to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
        ));
        assert!(validator.get_errors().is_some());

        let mut values = Map::new();
        values.assign("user[email]", Value::String(("test@google.com").into())).unwrap();
        values.assign("new[email]", Value::String(("test@google.com.").into())).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "ne_field".to_string() => CompareField(values.find(&["new", "email"])).to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
        ));
        assert!(validator.get_errors().is_none());

        //=====================================
        // Check types
        let mut values = Map::new();
        values.assign("item", Value::U64(10)).unwrap();
        values.assign("new_item", Value::U64(20)).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<u64>::new(btreemap! {
                "ne_field".to_string() => CompareField(values.find(&["new_item"])).to_json(),
                "vtype".to_string() => "u64".to_json(),
            }).validate("item".to_string(), values.find(&["item"])),
        ));
        assert!(validator.get_errors().is_none());

        let mut values = Map::new();
        values.assign("item", Value::I64(-10)).unwrap();
        values.assign("new_item", Value::I64(-10)).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<i64>::new(btreemap! {
                "ne_field".to_string() => CompareField(values.find(&["new_item"])).to_json(),
                "vtype".to_string() => "i64".to_json(),
            }).validate("item".to_string(), values.find(&["item"])),
        ));
        assert!(validator.get_errors().is_some());

        let mut values = Map::new();
        values.assign("item", Value::F64(-10.3)).unwrap();
        values.assign("new_item", Value::F64(-10.4)).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<f64>::new(btreemap! {
                "ne_field".to_string() => CompareField(values.find(&["new_item"])).to_json(),
                "vtype".to_string() => "f64".to_json(),
            }).validate("item".to_string(), values.find(&["item"])),
        ));
        assert!(validator.get_errors().is_none());

        let mut values = Map::new();
        values.assign("item", Value::Boolean(true)).unwrap();
        values.assign("new_item", Value::Boolean(false)).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<bool>::new(btreemap! {
                "ne_field".to_string() => CompareField(values.find(&["new_item"])).to_json(),
                "vtype".to_string() => "bool".to_json(),
            }).validate("item".to_string(), values.find(&["item"])),
        ));
        assert!(validator.get_errors().is_none());

        // Value is set to Null - same as not set
        let mut values = Map::new();
        values.assign("item", Value::Null).unwrap();
        values.assign("new_item", Value::Boolean(false)).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<bool>::new(btreemap! {
                "ne_field".to_string() => CompareField(values.find(&["new_item"])).to_json(),
                "vtype".to_string() => "bool".to_json(),
            }).validate("item".to_string(), values.find(&["item"])),
        ));
        assert!(validator.get_errors().is_some());

        // Compared Value is set to Null
        // values should set as required
        let mut values = Map::new();
        values.assign("item", Value::Boolean(false)).unwrap();
        values.assign("new_item", Value::Null).unwrap();

        let validator = ValidateResults(vec!(
            Validator::<bool>::new(btreemap! {
                "ne_field".to_string() => CompareField(values.find(&["new_item"])).to_json(),
                "vtype".to_string() => "bool".to_json(),
            }).validate("item".to_string(), values.find(&["item"])),
        ));
        assert!(validator.get_errors().is_none());
        //=====================================

        let mut values = Map::new();
        values.assign("new[email]", Value::String(("test@google.com.").into())).unwrap();

        // First field not set, then rool is not invoked
        // If we want added rool invokation we should added
        // additional validator - required
        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "ne_field".to_string() => CompareField(values.find(&["new", "email"])).to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
        ));
        assert!(validator.get_errors().is_none());

        let mut values = Map::new();
        values.assign("user[email]", Value::String(("test@google.com").into())).unwrap();

        // We should added required validator for 2-d field
        // if we want check fields equal
        let validator = ValidateResults(vec!(
            Validator::<String>::new(btreemap! {
                "ne_field".to_string() => CompareField(values.find(&["new", "email"])).to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("user_email".to_string(), values.find(&["user", "email"])),
            Validator::<String>::new(btreemap! {
                "required".to_string() => true.to_json(),
                "vtype".to_string() => "string".to_json(),
            }).validate("new_email".to_string(), values.find(&["new", "email"])),
        ));
        assert!(validator.get_errors().is_some());
    });

    /// Test validator: alpha
    test!(alpha_validator_test = {
        // Invalid
        invalid! (alpha => String "");
        invalid! (alpha => f64 100.3);
        invalid! (alpha => String "00-90-8787");
        invalid! (alpha => String "abc123");
        invalid! (alpha => String "abc+");
        invalid! (alpha => String "abc123+");
        invalid! (alpha => String "abc®");
        invalid! (alpha => String "тест");
        invalid! (alpha => String "test test");
        // Valid
        valid! (alpha => String  "test");
        valid! (alpha => String  "az");
        // Value not set
        valid! (alpha => String);
    });

    /// Test validator: alphanum
    test!(alphanum_validator_test = {
        // Invalid
        invalid! (alphanum => String "");
        invalid! (alphanum => f64 100.3);
        invalid! (alphanum => String "00-90-8787");
        invalid! (alphanum => String "abc+");
        invalid! (alphanum => String "abc®");
        invalid! (alphanum => String "abc123+");
        invalid! (alphanum => String "тест");
        invalid! (alphanum => String "test test");
        // Valid
        valid! (alphanum => String  "test");
        valid! (alphanum => String  "az");
        valid! (alphanum => String  "123");
        valid! (alphanum => String  "test123");
        // Value not set
        valid! (alphanum => String);
    });

    /// Test validator: number
    test!(number_validator_test = {
        // Invalid
        invalid! (number => String "");
        invalid! (number => f64 100.3);
        invalid! (number => String "00-90-8787");
        invalid! (number => String "test");
        invalid! (number => String "abc+");
        invalid! (number => String "abc®");
        invalid! (number => String "abc123+");
        invalid! (number => String "тест");
        invalid! (number => String "test test");
        invalid! (number => String "asd123");
        invalid! (number => String "100.20");
        invalid! (number => String "-123");
        // Valid
        valid! (number => String  "123");
        // Value not set
        valid! (number => String);
    });

    /// Test validator: numeric
    test!(numeric_validator_test = {
        // Invalid
        invalid! (numeric => String "");
        invalid! (numeric => f64 100.3);
        invalid! (numeric => String "00-90-8787");
        invalid! (numeric => String "test");
        invalid! (numeric => String "abc+");
        invalid! (numeric => String "abc®");
        invalid! (numeric => String "abc123+");
        invalid! (numeric => String "тест");
        invalid! (numeric => String "test test");
        invalid! (numeric => String "asd123");
        // Valid
        valid! (numeric => String  "123");
        valid! (numeric => String  "-123");
        valid! (numeric => String  "123.20");
        valid! (numeric => String  "-123.20");
        // Value not set
        valid! (numeric => String);
    });

    /// Test validator: alphaunicode
    test!(alphaunicode_validator_test = {
        // Invalid
        invalid! (alphaunicode => String "");
        invalid! (alphaunicode => f64 100.3);
        invalid! (alphaunicode => String "this is a test string");
        invalid! (alphaunicode => String "123");
        invalid! (alphaunicode => String "test123");
        invalid! (alphaunicode => String "тест123");
        invalid! (alphaunicode => String "<>@;.-=");
        invalid! (alphaunicode => String "ひらがな・カタカナ、．漢字");
        invalid! (alphaunicode => String "test＠example.com");
        invalid! (alphaunicode => String "1234abcDE");
        // Valid
        valid! (alphaunicode => String "test");
        valid! (alphaunicode => String "这是一个测试字符串");
        valid! (alphaunicode => String "あいうえおfoobar");
        valid! (alphaunicode => String "Тестирование");
        valid! (alphaunicode => String "ｶﾀｶﾅ");
        // Value not set
        valid! (alphaunicode => String);
    });

    /// Test validator: alphanumunicode
    test!(alphanumunicode_validator_test = {
        // Invalid
        invalid! (alphanumunicode => String "");
        invalid! (alphanumunicode => f64 100.3);
        invalid! (alphanumunicode => String "this is a test string");
        invalid! (alphanumunicode => String "<>@;.-=");
        invalid! (alphanumunicode => String "ひらがな・カタカナ、．漢字");
        invalid! (alphanumunicode => String "test＠example.com");
        // Valid
        valid! (alphanumunicode => String "test");
        valid! (alphanumunicode => String "123");
        valid! (alphanumunicode => String "这是一个测试字符串");
        valid! (alphanumunicode => String "\u{0031}\u{0032}\u{0033}"); // unicode 5
        valid! (alphanumunicode => String "あいうえおfoobar");
        valid! (alphanumunicode => String "Тестирование");
        valid! (alphanumunicode => String "ｶﾀｶﾅ");
        valid! (alphanumunicode => String "1234abcDE");
        // Value not set
        valid! (alphanumunicode => String);
    });

    /// Test validator: hexadecimal
    test!(hexadecimal_validator_test = {
        // Invalid
        invalid! (hexadecimal => String "");
        invalid! (hexadecimal => f64 100.3);
        invalid! (hexadecimal => String "test");
        invalid! (hexadecimal => String "qwe123");
        invalid! (hexadecimal => String "Тест");
        invalid! (hexadecimal => String "1A23c2B5eFdg");
        // Valid
        valid! (hexadecimal => String "acdf1234");
        valid! (hexadecimal => String "123");
        valid! (hexadecimal => String "1AF123BD23");
        valid! (hexadecimal => String "1A23c2B5eFd");
        // Value not set
        valid! (hexadecimal => String);
    });

    /// Test validator: hexcolor
    test!(hexcolor_validator_test = {
        // Invalid
        invalid! (hexcolor => String "");
        invalid! (hexcolor => f64 100.3);
        invalid! (hexcolor => String "test");
        invalid! (hexcolor => String "qwe123");
        invalid! (hexcolor => String "Тест");
        invalid! (hexcolor => String "1A23c2B5eFdg");
        invalid! (hexcolor => String "#c2c2g2");
        invalid! (hexcolor => String "fff");
        invalid! (hexcolor => String "fffFFF");
        invalid! (hexcolor => String "#ABCDEF123");
        invalid! (hexcolor => String "#c2c2c2c");
        // Valid
        valid! (hexcolor => String "#c2c2c2");
        valid! (hexcolor => String "#fff");
        valid! (hexcolor => String "#ffF");
        valid! (hexcolor => String "#000");
        // Value not set
        valid! (hexcolor => String);
    });

    /// Test validator: rgb
    test!(rgb_validator_test = {
        // Invalid
        invalid! (rgb => String "");
        invalid! (rgb => f64 100.3);
        invalid! (rgb => String "test");
        invalid! (rgb => String "qwe123");
        invalid! (rgb => String "Тест");
        invalid! (rgb => String "1A23c2B5eFdg");
        invalid! (rgb => String "#c2c2g2");
        invalid! (rgb => String "fff");
        invalid! (rgb => String "fffFFF");
        invalid! (rgb => String "#ABCDEF123");
        invalid! (rgb => String "#c2c2c2c");
        invalid! (rgb => String "rgb(10%,  50%, 55)");
        invalid! (rgb => String "rgb(10%,  50%, 55)");
        invalid! (rgb => String "rgb(1,349,275)");
        invalid! (rgb => String "rgb(01,31,255)");
        invalid! (rgb => String "rgba(0,31,255)");
        invalid! (rgb => String "rgb(255, 256, 255)");
        invalid! (rgb => String "rgb(-10, 100, 200)");
        // Valid
        valid! (rgb => String "rgb(0,31,255)");
        valid! (rgb => String "rgb(0,  31, 255)");
        valid! (rgb => String "rgb(255, 255, 255)");
        // Value not set
        valid! (rgb => String);
    });

    /// Test validator: rgba
    test!(rgba_validator_test = {
        // Invalid
        invalid! (rgba => String "");
        invalid! (rgba => f64 100.3);
        invalid! (rgba => String "test");
        invalid! (rgba => String "qwe123");
        invalid! (rgba => String "Тест");
        invalid! (rgba => String "1A23c2B5eFdg");
        invalid! (rgba => String "#c2c2g2");
        invalid! (rgba => String "fff");
        invalid! (rgba => String "fffFFF");
        invalid! (rgba => String "#ABCDEF123");
        invalid! (rgba => String "#c2c2c2c");
        invalid! (rgba => String "rgb(1, 31, 255)");
        invalid! (rgba => String "rgb(255, 256, 255)");
        invalid! (rgba => String "rgb(100, 100, 100, 0.1)");
        invalid! (rgba => String "rgba(255, 256, 255)");
        invalid! (rgba => String "rgba(255, 256, 255, 0.1)");
        invalid! (rgba => String "rgba(01, 100, 100, 0.1)");
        invalid! (rgba => String "rgba(12%,55,100%,0.12)");
        invalid! (rgba => String "rgb(1,349,275,0.5)");
        invalid! (rgba => String "rgb(01,31,255,0.5)");
        invalid! (rgba => String "rgba(-1, 31,255, 0.5)");
        invalid! (rgba => String "rgba(255, 255, 255, 2)");
        invalid! (rgba => String "rgba(255, 257, 255, 0.5)");
        // Valid
        valid! (rgba => String "rgba(0,31,255,0.5)");
        valid! (rgba => String "rgba( 0, 31, 255, 0.5 )");
        valid! (rgba => String "rgba(0,  31, 255, 0.12)");
        valid! (rgba => String "rgba(255, 255, 255, 1)");
        valid! (rgba => String "rgba(12%,55%,100%,0.12)");
        valid! (rgba => String "rgba(0,  31, 255, 0)");
        // Value not set
        valid! (rgba => String);
    });

    /// Test validator: hsl
    test!(hsl_validator_test = {
        // Invalid
        invalid! (hsl => String "");
        invalid! (hsl => f64 100.3);
        invalid! (hsl => String "test");
        invalid! (hsl => String "qwe123");
        invalid! (hsl => String "Тест");
        invalid! (hsl => String "hsl(361,100%,50%)");
        invalid! (hsl => String "hsl(361,101%,50%)");
        invalid! (hsl => String "hsl(361,100%,101%)");
        invalid! (hsl => String "hsl(-10,100%,100%)");
        // Valid
        valid! (hsl => String "hsl(360,100%,50%)");
        valid! (hsl => String "hsl(0,0%,0%)");
        // Value not set
        valid! (hsl => String);
    });

    /// Test validator: hsla
    test!(hsla_validator_test = {
        // Invalid
        invalid! (hsla => String "");
        invalid! (hsla => f64 100.3);
        invalid! (hsla => String "test");
        invalid! (hsla => String "qwe123");
        invalid! (hsla => String "Тест");
        invalid! (hsla => String "hsl(361,100%,50%,1)");
        invalid! (hsla => String "hsl(361,100%,50%)");
        invalid! (hsla => String "hsla(361,100%,50%)");
        invalid! (hsla => String "hsla(360,101%,50%)");
        invalid! (hsla => String "hsla(360,100%,101%)");
        invalid! (hsla => String "hsla(-360, 100%, 100%, 1)");
        // Valid
        valid! (hsla => String "hsla(360,100%,100%,1)");
        valid! (hsla => String "hsla(360, 100%, 100%, 1)");
        valid! (hsla => String "hsla(360,100%,100%,0.5)");
        valid! (hsla => String "hsla(0,0%,0%, 0)");
        // Value not set
        valid! (hsla => String);
    });

    /// Test validator: len
    test!(len_validator_test = {
        // Invalid
        invalid! (len 4 => String "");
        invalid! (len 4 => bool true);
        invalid! (len 0 => i64 0);
        // Valid
        valid! (len 4 => String "test");
        valid! (len 4 => u64 4u64);
        valid! (len 4 => i64 4i64);
        valid! (len 4 => f64 4.0f64);
        // Value not set
        valid! (len 4 => String);
    });

    /// Test validator: contains
    test!(contains_validator_test = {
        // Invalid
        invalid! (contains "test" => String "");
        invalid! (contains "tset" => f64 100.3);
        invalid! (contains "new" => String "test");
        invalid! (contains "test" => String "Тест");
        // Valid
        valid! (contains "te" => String "test");
        valid! (contains "ест" => String "Тест");
        valid! (contains "@" => String "test@google.com");
        // Value not set
        valid! (contains "test" => String);
    });

}
