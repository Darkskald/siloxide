use crate::binding::sila_standard as sila;
use std::collections::HashMap;

/// SiLAs basic (primitive) types
#[derive(Clone, Debug)]
pub enum SiLABasic {
    String(sila::String),
    Integer(sila::Integer),
    Real(sila::Real),
    Boolean(sila::Boolean),
    Binary(sila::Binary),
    Date(sila::Date),
    Time(sila::Time),
    Timestamp(sila::Timestamp),
    Any(sila::Any),
    Void,
}


#[derive(Debug, Clone)]
pub struct Constraint {}

#[derive(Debug, Clone)]
pub struct ConstrainedType {
    inner: SiLABasic,
    constraint: Constraint,
}

#[derive(Debug, Clone)]
pub struct StructureElement {
    identifier: String,
    // TODO generate display name according to spec
    description: String,
    // TODO generate a type ID from the variant
    value: SiLAVariant,
}

#[derive(Clone, Debug)]
pub struct StructureType {
    inner: HashMap<String, StructureElement>,
}

#[derive(Debug, Clone)]
pub enum ListElement{
    Basic(SiLABasic),
    Constrained(ConstrainedType),
    Structure(StructureType)
}

/// Generic ListType to enforce single element type in the list
#[derive(Clone, Debug)]
pub struct ListType {
    inner: Vec<ListElement>,  // T can be either SiLABasic or ConstrainedType
}

// TODO runtime check to guarantee only one type of elements is present


/// SiLAs basic (primitive) types
#[derive(Clone, Debug)]
pub enum SiLAVariant {
    Basic(SiLABasic),
    Constrained(ConstrainedType),
    Structure(StructureType)
}

