use thiserror::Error;

#[derive(Debug)]
pub struct ErrorContent {
    reason: String,
    suggestion: Option<String>
}

#[derive(Debug, Error)]
pub enum SiLAError{
    #[error("validation error")]
    ValidationError{
        parameter_fqi: String,
        content: String
    },
     #[error("defined execution error")]
    DefinedExecutionError{
        error_fiq: String,
        content: String
    },
    #[error("undefined execution error")]
    UndefinedExecutionError(String),
    #[error("framework error")]
    FrameworkError{
        error_type: FrameworkErrorType,
        content: String,
    },
}

#[derive(Debug, Error)]
pub enum FrameworkErrorType {
    #[error("inner framework error")]
    CommandExecutionNotAccepted,
    #[error("inner framework error")]
    InvalidCommandExecutionUUID,
    #[error("inner framework error")]
    CommandExecutionNotFinished,
    #[error("inner framework error")]
    InvalidMetadata,
    #[error("inner framework error")]
    NoMetadataAllowed
}
