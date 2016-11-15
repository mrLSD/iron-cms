//! # What is Validator
//! It useful for Form and Model validation.
//! Implementation for Validation via BTreeMap structure.
//! It consist basic validations rule, validatior, and
//! validation Result and validation Errors.
//!
//! ## How to use
//! Most common examples you can find at src/admin/models/*
pub use rustc_serialize::json::{self, Json, ToJson};
pub use rustc_serialize::json::DecoderError::*;
pub use rustc_serialize::Decodable;
use params::{Value, FromValue};
use super::render::{BaseDataMap, BaseDataMapDecoder};
use std::collections::BTreeMap;
use std::string::String;

/// Base Validator struct
#[derive(RustcDecodable, Debug)]
pub struct Validator<T> {
    /// Type of validator
    /// For example: string, bool, i64 etc.
    pub vtype: String,
    pub requiered: Option<bool>,
    pub not_empty: Option<bool>,
    pub min: Option<i64>,
    pub max: Option<u64>,
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
impl<T: FromValue + ToJson + Decodable> Validator<T> {
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
        self.requiered(&value);
        self.not_empty(&value);
        self.max(&value);
        self.min(&value);
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
    fn requiered(&mut self, value: &Option<Value>) {
        if self.requiered.is_some() {
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
                let msg = format!("Field requiered: {}", error.field);
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
            let mut requiered_value: u64 = 0;
            if let Some(max) = self.max {
                if max == 0 {
                    if let Some(ref mut error) = self.errors {
                        let msg = format!("Validation value can't be equal: {}", max);
                        error.add(msg);
                    }
                    return ()
                }
                requiered_value = max;
            }
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    value.chars().count() as u64 <= requiered_value
                },
                Some(Value::U64(value)) => {
                    value <= requiered_value
                },
                Some(Value::I64(value)) => {
                    value as u64 <= requiered_value
                },
                Some(Value::F64(value)) => {
                    value as u64 <= requiered_value
                },
                Some(Value::Boolean(value)) => {
                    value as u64 <= requiered_value
                },
                _ => false
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} can't be min then: {}", error.field, requiered_value);
                    error.add(msg);
                }
            }
        }
    }

    /// Min value validator
    /// Multitype
    fn min(&mut self, value: &Option<Value>) {
        if self.min.is_some() && value.is_some() {
            let mut requiered_value: i64 = 0;
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
                requiered_value = min;
            }
            let is_valid = match *value {
                Some(Value::String(ref value)) => {
                    value.chars().count() as i64 >= requiered_value
                },
                Some(Value::U64(value)) => {
                    value as i64>= requiered_value
                },
                Some(Value::I64(value)) => {
                    value >= requiered_value
                },
                Some(Value::F64(value)) => {
                    value as i64 >= requiered_value
                },
                Some(Value::Boolean(value)) => {
                    value as i64 >= requiered_value
                },
                _ => false
            };
            if !is_valid {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field {} can't be min then: {}", error.field, requiered_value);
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
                    return None;
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
                    return None;
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
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
}
