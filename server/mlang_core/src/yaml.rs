use serde::Deserialize;

use crate::{AnyMCoreDefinition, MCoreFunctionDefinition};

pub(crate) fn load() -> BuiltInData {
    let yaml_data = include_str!("../coreApi/api.yaml");

    let parsed_data: BuiltInData = serde_yaml::from_str(yaml_data).unwrap();

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

impl From<Function> for Vec<MCoreFunctionDefinition> {
    fn from(val: Function) -> Self {
        val.aliases
            .into_iter()
            .chain([val.name])
            .map(|name| MCoreFunctionDefinition {
                id: name,
                description: val.description.clone(),
            })
            .collect()
    }
}

impl From<BuiltInData> for Vec<AnyMCoreDefinition> {
    fn from(val: BuiltInData) -> Self {
        val.functions
            .into_iter()
            .flat_map(Into::<Vec<MCoreFunctionDefinition>>::into)
            .map(AnyMCoreDefinition::MCoreFunctionDefinition)
            .collect::<Vec<_>>()
    }
}
