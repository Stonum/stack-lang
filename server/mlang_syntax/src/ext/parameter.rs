use crate::{
    AnyMBinding, AnyMConstructorParameter, AnyMParameter, MConstructorParameterList, MLanguage,
    MParameterList,
};
use biome_rowan::{
    AstNode, AstSeparatedList, AstSeparatedListNodesIterator, SyntaxResult, declare_node_union,
};

/// An enumeration representing different types of parameter lists.
///
/// This enum can represent a regular parameter list (i.e., for functions)
/// or a constructor parameter list (i.e., for class constructors).
///
/// # Variants
///
/// * `MParameterList` - A list of parameters for a function.
/// * `MConstructorParameterList` - A list of parameters for a constructor.
#[derive(Debug)]
pub enum AnyMParameterList {
    MParameterList(MParameterList),
    MConstructorParameterList(MConstructorParameterList),
}

impl From<MParameterList> for AnyMParameterList {
    fn from(list: MParameterList) -> Self {
        AnyMParameterList::MParameterList(list)
    }
}

impl From<MConstructorParameterList> for AnyMParameterList {
    fn from(list: MConstructorParameterList) -> Self {
        AnyMParameterList::MConstructorParameterList(list)
    }
}

impl AnyMParameterList {
    /// This method allows to get the length of a parameter list, regardless
    /// of whether it's a standard parameter list or a constructor parameter list.
    ///
    /// # Returns
    ///
    /// Returns the length of the parameter list.
    pub fn len(&self) -> usize {
        match self {
            AnyMParameterList::MParameterList(parameters) => parameters.len(),
            AnyMParameterList::MConstructorParameterList(parameters) => parameters.len(),
        }
    }

    /// This method checks if a parameter list is empty.
    ///
    /// # Returns
    ///
    /// Returns `true` if the parameter list contains no parameters, false otherwise.
    pub fn is_empty(&self) -> bool {
        match self {
            AnyMParameterList::MParameterList(parameters) => parameters.is_empty(),
            AnyMParameterList::MConstructorParameterList(parameters) => parameters.is_empty(),
        }
    }

    /// This method allows to get the first parameter in the parameter list.
    ///
    /// # Returns
    ///
    /// Returns the first parameter in the parameter list if it exists.
    pub fn first(&self) -> Option<SyntaxResult<AnyParameter>> {
        Some(match self {
            AnyMParameterList::MParameterList(parameters) => {
                parameters.first()?.map(|parameter| parameter.into())
            }
            AnyMParameterList::MConstructorParameterList(parameters) => {
                parameters.first()?.map(|parameter| parameter.into())
            }
        })
    }

    /// This method allows you to iterate over the parameters in a `MParameterList` or a `MConstructorParameterList`,
    /// depending on the variant of the `AnyMParameterList` enum.
    ///
    /// # Returns
    ///
    /// Returns an iterator over the parameters in the list.
    ///
    pub fn iter(&self) -> AnyMParameterListNodeIter {
        match self {
            AnyMParameterList::MParameterList(list) => {
                AnyMParameterListNodeIter::MParameterList(list.iter())
            }
            AnyMParameterList::MConstructorParameterList(list) => {
                AnyMParameterListNodeIter::MConstructorParameterList(list.iter())
            }
        }
    }

    /// This method allows to get the last parameter in the parameter list.
    ///
    /// # Returns
    ///
    /// Returns the last parameter in the parameter list if it exists.
    ///
    pub fn last(&self) -> Option<SyntaxResult<AnyParameter>> {
        Some(match self {
            AnyMParameterList::MParameterList(parameters) => {
                parameters.last()?.map(|parameter| parameter.into())
            }
            AnyMParameterList::MConstructorParameterList(parameters) => {
                parameters.last()?.map(|parameter| parameter.into())
            }
        })
    }
}

impl std::fmt::Display for AnyMParameterList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return write!(f, "()");
        }

        match self {
            AnyMParameterList::MParameterList(parameters) => {
                write!(f, "( {} )", parameters.syntax().to_string().trim())
            }
            AnyMParameterList::MConstructorParameterList(parameters) => {
                write!(f, "( {} )", parameters.syntax().to_string().trim())
            }
        }
    }
}

/// An iterator over the parameters in an `AnyMParameterList`.
///
/// This iterator can traverse a regular parameter list (i.e., for functions)
/// or a constructor parameter list (i.e., for class constructors), depending
/// on the variant of the `AnyMParameterListNodeIter` enum.
pub enum AnyMParameterListNodeIter {
    MParameterList(AstSeparatedListNodesIterator<MLanguage, AnyMParameter>),
    MConstructorParameterList(AstSeparatedListNodesIterator<MLanguage, AnyMConstructorParameter>),
}

impl Iterator for AnyMParameterListNodeIter {
    type Item = SyntaxResult<AnyParameter>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self {
            AnyMParameterListNodeIter::MParameterList(inner) => {
                inner.next()?.map(AnyParameter::from)
            }
            AnyMParameterListNodeIter::MConstructorParameterList(inner) => {
                inner.next()?.map(AnyParameter::from)
            }
        })
    }
}

declare_node_union! {
    /// The `AnyParameter` union can represent either a standard parameter
    /// or a constructor parameter. This is useful in contexts where a
    /// function could accept either type of parameter.
    pub AnyParameter = AnyMConstructorParameter | AnyMParameter
}

impl AnyParameter {
    pub fn binding(&self) -> Option<AnyMBinding> {
        match self {
            AnyParameter::AnyMConstructorParameter(parameter) => match parameter {
                AnyMConstructorParameter::AnyMFormalParameter(parameter) => {
                    parameter.as_m_formal_parameter()?.binding().ok()
                }
                AnyMConstructorParameter::MRestParameter(parameter) => parameter.binding(),
            },
            AnyParameter::AnyMParameter(parameter) => match parameter {
                AnyMParameter::AnyMFormalParameter(parameter) => {
                    parameter.as_m_formal_parameter()?.binding().ok()
                }
                AnyMParameter::MRestParameter(parameter) => parameter.binding(),
            },
        }
    }
    pub fn is_rest(&self) -> bool {
        match self {
            AnyParameter::AnyMConstructorParameter(parameter) => match parameter {
                AnyMConstructorParameter::AnyMFormalParameter(_) => false,
                AnyMConstructorParameter::MRestParameter(_) => true,
            },
            AnyParameter::AnyMParameter(parameter) => match parameter {
                AnyMParameter::AnyMFormalParameter(_) => false,
                AnyMParameter::MRestParameter(_) => true,
            },
        }
    }
}
