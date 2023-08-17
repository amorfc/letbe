use tonic::{
    metadata::{Ascii, MetadataMap, MetadataValue},
    Code, Status,
};

use thiserror::Error;

impl From<LettError> for Status {
    fn from(value: LettError) -> Self {
        value.to_status()
    }
}

#[derive(Error, Debug)]
pub enum LettError {
    #[error("Entity {entity:?} not found")]
    NotFound { entity: String, id: String },
    #[error("Internal error occured: {0}")]
    InternalServerError(String),
    #[error("Unauthorized email please re login: {email:?}. {message:?}")]
    Unauthorized { email: String, message: String },
    #[error("Opss! Somethings went wrong. {0} ")]
    BadRequest(String),
    #[error("{0}")]
    DevInfo(String),
    #[error("Error {0}   ")]
    Other(String),
}

impl LettError {
    pub fn to_status(&self) -> Status {
        let message = self.to_string();
        let code = self.code();
        let metadata = self.metadata_map();

        Status::with_metadata(code, message, metadata)
    }

    pub fn code(&self) -> Code {
        match self {
            Self::NotFound { .. } => Code::NotFound,
            Self::InternalServerError(..) => Code::Internal,
            Self::BadRequest(..) => Code::InvalidArgument,
            Self::Unauthorized { .. } => Code::Unauthenticated,
            Self::DevInfo(..) => Code::Internal,
            Self::Other(..) => Code::Internal,
        }
    }

    pub fn lett_show(&self) -> LettMessageInfo {
        use LettMessageInfo::*;

        match self {
            Self::NotFound { .. } => SHOW,
            Self::BadRequest(..) => SHOW,
            Self::Unauthorized { .. } => SHOW,
            _ => SNEAKY,
        }
    }

    pub fn metadata_map(&self) -> MetadataMap {
        let mut metadata = MetadataMap::with_capacity(2);
        let message_info_str = self.lett_show().to_string().parse().unwrap_or(
            MetadataValue::<Ascii>::try_from(LettMessageInfo::SNEAKY.to_string()).unwrap(),
        );
        metadata.insert("message-info", message_info_str);

        metadata
    }
}

pub enum LettMessageInfo {
    SHOW,
    SNEAKY,
}

impl ToString for LettMessageInfo {
    fn to_string(&self) -> String {
        match self {
            Self::SHOW => "SHOW".to_string(),
            Self::SNEAKY => "SNEAKY".to_string(),
        }
    }
}

impl From<anyhow::Error> for LettError {
    fn from(value: anyhow::Error) -> Self {
        Self::InternalServerError(value.to_string())
    }
}
