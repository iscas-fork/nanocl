use serde::{Serialize, Deserialize};

use crate::cargo_config::CargoConfigPartial;

use super::resource::ResourcePartial;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct StateConfig {
  pub api_version: String,
  pub r#type: String,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct StateResources {
  pub resources: Vec<ResourcePartial>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct StateCargo {
  pub namespace: Option<String>,
  pub cargoes: Vec<CargoConfigPartial>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct StateDeployment {
  pub namespace: Option<String>,
  pub resources: Option<Vec<ResourcePartial>>,
  pub cargoes: Option<Vec<CargoConfigPartial>>,
}