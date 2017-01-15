//! # What is Validator
//! It useful for Form and Model validation.
//! Implementation for Validation via BTreeMap structure.
//! It consist basic validations rule, validatior, and
//! validation Result and validation Errors.
//!
//! ## How to use
//! Most common examples you can find at src/admin/models/*
//!
//! ```ignore
//! fn validate(values: &Map) -> ValidateResults {
//!    ValidateResults(vec!(
//!        // Validator declaration
//!        // `::<String>::` meen Generic type for Default value
//!        Validator::<String>::new(btreemap! {
//!            // Validator type
//!            "required".to_string() => true.to_json(),
//!            // Value type (from POST/GET/.. request)
//!            // We'll get validation error if value type wrong.
//!            // It is required field for validations rule
//!            "vtype".to_string() => "string".to_json(),
//!        }).validate("title".to_string(), values.find(&["pages", "title"])),
//!
//!        // Validator declaration
//!        // `::<bool>::` meen Generic type for Default value
//!        // and it should be same type as default value
//!        Validator::<bool>::new(btreemap! {
//!            // Validator type declaration should be same
//!            // as default value type
//!            "default".to_string() => false.to_json(),
//!            "vtype".to_string() => "bool".to_json(),
//!        }).validate("published".to_string(), values.find(&["published"])),
//!    ))
//! }
//! ```

pub use rustc_serialize::json::{self, Json, ToJson};
pub use rustc_serialize::json::DecoderError::*;
pub use rustc_serialize::Decodable;
use params::{Value, FromValue};
use super::render::{BaseDataMap, BaseDataMapDecoder};
use std::collections::BTreeMap;
use std::string::String;
use regex::Regex;
use std::fmt::{Display};

/// Base Validator struct
#[derive(RustcDecodable, Debug)]
pub struct Validator<T: Display> {
    /// Type of validator
    /// For example: string, bool, i64 etc.
    pub vtype: String,
    pub required: Option<bool>,
    pub not_empty: Option<bool>,
    pub min: Option<i64>,
    pub max: Option<u64>,
    pub len: Option<u64>,
    pub email: Option<bool>,
    pub url: Option<bool>,
    pub regexp: Option<String>,
    pub ssn: Option<bool>,
    pub longitude: Option<bool>,
    pub latitude: Option<bool>,
    pub ascii: Option<bool>,
    pub asciiprintable: Option<bool>,
    pub uuid: Option<bool>,
    pub uuid3: Option<bool>,
    pub uuid4: Option<bool>,
    pub uuid5: Option<bool>,
    pub eq: Option<T>,
    pub eq_field: Option<T>,
    pub ne: Option<T>,
    pub ne_field: Option<T>,
    pub alpha: Option<bool>,
    pub alphanum: Option<bool>,
    pub alphaunicode: Option<bool>,
    pub alphanumunicode: Option<bool>,
    pub numeric: Option<bool>,
    pub number: Option<bool>,
    pub hexadecimal: Option<bool>,
    pub hexcolor: Option<bool>,
    pub rgb: Option<bool>,
    pub rgba: Option<bool>,
    pub hsl: Option<bool>,
    pub hsla: Option<bool>,
    pub contains: Option<String>,
    pub default: Option<T>,
    errors: Option<ErrorValidator>,
}

/// Validation result enum.
/// Consist Values and Errors
#[derive(Debug)]
pub struct ValidateResult(BaseDataMap, ErrorValidator);
/// Array of Validatopn Results for all validatior
#[derive(Debug)]
pub struct ValidateResults(pub Vec<ValidateResult>);
// Error result aggregator
pub type ErrorsResult = Option<Vec<ErrorValidator>>;

pub struct CompareField<'a>(pub Option<&'a Value>);

/// Validation results methods
impl ValidateResults {
    /// Get Validation Errors result
    pub fn get_errors(&self) -> ErrorsResult {
        let &ValidateResults(ref results) = self;
        let mut errors = vec!();
        for &ValidateResult(_, ref err) in results {
            if err.errors_count.is_some() {
                errors.push(err.to_owned());
            }
        }
        if errors.len() > 0 { Some(errors) } else { None }
    }

    /// Get Validation Values result
    pub fn get_values(&self) -> BaseDataMap {
        let &ValidateResults(ref results) = self;
        let mut values: BaseDataMap = BTreeMap::new();
        for &ValidateResult(ref val, _) in results {
            values.append(&mut val.clone());
        }
        values
    }
}

impl <'a> ToJson for CompareField<'a> {
    fn to_json(&self) -> Json {
        let &CompareField(ref value) = self;
        match *value {
            Some(&Value::String(ref value)) => {
                value.to_json()
            },
            Some(&Value::U64(value)) => {
                value.to_json()
            },
            Some(&Value::I64(value)) => {
                value.to_json()
            },
            Some(&Value::F64(value)) => {
                value.to_json()
            },
            Some(&Value::Boolean(value)) => {
                value.to_json()
            },
            _ => Json::Null
        }
    }
}

/// Convert Validation Results to Json
impl ToJson for ValidateResults {
    fn to_json(&self) -> Json {
        let mut data: Vec<Json> = vec!();
        let &ValidateResults(ref results) = self;
        let mut i = 0;
        for &ValidateResult(ref val, ref err) in results {
            let mut d = BTreeMap::new();
            // Set `field` attribute
            d.insert("field".to_string(), err.field.to_json());
            // Set `errors` attribute
            d.insert("errors".to_string(), err.errors.to_json());
            // Set `values` attribute
            d.insert("values".to_string(), val.to_json());

            data.push(d.to_json());

            //data.append(&mut d);
            i = i + 1;
        }
        Json::Array(data)
    }
}

/// Validator methods
/// It depends from various additional **traits**.
impl<T: FromValue + ToJson + Decodable + Display> Validator<T> {
    /// Init Validation rule
    pub fn new(validator_rules: BaseDataMap) -> Validator<T> {
        validator_rules.decode()
    }

    /// Main validor for all validations rules
    pub fn validate(&mut self, field: String, value: Option<&Value>) -> ValidateResult {
        let mut value: Option<Value> = if let Some(val) = value {
            Some(val.to_owned())
        } else { None };
        // Init Errors
        self.errors = Some(ErrorValidator::new(&field));

        // Invoke validators
        self.required(&value);
        self.not_empty(&value);
        self.max(&value);
        self.min(&value);
        self.len(&value);
        self.email(&value);
        self.url(&value);
        self.regexp(&value);
        self.ssn(&value);
        self.longitude(&value);
        self.latitude(&value);
        self.ascii(&value);
        self.asciiprintable(&value);
        self.uuid(&value);
        self.uuid3(&value);
        self.uuid4(&value);
        self.uuid5(&value);
        self.eq(&value);
        self.eq_field(&value);
        self.ne(&value);
        self.ne_field(&value);
        self.alpha(&value);
        self.alphanum(&value);
        self.alphaunicode(&value);
        self.alphanumunicode(&value);
        self.numeric(&value);
        self.number(&value);
        self.hexadecimal(&value);
        self.hexcolor(&value);
        self.rgb(&value);
        self.rgba(&value);
        self.hsl(&value);
        self.hsla(&value);
        self.contains(&value);
        value = self.default(&value);

        let json_value: Json = match self.type_cast(&value) {
            Some(ref json_value) => json_value.to_owned(),
            None => {
                if let Some(ref mut error) = self.errors {
                    if value.is_some() {
                        let msg = format!("Field wrong type: {}", error.field);
                        error.add(msg);
                    }
                }
                Json::Null
            }
        };

        let mut err = ErrorValidator::new(&field);
        if let Some(ref err_results) = self.errors {
            err = err_results.to_owned();
        }

        ValidateResult(btreemap! {
            field.to_owned() => json_value
        }, err)
    }

    /// Requered validator
    fn required(&mut self, value: &Option<Value>) {
        if self.required.is_some() {
            if value.is_some() {
                let check_value = match *value {
                    Some(Value::String(ref value)) => {
                        !value.trim().is_empty()
                    },
                    _ => true
                };
                if check_value {
                    return ()
                }
            }
            if let Some(ref mut error) = self.errors {
                let msg = format!("Field required: {}", error.field);
                error.add(msg);
            }
        }
    }

    /// Not Empty validator
    fn not_empty(&mut self, value: &Option<Value>) {
        // We apply validarot onlu if values is set
        // Requered validator analize if value was set
        if self.not_empty.is_some() && value.is_some() {
            let is_empty = match *value {
                // We check only strings
                Some(Value::String(ref value)) => {
                    value.trim().is_empty()
                },
                _ => false
            };
            if is_empty {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field can't be empty: {}", error.field);
                    error.add(msg);
                }
            }
        }
    }

    /// Max value validator
    /// Multitype
    fn max(&mut self, value: &Option<Value>) {
        if self.max.is_some() && value.is_some() {
            let mut required_value: u64 = 0;
            if let Some(max) = self.max {
                if max == 0 {
                    if let Some(ref mut error) = self.errors {
                        let msg = format!("Validation value can't be equal: {}", max);
                        error.add(msg);
                    }
                    return ()
                }
                required_value = max;
            }
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    value.chars().count() as u64 <= required_value
                },
                Some(Value::U64(value)) => {
                    value <= required_value
                },
                Some(Value::I64(value)) => {
                    value as u64 <= required_value
                },
                Some(Value::F64(value)) => {
                    value as u64 <= required_value
                },
                Some(Value::Boolean(value)) => {
                    value as u64 <= required_value
                },
                _ => false
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} can't be min then: {}", error.field, required_value);
                    error.add(msg);
                }
            }
        }
    }

    /// Min value validator
    /// Multitype
    fn min(&mut self, value: &Option<Value>) {
        if self.min.is_some() && value.is_some() {
            let mut required_value: i64 = 0;
            if let Some(min) = self.min {
                if let Some(max) = self.max {
                    if min >= max as i64 {
                        if let Some(ref mut error) = self.errors {
                            let msg = format!("Validation rule {} can't be greater or equal max rule: {}", min, max);
                            error.add(msg);
                        }
                        return ()
                    }
                }
                required_value = min;
            }
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    value.chars().count() as i64 >= required_value
                },
                Some(Value::U64(value)) => {
                    value as i64 >= required_value
                },
                Some(Value::I64(value)) => {
                    value >= required_value
                },
                Some(Value::F64(value)) => {
                    value as i64 >= required_value
                },
                _ => false
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} can't be min then: {}", error.field, required_value);
                    error.add(msg);
                }
            }
        }
    }

    /// Length value validator
    /// Multitype
    /// For numbers and bool, length will ensure that
    /// the value is equal to the parameter given.
    /// For strings, it checks that the string length
    /// is exactly that number of characters.
    fn len(&mut self, value: &Option<Value>) {
        if self.len.is_some() && value.is_some() {
            let mut required_value: u64 = 0;
            if let Some(len) = self.len {
                if len <= 0 {
                    if let Some(ref mut error) = self.errors {
                        let msg = format!("Validation value can't be equal: {}", len);
                        error.add(msg);
                    }
                    return ()
                }
                required_value = len;
            }
            let mut numeric = true;
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    numeric = false;
                    value.chars().count() as u64 == required_value
                },
                Some(Value::U64(value)) => {
                    value == required_value
                },
                Some(Value::I64(value)) => {
                    value as u64 == required_value
                },
                Some(Value::F64(value)) => {
                    value as u64 == required_value
                },
                _ => false
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg;
                    if numeric {
                        // For numbers
                        msg = format!("Field {} value should be equal: {}", error.field, required_value);
                    } else {
                        // For strings
                        msg = format!("Field {} value should be equal length: {}", error.field, required_value);
                    }
                    error.add(msg);
                }
            }
        }
    }

    /// Equals value validator
    /// Multitype
    /// For strings & numbers, eq will ensure that the
    /// value is equal to the parameter given.
    fn eq(&mut self, value: &Option<Value>) {
        if value.is_some() {
            let required_value = if let Some(ref required_value) = self.eq {
                required_value
            } else {
                return ()
            };

            let is_valid = self.compare(&value, &required_value.to_json());
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} value should be equal: {}", error.field, required_value);
                    error.add(msg);
                }
            }
        }
    }

    /// Equals to another fiold validator
    /// Multitype
    fn eq_field(&mut self, value: &Option<Value>) {
        if value.is_some() && self.eq_field.is_some()  {
            let (required_value, is_valid) = if let Some(ref required_value) = self.eq_field {
                let is_valid = self.compare(&value, &required_value.to_json());
                (required_value.to_string(), is_valid)
            } else {
                ("".to_string(), false)
            };

            let value_str = match *value {
                Some(Value::String(ref value)) => {
                    value.to_string()
                },
                Some(Value::U64(value)) => {
                    value.to_string()
                },
                Some(Value::I64(value)) => {
                    value.to_string()
                },
                Some(Value::F64(value)) => {
                    value.to_string()
                },
                Some(Value::Boolean(value)) => {
                    value.to_string()
                },
                _ => "".to_string()
            };

            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} with value {} should be equal field value {}", error.field, value_str, required_value);
                    error.add(msg);
                }
            }
        }
    }

    /// Not Equals value validator
    /// Multitype
    /// For strings & numbers, eq will ensure that the
    /// value is not equal to the parameter given.
    fn ne(&mut self, value: &Option<Value>) {
        if value.is_some() && self.ne.is_some() {
            let required_value = if let Some(ref required_value) = self.ne {
                required_value
            } else {
                return ()
            };

            let is_valid = !self.compare(&value, &required_value.to_json());
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} value should be not equal: {}", error.field, required_value);
                    error.add(msg);
                }
            }
        }
    }

    /// Not qquals to another fiold validator
    /// Multitype
    fn ne_field(&mut self, value: &Option<Value>) {
        if value.is_some() && self.ne_field.is_some()  {
            let (required_value, is_valid) = if let Some(ref required_value) = self.ne_field {
                let is_valid = !self.compare(&value, &required_value.to_json());
                (required_value.to_string(), is_valid)
            } else {
                ("".to_string(), false)
            };

            let value_str = match *value {
                Some(Value::String(ref value)) => {
                    value.to_string()
                },
                Some(Value::U64(value)) => {
                    value.to_string()
                },
                Some(Value::I64(value)) => {
                    value.to_string()
                },
                Some(Value::F64(value)) => {
                    value.to_string()
                },
                Some(Value::Boolean(value)) => {
                    value.to_string()
                },
                _ => "".to_string()
            };

            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} with value {} should be not equal field value {}", error.field, value_str, required_value);
                    error.add(msg);
                }
            }
        }
    }

    /// Alpha Only validator
    /// This validates that a string value contains
    /// ASCII alpha characters only
    fn alpha(&mut self, value: &Option<Value>) {
        if self.alpha.is_some() && value.is_some() {
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    let re = Regex::new(r"^[a-zA-Z]+$").unwrap();
                    re.is_match(value)
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} should contain ASCII alpha characters only", error.field);
                    error.add(msg);
                }
            }
        }
    }

    /// Alphanumeric validator
    /// This validates that a string value contains
    /// ASCII alpha characters and numbers only
    fn alphanum(&mut self, value: &Option<Value>) {
        if self.alphanum.is_some() && value.is_some() {
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    let re = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
                    re.is_match(value)
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} should contain ASCII alpha characters and numbers only", error.field);
                    error.add(msg);
                }
            }
        }
    }

    /// Alpha Unicode validator
    /// This validates that a string value contains
    /// unicode alpha characters only
    fn alphaunicode(&mut self, value: &Option<Value>) {
        if self.alphaunicode.is_some() && value.is_some() {
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    let re = Regex::new(r"^[\p{L}]+$").unwrap();
                    re.is_match(value)
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} should contain Unicode alpha characters", error.field);
                    error.add(msg);
                }
            }
        }
    }

    /// Alphanumeric Unicode validator
    ///
    /// This validates that a string value contains unicode
    /// alphanumeric characters only
    fn alphanumunicode(&mut self, value: &Option<Value>) {
        if self.alphanumunicode.is_some() && value.is_some() {
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    let re = Regex::new(r"^[\p{L}\p{N}]+$").unwrap();
                    re.is_match(value)
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} should contain Unicode alpha characters and numbers", error.field);
                    error.add(msg);
                }
            }
        }
    }

    /// Numeric validator
    ///
    /// This validates that a string value contains a
    /// basic numeric value. basic excludes exponents etc...
    fn numeric(&mut self, value: &Option<Value>) {
        if self.numeric.is_some() && value.is_some() {
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    let re = Regex::new(r"^[-+]?[0-9]+(?:\.[0-9]+)?$").unwrap();
                    re.is_match(value)
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} should contain Numeric characters only", error.field);
                    error.add(msg);
                }
            }
        }
    }

    /// Hexadecimal string validator
    ///
    /// This validates that a string value contains
    /// a valid hexadecimal.
    fn hexadecimal(&mut self, value: &Option<Value>) {
        if self.hexadecimal.is_some() && value.is_some() {
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    let re = Regex::new(r"^[0-9a-fA-F]+$").unwrap();
                    re.is_match(value)
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} should contain Hexadecimal characters only", error.field);
                    error.add(msg);
                }
            }
        }
    }

    /// Hexcolor String validator
    ///
    /// This validates that a string value contains
    /// a valid hex color including hashtag (#)
    fn hexcolor(&mut self, value: &Option<Value>) {
        if self.hexcolor.is_some() && value.is_some() {
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    let re = Regex::new(r"^#(?:[0-9a-fA-F]{3}|[0-9a-fA-F]{6})$").unwrap();
                    re.is_match(value)
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} should contain Hexcolor characters only", error.field);
                    error.add(msg);
                }
            }
        }
    }

    /// RGB String validator
    ///
    /// This validates that a string value contains
    /// a valid rgb color
    fn rgb(&mut self, value: &Option<Value>) {
        if self.rgb.is_some() && value.is_some() {
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    let re = Regex::new(r"^rgb\(\s*(?:(?:0|[1-9]\d?|1\d\d?|2[0-4]\d|25[0-5])\s*,\s*(?:0|[1-9]\d?|1\d\d?|2[0-4]\d|25[0-5])\s*,\s*(?:0|[1-9]\d?|1\d\d?|2[0-4]\d|25[0-5])|(?:0|[1-9]\d?|1\d\d?|2[0-4]\d|25[0-5])%\s*,\s*(?:0|[1-9]\d?|1\\d\d?|2[0-4]\d|25[0-5])%\s*,\s*(?:0|[1-9]\d?|1\d\d?|2[0-4]\d|25[0-5])%)\s*\)$").unwrap();
                    re.is_match(value)
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} should contain RGB color characters only", error.field);
                    error.add(msg);
                }
            }
        }
    }

    /// RGBA String validator
    ///
    /// This validates that a string value contains
    /// a valid rgba color
    fn rgba(&mut self, value: &Option<Value>) {
        if self.rgba.is_some() && value.is_some() {
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    let re = Regex::new(r"rgba\(\s*(?:(?:0|[1-9]\d?|1\d\d?|2[0-4]\d|25[0-5])\s*,\s*(?:0|[1-9]\d?|1\d\d?|2[0-4]\d|25[0-5])\s*,\s*(?:0|[1-9]\d?|1\d\d?|2[0-4]\d|25[0-5])|(?:0|[1-9]\d?|1\d\d?|2[0-4]\d|25[0-5])%\s*,\s*(?:0|[1-9]\d?|1\d\d?|2[0-4]\d|25[0-5])%\s*,\s*(?:0|[1-9]\d?|1\d\d?|2[0-4]\d|25[0-5])%)\s*,\s*(?:(?:0.[1-9]*)|[01])\s*\)$").unwrap();
                    re.is_match(value)
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} should contain RGBa color characters only", error.field);
                    error.add(msg);
                }
            }
        }
    }

    /// HSL String validator
    ///
    /// This validates that a string value contains
    /// a valid hsl color
    fn hsl(&mut self, value: &Option<Value>) {
        if self.hsl.is_some() && value.is_some() {
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    let re = Regex::new(r"^hsl\(\s*(?:0|[1-9]\d?|[12]\d\d|3[0-5]\d|360)\s*,\s*(?:(?:0|[1-9]\d?|100)%)\s*,\s*(?:(?:0|[1-9]\d?|100)%)\s*\)$").unwrap();
                    re.is_match(value)
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} should contain HSL color characters only", error.field);
                    error.add(msg);
                }
            }
        }
    }

    /// HSLa String validator
    ///
    /// This validates that a string value contains
    /// a valid hsla color
    fn hsla(&mut self, value: &Option<Value>) {
        if self.hsla.is_some() && value.is_some() {
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    let re = Regex::new(r"hsla\(\s*(?:0|[1-9]\d?|[12]\d\d|3[0-5]\d|360)\s*,\s*(?:(?:0|[1-9]\d?|100)%)\s*,\s*(?:(?:0|[1-9]\d?|100)%)\s*,\s*(?:(?:0.[1-9]*)|[01])\s*\)$").unwrap();
                    re.is_match(value)
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} should contain HSLa color characters only", error.field);
                    error.add(msg);
                }
            }
        }
    }

    /// Contains String validator
    ///
    /// This validates that a string value contains the
    /// substring value.
    fn contains(&mut self, value: &Option<Value>) {
        if self.contains.is_some() && value.is_some() {
            let required_value = match self.contains {
                Some(ref val) => val,
                _ => return (),
            };
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    value.contains(required_value)
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} should contain {} string.", error.field, required_value);
                    error.add(msg);
                }
            }
        }
    }

    /// number validator
    ///
    /// This validates that a string value contains a
    /// numbers only.
    fn number(&mut self, value: &Option<Value>) {
        if self.number.is_some() && value.is_some() {
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    let re = Regex::new(r"^[0-9]+$").unwrap();
                    re.is_match(value)
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} should contain Numbers characters only", error.field);
                    error.add(msg);
                }
            }
        }
    }

    /// E-mail validator
    fn email(&mut self, value: &Option<Value>) {
        if self.email.is_some() && value.is_some() {
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    let re = Regex::new(r"\A(?i)[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\z").unwrap();
                    re.is_match(value)
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} is not valid e-mail address", error.field);
                    error.add(msg);
                }
            }
        }
    }

    /// URL validator
    /// Algorithm: https://mathiasbynens.be/demo/url-regex
    /// Author of algorithm: @imme_emosol
    fn url(&mut self, value: &Option<Value>) {
        if self.url.is_some() && value.is_some() {
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    let re = Regex::new(r"\A(?i)(https?|ftp)://(-\.)?([^\s/?\.#-]+\.?)+(/[^\s]*)?\z").unwrap();
                    re.is_match(value)
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} is not valid URL", error.field);
                    error.add(msg);
                }
            }
        }
    }

    /// Regular expression value validator
    fn regexp(&mut self, value: &Option<Value>) {
        if value.is_none() {
            return
        }
        if let Some(ref regexp) = self.regexp {
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    let re = Regex::new(regexp).unwrap();
                    re.is_match(value)
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} is not valid regexp rules", error.field);
                    error.add(msg);
                }
            }
        }
    }

    /// SSN expression validator
    /// SSN: Social Security Number
    fn ssn(&mut self, value: &Option<Value>) {
        if self.ssn.is_some() && value.is_some() {
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    let re = Regex::new(r"^\d{3}[-\s]?\d{2}[-\s]?\d{4}$").unwrap();
                    re.is_match(value)
                        && value.chars().count() as u64 == 11
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} is not valid SSN", error.field);
                    error.add(msg);
                }
            }
        }
    }

    /// Longitude validator
    fn longitude(&mut self, value: &Option<Value>) {
        if self.longitude.is_some() && value.is_some() {
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    let re = Regex::new(r"^(\+|-)?(?:180(?:(?:\.0{1,6})?)|(?:[0-9]|[1-9][0-9]|1[0-7][0-9])(?:(?:\.[0-9]{1,6})?))$").unwrap();
                    re.is_match(value)
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} is not valid Longitude", error.field);
                    error.add(msg);
                }
            }
        }
    }

    /// Latitude validator
    fn latitude(&mut self, value: &Option<Value>) {
        if self.latitude.is_some() && value.is_some() {
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    let re = Regex::new(r"^(\+|-)?(?:90(?:(?:\.0{1,6})?)|(?:[0-9]|[1-8][0-9])(?:(?:\.[0-9]{1,6})?))$").unwrap();
                    re.is_match(value)
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} is not valid Latitude", error.field);
                    error.add(msg);
                }
            }
        }
    }

    /// Printable ASCII
    /// This validates that a string value contains only
    /// printable ASCII characters.
    /// NOTE: if the string is blank, this validates as true.
    fn asciiprintable(&mut self, value: &Option<Value>) {
        if self.asciiprintable.is_some() && value.is_some() {
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    let re = Regex::new(r"^[\x20-\x7E]*$").unwrap();
                    re.is_match(value)
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} is not valid ASCII printable", error.field);
                    error.add(msg);
                }
            }
        }
    }

    /// Printable ASCII
    /// This validates that a string value contains only ASCII
    /// characters (including not printable).
    /// NOTE: if the string is blank, this validates as true.
    fn ascii(&mut self, value: &Option<Value>) {
        if self.ascii.is_some() && value.is_some() {
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    let re = Regex::new(r"^[\x00-\x7F]*$").unwrap();
                    re.is_match(value)
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} is not valid ASCII", error.field);
                    error.add(msg);
                }
            }
        }
    }

    /// UUID validator
    /// UUID: Universally Unique Identifier
    fn uuid(&mut self, value: &Option<Value>) {
        if self.uuid.is_some() && value.is_some() {
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    let re = Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap();
                    re.is_match(value)
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} is not valid UUID", error.field);
                    error.add(msg);
                }
            }
        }
    }

    /// UUID v3 validator
    /// UUID v3: Universally Unique Identifier version 3
    fn uuid3(&mut self, value: &Option<Value>) {
        if self.uuid3.is_some() && value.is_some() {
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    let re = Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-3[0-9a-f]{3}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap();
                    re.is_match(value)
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} is not valid UUID v3", error.field);
                    error.add(msg);
                }
            }
        }
    }

    /// UUID v4 validator
    /// UUID v4: Universally Unique Identifier version 4
    fn uuid4(&mut self, value: &Option<Value>) {
        if self.uuid4.is_some() && value.is_some() {
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    let re = Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$").unwrap();
                    re.is_match(value)
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} is not valid UUID v4", error.field);
                    error.add(msg);
                }
            }
        }
    }

    /// UUID v5 validator
    /// UUID v5: Universally Unique Identifier version 5
    fn uuid5(&mut self, value: &Option<Value>) {
        if self.uuid5.is_some() && value.is_some() {
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    let re = Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-5[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$").unwrap();
                    re.is_match(value)
                },
                _ => false,
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} is not valid UUID v5", error.field);
                    error.add(msg);
                }
            }
        }
    }


    /// Default value validator
    /// Validate by type and default field
    fn default(&mut self, value: &Option<Value>) -> Option<Value> {
        if let Some(ref default_value) = self.default {
            if value.is_none() {
                return self.to_value(default_value.to_json());
            }
        }
        value.to_owned()
    }

    /// Convert value based ot specific type to Value
    pub fn to_value(&self, json_value: Json) -> Option<Value> {
        match json_value {
            Json::I64(value) => Some(Value::I64(value)),
            Json::U64(value) => Some(Value::U64(value)),
            Json::F64(value) => Some(Value::F64(value)),
            Json::String( ref value) => Some(Value::String(value.clone())),
            Json::Boolean(value) => Some(Value::Boolean(value)),
            Json::Null => Some(Value::Null),
            _ => None,
        }
    }

    /// Comapre vaules as Json types
    /// Return boolean if values compared
    fn compare(&self, value: &Option<Value>, required_value: &Json) -> bool {
        match *value {
            Some(Value::String(ref value)) => {
                value.to_json() == *required_value
            },
            Some(Value::U64(value)) => {
                value.to_json() == *required_value
            },
            Some(Value::I64(value)) => {
                value.to_json() == *required_value
            },
            Some(Value::F64(value)) => {
                value.to_json() == *required_value
            },
            Some(Value::Boolean(value)) => {
                value.to_json() == *required_value
            },
            _ => false
        }
    }

    // Type cast for Value.
    // Returned as Json
    fn type_cast(&self, value: &Option<Value>) -> Option<Json> {
        let val: Value;
        if let Some(name) = value.as_ref() {
            val = name.to_owned();
        } else {
            return None
        }

        match &self.vtype.as_ref() as &str {
            "array" | "vec" => {
                if let Some(val) = <Vec<String> as FromValue>::from_value(&val) {
                    Some(val.to_json())
                } else {
                    None
                }
            },
            "bool" | "boolean" => {
                if let Some(val) = <bool as FromValue>::from_value(&val) {
                    Some(Json::Boolean(val))
                } else {
                    None
                }
            },
            "string" | "str" => {
                if let Some(val) = <String as FromValue>::from_value(&val) {
                    Some(Json::String(val))
                } else {
                    None
                }
            },
            "u8" | "u16" | "u32" | "u64" | "usize" => {
                if let Some(val) = <u64 as FromValue>::from_value(&val) {
                    Some(Json::U64(val))
                } else {
                    None
                }
            },
            "i8" | "i16" | "i32" | "i64" | "isize" => {
                if let Some(val) = <i64 as FromValue>::from_value(&val) {
                    Some(Json::I64(val))
                } else {
                    None
                }
            },
            "f32" | "f64" => {
                if let Some(val) = <f64 as FromValue>::from_value(&val) {
                    Some(Json::F64(val))
                } else {
                    None
                }
            },
            _ => None,
        }
    }
}

/// Validator Errors
#[derive(RustcDecodable, Debug, Clone)]
pub struct ErrorValidator {
    pub errors: Vec<String>,
    pub errors_count: Option<u32>,
    pub field : String,
}

/// Validator Errors methods
impl ErrorValidator {
    /// Init error
    fn new(field: &String) -> Self {
        ErrorValidator {
            field: field.to_owned(),
            errors: vec!(),
            errors_count: None,
        }
    }

    /// Add error
    fn add(&mut self, error: String) {
        if self.errors_count.is_none() {
            self.errors = vec!(error);
            self.errors_count = Some(1);
        } else {
            if let Some(count) = self.errors_count {
                self.errors.push(error);
                self.errors_count = Some(count + 1);
            }
        }
    }
}
