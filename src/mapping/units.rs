use crate::binding::{SiLAInteger, SiLAString};

// string

impl From<SiLAString> for String{
    fn from(value: SiLAString) -> Self {
        value.value
    }
}

impl From<String> for SiLAString {
    fn from(value: String) -> Self {
       SiLAString{
           value,
       }
    }
}

// integer

impl From<i64> for SiLAInteger {
    fn from(value: i64) -> Self {
       SiLAInteger{
           value
       }
    }
}

impl From<SiLAInteger> for i64 {
    fn from(value: SiLAInteger) -> Self {
        value.value
    }
}