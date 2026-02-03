mod json;
mod yaml;

use std::sync::{Arc, Weak};

use mlang_lsp_definition::{CodeSymbolDefinition, MarkupDefinition};

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

impl CodeSymbolDefinition for AnyMCoreDefinition {
    fn is_class(&self) -> bool {
        matches!(self, AnyMCoreDefinition::MCoreEntityDefinition(_))
    }

    fn is_function(&self) -> bool {
        matches!(self, AnyMCoreDefinition::MCoreFunctionDefinition(_))
    }

    fn is_method(&self) -> bool {
        matches!(self, AnyMCoreDefinition::MCoreEntityMemberDefinition(_))
    }

    fn container(&self) -> Option<AnyMCoreDefinition> {
        match self {
            AnyMCoreDefinition::MCoreEntityMemberDefinition(method) => method
                .class
                .upgrade()
                .map(AnyMCoreDefinition::MCoreEntityDefinition),
            _ => None,
        }
    }

    fn parent(&self) -> Option<&str> {
        None
    }

    fn id(&self) -> &str {
        match self {
            AnyMCoreDefinition::MCoreFunctionDefinition(f) => &f.id,
            AnyMCoreDefinition::MCoreEntityDefinition(c) => &c.id,
            AnyMCoreDefinition::MCoreEntityMemberDefinition(m) => &m.id,
        }
    }

    fn parameters(&self) -> Option<&str> {
        None
    }
}

impl MarkupDefinition for AnyMCoreDefinition {
    fn markdown(&self) -> String {
        match self {
            AnyMCoreDefinition::MCoreFunctionDefinition(function) => {
                function.description.to_string()
            }
            AnyMCoreDefinition::MCoreEntityDefinition(entity) => entity.description.to_string(),
            AnyMCoreDefinition::MCoreEntityMemberDefinition(member) => {
                member.description.to_string()
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
