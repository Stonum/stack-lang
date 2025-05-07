use crate::formatter::prelude::*;
use biome_formatter::{write, CstFormatContext};

use crate::formatter::rules::lists::parameter_list::FormatMAnyParameterList;
use crate::syntax::{
    AnyMBinding, AnyMConstructorParameter, AnyMExpression, AnyMFormalParameter, AnyMParameter,
    AnyMParameterList, AnyParameter, MConstructorParameters, MParameters, MSyntaxNode,
    MSyntaxToken,
};
use biome_rowan::{declare_node_union, AstNode, SyntaxResult};

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct FormatMParameters();
impl_format_with_rule!(MParameters, FormatMParameters);

impl FormatNodeRule<MParameters> for FormatMParameters {
    fn fmt_fields(&self, node: &MParameters, f: &mut MFormatter) -> FormatResult<()> {
        FormatAnyMParameters::from(node.clone()).fmt(f)
    }

    fn fmt_dangling_comments(&self, _: &MParameters, _: &mut MFormatter) -> FormatResult<()> {
        // Formatted inside of `FormatMAnyParameters`
        Ok(())
    }
}

declare_node_union! {
    pub(crate) FormatAnyMParameters = MParameters | MConstructorParameters
}

impl Format<MFormatContext> for FormatAnyMParameters {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        let list = self.list();

        let parentheses_not_needed = false;

        let can_hug =
            should_hug_function_parameters(self, f.context().comments(), parentheses_not_needed)?;

        let layout = if list.is_empty() {
            ParameterLayout::NoParameters
        } else if can_hug {
            ParameterLayout::Hug
        } else {
            ParameterLayout::Default
        };

        let l_paren_token = self.l_paren_token()?;
        let r_paren_token = self.r_paren_token()?;

        match layout {
            ParameterLayout::NoParameters => {
                write!(
                    f,
                    [
                        l_paren_token.format(),
                        format_dangling_comments(self.syntax()).with_soft_block_indent(),
                        r_paren_token.format()
                    ]
                )
            }
            ParameterLayout::Hug => {
                if !parentheses_not_needed {
                    write!(f, [l_paren_token.format(), space()])?;
                } else {
                    write!(f, [format_removed(&l_paren_token)])?;
                }

                write!(
                    f,
                    [FormatMAnyParameterList::with_layout(
                        &list,
                        ParameterLayout::Hug
                    )]
                )?;

                if !parentheses_not_needed {
                    write!(f, [space(), &r_paren_token.format()])?;
                } else {
                    write!(f, [format_removed(&r_paren_token)])?;
                }

                Ok(())
            }
            ParameterLayout::Default => {
                if !parentheses_not_needed {
                    write!(f, [l_paren_token.format(), space()])?;
                } else {
                    write!(f, [format_removed(&l_paren_token)])?;
                }

                write!(
                    f,
                    [soft_block_indent(&FormatMAnyParameterList::with_layout(
                        &list,
                        ParameterLayout::Default,
                    ))]
                )?;

                if !parentheses_not_needed {
                    write!(f, [space(), r_paren_token.format()])?;
                } else {
                    write!(f, [format_removed(&r_paren_token)])?;
                }

                Ok(())
            }
        }
    }
}

impl FormatAnyMParameters {
    fn l_paren_token(&self) -> SyntaxResult<MSyntaxToken> {
        match self {
            FormatAnyMParameters::MParameters(parameters) => parameters.l_paren_token(),
            FormatAnyMParameters::MConstructorParameters(parameters) => parameters.l_paren_token(),
        }
    }

    fn list(&self) -> AnyMParameterList {
        match self {
            FormatAnyMParameters::MParameters(parameters) => {
                AnyMParameterList::from(parameters.items())
            }
            FormatAnyMParameters::MConstructorParameters(parameters) => {
                AnyMParameterList::from(parameters.parameters())
            }
        }
    }

    fn r_paren_token(&self) -> SyntaxResult<MSyntaxToken> {
        match self {
            FormatAnyMParameters::MParameters(parameters) => parameters.r_paren_token(),
            FormatAnyMParameters::MConstructorParameters(parameters) => parameters.r_paren_token(),
        }
    }

    fn syntax(&self) -> &MSyntaxNode {
        match self {
            FormatAnyMParameters::MParameters(parameters) => parameters.syntax(),
            FormatAnyMParameters::MConstructorParameters(parameters) => parameters.syntax(),
        }
    }
}

#[derive(Copy, Debug, Clone, Eq, PartialEq)]
pub enum ParameterLayout {
    /// ```javascript
    /// function test() {}
    /// ```
    NoParameters,

    /// Enforce that the opening and closing parentheses aren't separated from the first token of the parameter.
    /// For example, to enforce that the `{`  and `}` of an object expression are formatted on the same line
    /// as the `(` and `)` tokens even IF the object expression itself breaks across multiple lines.
    ///
    /// ```javascript
    /// function test({
    ///     aVeryLongObjectBinding,
    ///     thatContinuesAndExceeds,
    ///     theLineWidth
    /// }) {}
    /// ```
    Hug,

    /// The default layout formats all parameters on the same line if they fit or breaks after the `(`
    /// and before the `)`.
    ///
    /// ```javascript
    /// function test(
    ///     firstParameter,
    ///     secondParameter,
    ///     thirdParameter
    /// ) {}
    /// ```
    Default,
}

pub(crate) fn should_hug_function_parameters(
    parameters: &FormatAnyMParameters,
    comments: &MComments,
    parentheses_not_needed: bool,
) -> FormatResult<bool> {
    /// Returns true if the first parameter should be forced onto the same line as the `(` and `)` parentheses.
    /// See the `[ParameterLayout::Hug] documentation.
    ///
    /// parameter `should_hug_formal_parameter` is a bool value used to determine whether the parenthesized arrow function parameters
    /// should be on the same line as the arrow function after removing the parentheses.
    fn hug_formal_parameter(
        parameter: &self::AnyMFormalParameter,
        should_hug_formal_parameter: bool,
    ) -> FormatResult<bool> {
        let result = match parameter {
            AnyMFormalParameter::MFormalParameter(parameter) => match parameter.initializer() {
                None => match parameter.binding()? {
                    AnyMBinding::MIdentifierBinding(_) => should_hug_formal_parameter,
                    AnyMBinding::MBogusBinding(_) => {
                        return Err(FormatError::SyntaxError);
                    }
                },

                Some(initializer) => {
                    

                    match initializer.expression()? {
                        AnyMExpression::MObjectExpression(object) => object.members().is_empty(),
                        AnyMExpression::MArrayExpression(array) => array.elements().is_empty(),
                        AnyMExpression::MIdentifierExpression(_) => true,
                        _ => false,
                    }
                }
            },
            AnyMFormalParameter::MBogusParameter(_) => return Err(FormatError::SyntaxError),
        };

        Ok(result)
    }

    let list = parameters.list();
    if list.len() != 1 {
        return Ok(false);
    }

    // SAFETY: Safe because of the length check above
    let only_parameter = list.first().unwrap()?;

    if comments.has_comments(only_parameter.syntax()) {
        return Ok(false);
    }

    let has_parentheses = parameters.l_paren_token().is_ok() && parameters.r_paren_token().is_ok();
    let should_hug_formal_parameter = has_parentheses && parentheses_not_needed;

    let result = match only_parameter {
        AnyParameter::AnyMParameter(parameter) => match parameter {
            AnyMParameter::AnyMFormalParameter(formal_parameter) => {
                hug_formal_parameter(&formal_parameter, should_hug_formal_parameter)?
            }
            AnyMParameter::MRestParameter(_) => false,
        },
        AnyParameter::AnyMConstructorParameter(constructor_parameter) => {
            match constructor_parameter {
                AnyMConstructorParameter::AnyMFormalParameter(formal_parameter) => {
                    hug_formal_parameter(&formal_parameter, should_hug_formal_parameter)?
                }
                AnyMConstructorParameter::MRestParameter(_) => false,
            }
        }
    };

    Ok(result)
}

/// Tests if all of the parameters of `expression` are simple enough to allow
/// a function to group.
pub(crate) fn has_only_simple_parameters(parameters: &MParameters) -> bool {
    parameters
        .items()
        .into_iter()
        .flatten()
        .all(|parameter| is_simple_parameter(&parameter))
}

/// Tests if the single parameter is "simple", as in a plain identifier with no
/// explicit type annotation and no initializer:
///
/// Examples:
/// foo             => true
/// foo?            => true
/// foo = 'bar'     => false
/// foo: string     => false
/// {a, b}          => false
/// {a, b} = {}     => false
/// [a, b]          => false
///
pub(crate) fn is_simple_parameter(parameter: &AnyMParameter) -> bool {
    match parameter {
        AnyMParameter::AnyMFormalParameter(AnyMFormalParameter::MFormalParameter(param)) => {
            matches!(param.binding(), Ok(AnyMBinding::MIdentifierBinding(_)))
                && param.initializer().is_none()
        }
        _ => false,
    }
}
