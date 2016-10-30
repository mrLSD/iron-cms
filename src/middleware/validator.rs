use rustc_serialize::json::{self, Json, ToJson};
use params::Value;

#[derive(RustcDecodable, Debug)]
pub struct Validator<T> {
    pub requiered: Option<bool>,
    pub empty: Option<bool>,
    pub min: Option<u32>,
    pub max: Option<u32>,
    pub dafault: Option<T>,
    pub field: String,
    errors: ErrorValidator,
}

impl<T> Validator<T> {
    pub fn validate(&mut self, value: Option<&Value>) -> Result<(), ErrorValidator> {
        // Init Errors
        self.errors = ErrorValidator::new(&self.field);
        self.requiered(value);

        if self.errors.errors_count.is_some() {
            Err(self.errors.to_owned())
        } else {
            Ok(())
        }
    }

    fn requiered(&mut self, value: Option<&Value>) {
        if self.requiered.is_some() {
            if value.is_none() {
                let msg = format!("Field requiered: {}", self.field);
                self.errors.add(msg);
            }
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
