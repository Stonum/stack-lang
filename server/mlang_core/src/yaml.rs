use serde::Deserialize;
use serde_yaml;

use crate::{AnyMCoreDefinition, MCoreFunctionDefinition};

pub(crate) fn load() -> BuiltInData {
    let yaml_data = include_str!("../coreApi/api.yaml");

    let parsed_data: BuiltInData = serde_yaml::from_str(&yaml_data).unwrap();

    parsed_data
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub(crate) struct Function {
    name: String,
    description: String,
    aliases: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub(crate) struct BuiltInData {
    functions: Vec<Function>,
}

impl Into<Vec<MCoreFunctionDefinition>> for Function {
    fn into(self) -> Vec<MCoreFunctionDefinition> {
        self.aliases
            .into_iter()
            .chain([self.name].into_iter())
            .map(|name| MCoreFunctionDefinition {
                id: name,
                description: self.description.clone(),
            })
            .collect()
    }
}

impl Into<Vec<AnyMCoreDefinition>> for BuiltInData {
    fn into(self) -> Vec<AnyMCoreDefinition> {
        self.functions
            .into_iter()
            .flat_map(|f| Into::<Vec<MCoreFunctionDefinition>>::into(f))
            .map(AnyMCoreDefinition::MCoreFunctionDefinition)
            .collect::<Vec<_>>()
    }
}
