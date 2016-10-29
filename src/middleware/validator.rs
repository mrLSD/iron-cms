use rustc_serialize::json::{self, Json, ToJson};

#[derive(RustcDecodable, Debug)]
pub struct Validator<T> {
    pub requiered: Option<bool>,
    pub empty: Option<bool>,
    pub min: Option<u32>,
    pub max: Option<u32>,
    pub dafault: Option<T>,
    pub field: String,
}

#[derive(RustcDecodable, Debug)]
pub struct ErrorValidator {
    pub errors: Vec<String>
}

impl<T> Validator<T> {
    fn validate(&self, val: Option<&Value>) -> Result<(), ErrorValidator> {
        let mut err = vec!();
        if self.requiered.is_some() {
            if val.is_none() {
                let msg = format!("Field requiered: {}", self.field);
                err.push(String::from(msg));
            }
        }
        if err.len() > 0 {
            Err(ErrorValidator{errors: err})
        } else {
            Ok(())
        }
    }
}
