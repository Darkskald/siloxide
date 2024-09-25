use crate::sila_domain::common::{Category, Identifier, MaturityLevel, Version};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureAttributes {
    // only major and minor allowed
    sila2_version: Version,
    feature_version: Version,
    maturity_level: MaturityLevel,
    // has also constraints, maybe wrap into own type?
    originator: String,
    category: Category,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    attributes: FeatureAttributes,
    identifier: Identifier,
}