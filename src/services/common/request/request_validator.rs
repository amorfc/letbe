use tonic::Status;
use validator::{Validate, ValidationErrors, ValidationErrorsKind};

pub struct RequestValidator<'a, T: Validate> {
    inner: &'a T,
}

impl<'a, T: Validate> RequestValidator<'a, T> {
    pub fn new(inner: &'a T) -> Self {
        Self { inner }
    }

    fn error_messages(&self) -> Option<Vec<String>> {
        if let Err(errors) = self.inner.validate() {
            let messages = <dyn Validate>::as_lett_error(&Box::new(errors));
            return Some(messages);
        };

        None
    }

    pub fn validate_for_response(&self) -> Result<(), tonic::Status> {
        if let Some(messages) = self.error_messages() {
            //Can be send as a different error type
            return Err(Status::invalid_argument(messages.join(".")));
        };

        Ok(())
    }
}

trait ValidateExt {
    fn as_lett_error(v: &ValidationErrors) -> Vec<String>;
}

impl ValidateExt for dyn Validate {
    fn as_lett_error(v: &ValidationErrors) -> Vec<String> {
        let mut error_messages: Vec<String> = vec![];

        v.errors().iter().for_each(|e| {
            match e.1 {
                ValidationErrorsKind::Struct(s) => {
                    let messages = <dyn Validate>::as_lett_error(s);
                    error_messages.extend(messages);
                }
                ValidationErrorsKind::List(l) => {
                    l.iter().for_each(|e| {
                        let messages = <dyn Validate>::as_lett_error(e.1);
                        error_messages.extend(messages);
                    });
                }
                ValidationErrorsKind::Field(f) => {
                    f.iter().for_each(|e| {
                        error_messages.push(
                            e.message
                                .clone()
                                .map_or_else(|| String::from(""), |s| s.to_string()),
                        );
                    });
                }
            };
        });

        error_messages
    }
}
