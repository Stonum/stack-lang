use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

use serde::Deserialize;

use crate::{
    AnyMCoreDefinition, MCoreEntityDefinition, MCoreEntityMemberDefinition, MCoreFunctionDefinition,
};

pub(crate) fn load() -> KernelData {
    // Чтение файла в строку
    let json_data = include_str!("../coreApi/api.json");

    let parsed_data: KernelData = serde_json::from_str(json_data).unwrap();

    parsed_data
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub(crate) struct DetailedDescription {
    text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub(crate) struct FreeFunction {
    name: String,
    brief_description: String,
    detailed_description: DetailedDescription,
}

impl From<FreeFunction> for AnyMCoreDefinition {
    fn from(val: FreeFunction) -> Self {
        AnyMCoreDefinition::MCoreFunctionDefinition(MCoreFunctionDefinition {
            id: val.name,
            description: format!(
                "```\n{}\n```  \n{}",
                val.brief_description, val.detailed_description.text,
            ),
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub(crate) struct Method {
    name: String,
    brief_description: String,
    detailed_description: DetailedDescription,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub(crate) struct Entity {
    name: String,
    brief_description: String,
    detailed_description: DetailedDescription,
    methods: HashMap<String, Vec<Method>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub(crate) struct KernelData {
    free_functions: Vec<FreeFunction>,
    entities: Vec<Entity>,
}

impl From<KernelData> for Vec<AnyMCoreDefinition> {
    fn from(val: KernelData) -> Self {
        let mut functions: Vec<AnyMCoreDefinition> =
            val.free_functions.into_iter().map(|f| f.into()).collect();

        let mut entities: Vec<AnyMCoreDefinition> = Vec::with_capacity(val.entities.len() * 2);
        for entity in val.entities {
            let entity_def = Arc::new(MCoreEntityDefinition {
                id: entity.name,
                description: format!(
                    "{} {}",
                    entity.brief_description, entity.detailed_description.text,
                ),
            });

            let mut methods = entity
                .methods
                .into_iter()
                .flat_map(|(name, m)| convert_entity_method(name, m, Arc::downgrade(&entity_def)))
                .map(AnyMCoreDefinition::MCoreEntityMemberDefinition)
                .collect::<Vec<_>>();

            entities.push(AnyMCoreDefinition::MCoreEntityDefinition(entity_def));
            entities.append(&mut methods);
        }

        functions.extend(entities);
        functions
    }
}

fn convert_entity_method(
    name: String,
    inner: Vec<Method>,
    class: Weak<MCoreEntityDefinition>,
) -> Vec<MCoreEntityMemberDefinition> {
    if inner.first().is_none() {
        return vec![];
    }

    let descr = format!(
        "```\n{}\n```   \n{}",
        inner[0].brief_description, inner[0].detailed_description.text,
    );

    inner
        .into_iter()
        .map(|m| m.name)
        .chain([name])
        .map(|id| MCoreEntityMemberDefinition {
            id,
            description: descr.clone(),
            class: class.clone(),
        })
        .collect()
}
