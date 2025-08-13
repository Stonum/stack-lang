mod json;
mod yaml;

use std::sync::{Arc, Weak};

pub fn load_core_api() -> Vec<AnyMCoreDefinition> {
    let mut json_entities: Vec<AnyMCoreDefinition> = json::load().into();
    let yaml_functions: Vec<AnyMCoreDefinition> = yaml::load().into();

    json_entities.extend(yaml_functions);
    json_entities
}

#[derive(Debug, Eq, PartialEq)]
pub enum AnyMCoreDefinition {
    MCoreFunctionDefinition(MCoreFunctionDefinition),
    MCoreEntityDefinition(Arc<MCoreEntityDefinition>),
    MCoreEntityMemberDefinition(MCoreEntityMemberDefinition),
}

impl AnyMCoreDefinition {
    pub fn is_class(&self) -> bool {
        match self {
            AnyMCoreDefinition::MCoreEntityDefinition(_) => true,
            _ => false,
        }
    }

    pub fn is_function(&self) -> bool {
        match self {
            AnyMCoreDefinition::MCoreFunctionDefinition(_) => true,
            _ => false,
        }
    }

    pub fn is_constructor(&self) -> bool {
        false
    }

    pub fn is_method(&self) -> bool {
        match self {
            AnyMCoreDefinition::MCoreEntityMemberDefinition(_) => true,
            _ => false,
        }
    }

    pub fn container(&self) -> Option<AnyMCoreDefinition> {
        match self {
            AnyMCoreDefinition::MCoreEntityMemberDefinition(method) => method
                .class
                .upgrade()
                .map(AnyMCoreDefinition::MCoreEntityDefinition),
            _ => None,
        }
    }

    pub fn id(&self) -> &str {
        match self {
            AnyMCoreDefinition::MCoreFunctionDefinition(function) => function.id.as_str(),
            AnyMCoreDefinition::MCoreEntityDefinition(entity) => entity.id.as_str(),
            AnyMCoreDefinition::MCoreEntityMemberDefinition(member) => member.id.as_str(),
        }
    }

    pub fn to_markdown(&self) -> String {
        match self {
            AnyMCoreDefinition::MCoreFunctionDefinition(function) => {
                format!("```{}```  \n{}", function.id, function.description)
            }
            AnyMCoreDefinition::MCoreEntityDefinition(entity) => {
                format!("```{}```  \n{}", entity.id, entity.description)
            }
            AnyMCoreDefinition::MCoreEntityMemberDefinition(member) => {
                format!("```{}```  \n{}", member.id, member.description)
            }
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct MCoreFunctionDefinition {
    id: String,
    description: String,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct MCoreEntityDefinition {
    id: String,
    description: String,
}

#[derive(Debug, Default)]
pub struct MCoreEntityMemberDefinition {
    id: String,
    class: Weak<MCoreEntityDefinition>,
    description: String,
}

impl PartialEq for MCoreEntityMemberDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.description == other.description
    }
}
impl Eq for MCoreEntityMemberDefinition {}
