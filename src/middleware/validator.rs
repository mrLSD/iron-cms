pub use rustc_serialize::json::{self, Json, ToJson};
pub use rustc_serialize::json::DecoderError::*;
pub use rustc_serialize::Decodable;
use params::{Value, FromValue};
use super::render::BaseDataMap;
use std::collections::BTreeMap;
use std::string::String;

/// Base Validator strict
#[derive(RustcDecodable, Debug)]
pub struct Validator<T> {
    pub vtype: String,
    pub requiered: Option<bool>,
    pub empty: Option<bool>,
    pub min: Option<u32>,
    pub max: Option<u32>,
    pub default: Option<T>,
    errors: Option<ErrorValidator>,
}

#[derive(Debug)]
pub struct ValidateResult(BaseDataMap, ErrorValidator);
#[derive(Debug)]
pub struct ValidateResults(pub Vec<ValidateResult>);
pub type ErrorsResult = Option<Vec<ErrorValidator>>;

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

impl<T: FromValue + ToJson + Decodable> Validator<T> {
    pub fn new(validator_rules: BaseDataMap) -> Validator<T> {
        let json_obj: Json = Json::Object(validator_rules);
        let json_str: String = json_obj.to_string();
        match json::decode(&json_str) {
            Ok(decoded) => decoded,
            Err(err) => {
                let msg = match err {
                    ParseError(_) => "JSON parse error",
                    ExpectedError(_, _) => "Validation field expected (not declared)",
                    MissingFieldError(_) => "Validation field missing",
                    _ => "Other error",
                };
                panic!("\
              \n\n |> Validator::new error: {:?}\
                \n |> Validation fields: {:?}\
                \n |> Message: {}\
                \n |> At source code: => ", err, json_obj, msg);
            }
        }
    }

    pub fn validate(&mut self, field: String, value: Option<&Value>) -> ValidateResult {
        let mut value: Option<Value> = if let Some(val) = value {
            Some(val.to_owned())
        } else { None };
        // Init Errors
        self.errors = Some(ErrorValidator::new(&field));

        // Invoke validators
        self.requiered(&value);
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
        if self.requiered.is_some() && value.is_none() {
            if let Some(ref mut error) = self.errors {
                let msg = format!("Field requiered: {}", error.field);
                error.add(msg);
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
    fn to_value(&self, json_value: Json) -> Option<Value> {
        match json_value.to_json() {
            Json::I64(value) => Some(Value::I64(value)),
            Json::U64(value) => Some(Value::U64(value)),
            Json::F64(value) => Some(Value::F64(value)),
            Json::String( ref value) => Some(Value::String(value.clone())),
            Json::Boolean(value) => Some(Value::Boolean(value)),
            Json::Null => Some(Value::Null),
            _ => None,
        }
    }

    fn type_cast(&self, value: &Option<Value>) -> Option<Json> {
        let mut val: Value;
        if let Some(name) = value.as_ref() {
            val = name.to_owned();
        } else {
            return None
        }
        match &self.vtype.as_ref() as &str {
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
        let val_req = Validator::<String>::new(btreemap! {
            "requiered".to_string() => true.to_json(),
            "vtype".to_string() => "bool".to_json(),
        });
        assert_eq!(val_req.requiered, Some(true));

        let val_req = Validator::<bool>::new(btreemap! {
            "default".to_string() => false.to_json(),
            "vtype".to_string() => "bool".to_json(),
        });
        //assert_eq!(val_req.default, Some(false));
    }
}
