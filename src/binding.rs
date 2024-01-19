pub mod sila {
    tonic::include_proto!("sila_codegen");
}

pub use sila::sila2::org::silastandard as sila_standard;
pub use sila::sila2::org::silastandard::core::silaservice::v1 as sila_service;


// unit re-exports
pub use sila_standard::String as SiLAString;
pub use sila_standard::Integer as SiLAInteger;
pub use sila_standard::Real as SiLAReal;
pub use sila_standard::Binary as SiLABinary;

// errors
pub use sila_standard::si_la_error::Error as ProtoSiLAError;
pub use crate::binding::sila_standard::FrameworkError as ProtoFrameworkError;
pub use crate::binding::sila_standard::ValidationError as ProtoValidationError;
pub use crate::binding::sila_standard::DefinedExecutionError as ProtoDefinedExecutionError;
pub use crate::binding::sila_standard::UndefinedExecutionError as ProtoUndefinedExecutionError;


// greeting provider as example
pub use sila::sila2::org::silastandard::examples::greetingprovider::v1 as sila_greeting_provider;
