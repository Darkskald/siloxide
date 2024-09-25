/*use prost::Message;
use tonic::{Code, Status};
use crate::binding::{ProtoDefinedExecutionError, ProtoFrameworkError, ProtoSiLAError, ProtoUndefinedExecutionError, ProtoValidationError};
use crate::binding::sila_standard::si_la_error::Error;
use crate::sila_domain::error::{FrameworkErrorType, SiLAError};
use base64::prelude::*;

impl From<ProtoSiLAError> for SiLAError {
    fn from(value: ProtoSiLAError) -> Self {
        match value {
            Error::ValidationError(inner) => {
                SiLAError::ValidationError {
                    parameter_fqi: inner.parameter,
                    content: inner.message,
                }
            }
            Error::DefinedExecutionError(inner) => {
                SiLAError::DefinedExecutionError {
                    error_fiq: inner.error_identifier,
                    content: inner.message,
                }
            }
            Error::UndefinedExecutionError(inner) => {
                SiLAError::UndefinedExecutionError(inner.message)
            }
            Error::FrameworkError(inner) => {
                let message = inner.message.to_string();
                SiLAError::FrameworkError {
                    error_type: inner.into(),
                    content: message,
                }
            }
        }
    }
}

impl From<ProtoFrameworkError> for FrameworkErrorType {
    fn from(value: ProtoFrameworkError) -> Self {
        match value.error_type {
            0 => FrameworkErrorType::CommandExecutionNotAccepted,
            1 => FrameworkErrorType::InvalidCommandExecutionUUID,
            2 => FrameworkErrorType::InvalidMetadata,
            3 => FrameworkErrorType::NoMetadataAllowed,
            _ => panic!("invalid error type")
        }
    }
}

impl From<SiLAError> for ProtoSiLAError {
    fn from(value: SiLAError) -> Self {
        match value {
            SiLAError::ValidationError { parameter_fqi, content } => Error::ValidationError(ProtoValidationError {
                parameter: parameter_fqi,
                message: content,
            }),
            SiLAError::DefinedExecutionError { error_fiq, content } => Error::DefinedExecutionError(
                ProtoDefinedExecutionError {
                    error_identifier: error_fiq,
                    message: content,
                }
            ),
            SiLAError::UndefinedExecutionError(inner) => Error::UndefinedExecutionError(ProtoUndefinedExecutionError {
                message: inner,
            }),

            SiLAError::FrameworkError { error_type, content } => Error::FrameworkError(
                ProtoFrameworkError {
                    error_type: error_type.into(),
                    message: content,
                }
            )
        }
    }
}

impl From<FrameworkErrorType> for i32 {
    fn from(value: FrameworkErrorType) -> Self {
        match value {
            FrameworkErrorType::CommandExecutionNotAccepted => 0,
            FrameworkErrorType::InvalidCommandExecutionUUID => 1,
            FrameworkErrorType::CommandExecutionNotFinished => 2,
            FrameworkErrorType::InvalidMetadata => 3,
            FrameworkErrorType::NoMetadataAllowed => 4,
            _ => panic!("invalid error type")
        }
    }
}


// convert to gRPC status

impl From<ProtoSiLAError> for Status{
    fn from(value: Error) -> Self {
        let mut buf = Vec::new();
        let t = value.encode(&mut buf);
        // the SiLA standard demands serializing the error Protobuf to a Base64 String as the message
        // field of the gRPC status while the status code must be "Aborted"
        let b64 =  BASE64_STANDARD.encode(buf);
        log::debug!("error string: {}", b64);
        Status::new(
            Code::Aborted,
            b64
        )
    }
}

impl From<SiLAError> for Status {
    fn from(value: SiLAError) -> Self {
        let temp: ProtoSiLAError = value.into();
        temp.into()
    }
}*/