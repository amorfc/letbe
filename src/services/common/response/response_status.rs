use tonic::{
    metadata::{Ascii, MetadataMap, MetadataValue},
    Code, Status,
};

use thiserror::Error;

impl From<LettResError> for Status {
    fn from(value: LettResError) -> Self {
        value.to_status()
    }
}

#[derive(Error, Debug)]
pub enum LettResError {
    #[error("Entity {entity:?} not found")]
    NotFound { entity: String, id: String },

    #[error("Internal error occured: {0}")]
    InternalServerError(String),

    #[error("Unauthorized {email:?}. {message:?}")]
    Unauthorized {
        email: Option<String>,
        message: String,
    },

    #[error("Unauthenticated id please re-login or refresh token: {id:?}. {message:?}")]
    Unauthenticated { id: String, message: String },

    #[error("Opss! Somethings went wrong. {0} ")]
    BadRequest(String),

    #[error(transparent)]
    DevInfo(#[from] anyhow::Error),

    #[error("Error {0}")]
    Other(String),
}

impl LettResError {
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
            Self::Unauthorized { .. } => Code::PermissionDenied,
            Self::Unauthenticated { .. } => Code::Unauthenticated,
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
