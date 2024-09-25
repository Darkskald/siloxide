use std::fmt::{write, Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::sila_domain::error::SiloxideError;

#[derive(Debug, Clone,PartialEq, Serialize, Deserialize)]
pub struct Identifier(String);

impl Identifier {
    pub fn from_str<S: AsRef<str>>(input: S) -> Result<Self, SiloxideError> {
        let input = input.as_ref();

        // Check the length constraint
        if input.len() > 255 {
            return Err(SiloxideError::InvalidIdentifier(
                "Identifier exceeds maximum length of 255 characters".into(),
            ));
        }

        // Check the first character is an uppercase letter
        let mut chars = input.chars();
        if let Some(first_char) = chars.next() {
            if !first_char.is_ascii_uppercase() {
                return Err(SiloxideError::InvalidIdentifier(
                    "Identifier must start with an uppercase letter".into(),
                ));
            }
        } else {
            return Err(SiloxideError::InvalidIdentifier(
                "Identifier cannot be empty".into(),
            ));
        }

        // Check the rest of the characters are lowercase or digits
        if !chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()) {
            return Err(SiloxideError::InvalidIdentifier(
                "Identifier contains invalid characters".into(),
            ));
        }

        Ok(Identifier(input.to_string()))
    }
}

#[derive(Debug, Clone,PartialEq, Serialize, Deserialize)]
pub struct Version {
    major: u32,
    minor: u32,
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

#[derive(Debug, Clone,PartialEq, Serialize, Deserialize)]
pub struct VersionWithPatch {
    major: u32,
    minor: u32,
    patch: Option<u32>,
}

impl Display for VersionWithPatch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let patch_string = self.patch
            .map(|x| format!(".{}", x))
            .unwrap_or("".to_string());
        write!(f, "{}.{}{}", self.major, self.minor, patch_string)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaturityLevel {
    Draft,
    Verified,
    Normative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Category {
    None,
    Core,
    Custom(String),
}