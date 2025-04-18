use super::is_call_like_expression;
use crate::syntax::{
    AnyMArrayElement, AnyMAssignment, AnyMCallArgument, AnyMExpression, AnyMLiteralExpression,
    AnyMObjectMember, AnyMObjectMemberName, MComputedMemberExpressionFields, MName,
    MPostUpdateOperator, MPreUpdateOperator, MSpread, MStaticMemberExpressionFields,
    MUnaryOperator,
};
use biome_rowan::{AstSeparatedList, SyntaxResult};

/// This enum tracks the arguments inside a call expressions and checks if they are
/// simple or not.
///
/// The heuristic changes based on its type and the depth of the expressions. For example
/// if we have expressions as arguments, having 2 or more them tags the first argument as "not simple".
///
/// Criteria are different:
/// - *complex*: if the chain of simple arguments exceeds the depth 2 or higher
/// - *complex*: if the argument is a [MRegexLiteralExpression] with len() greater than 5
/// - *simple*: the argument is a literal
/// - *simple*: the argument is a [MRegexLiteralExpression] with len() less than 5
/// - *simple*: the argument is a [MThisExpression]
/// - *simple*: the argument is a [MIdentifierExpression]
/// - *simple*: the argument is a [MSuperExpression]
/// - *simple*: the argument is a [MUnaryExpression], has a trivial operator (`!`, `-`, `~`, or `+`), and the argument is simple
/// - *simple*: the argument is a [MPreUpdateExpression] or [MPostUpdateExpression], with a trivial operator (`++` or `--`), and the argument is simple.
/// - *simple*: the argument is a [TsNonNullAssertionExpression] and the argument is simple
/// - if the argument is a template literal, check [is_simple_template_literal]
/// - if the argument is an object expression, all its members are checked if they are simple or not. Check [is_simple_object_expression]
/// - if the argument is an array expression, all its elements are checked if they are simple or not. Check [is_simple_array_expression]
///
/// This algorithm is inspired from [Prettier].
///
/// [MThisExpression]: [crate::syntax::MThisExpression]
/// [MIdentifierExpression]: [crate::syntax::MIdentifierExpression]
/// [MSuperExpression]: [crate::syntax::MSuperExpression]
/// [is_simple_object_expression]: [Simple::is_simple_object_expression]
/// [is_simple_array_expression]: [Simple::is_simple_array_expression]
/// [MUnaryExpression]: [crate::syntax::MUnaryExpression]
/// [MPreUpdateExpression]: [crate::syntax::MPreUpdateExpression]
/// [MPostUpdateExpression]: [crate::syntax::MPostUpdateExpression]
/// [TsNonNullAssertionExpression]: [crate::syntax::TsNonNullAssertionExpression]
/// [Prettier]: https://github.com/prettier/prettier/blob/a9de2a128cc8eea84ddd90efdc210378a894ab6b/src/language-M/utils/index.M#L802-L886
#[derive(Debug)]
pub enum SimpleArgument {
    Expression(AnyMExpression),
    Assignment(AnyMAssignment),
    Name(MName),
    Spread,
}

impl SimpleArgument {
    pub fn new(node: AnyMCallArgument) -> Self {
        match node {
            AnyMCallArgument::AnyMExpression(expr) => Self::Expression(expr),
            AnyMCallArgument::MSpread(_) => Self::Spread,
        }
    }

    pub fn is_simple(&self) -> bool {
        self.is_simple_impl(0)
    }

    fn is_simple_impl(&self, depth: u8) -> bool {
        if depth >= 2 {
            return false;
        }

        if self.is_simple_literal() {
            return true;
        }

        self.is_simple_object_expression(depth)
            || self.is_simple_array_expression(depth)
            || self.is_simple_unary_expression(depth).unwrap_or(false)
            || self.is_simple_update_expression(depth).unwrap_or(false)
            || self.is_simple_member_expression(depth).unwrap_or(false)
            || self.is_simple_call_like_expression(depth).unwrap_or(false)
            || self.is_simple_object_expression(depth)
    }

    fn is_simple_call_like_expression(&self, depth: u8) -> SyntaxResult<bool> {
        let result = if let SimpleArgument::Expression(any_expression) = self {
            if is_call_like_expression(any_expression) {
                let mut is_simple_callee = false;
                let arguments = match any_expression {
                    AnyMExpression::MNewExpression(expr) => {
                        let callee = expr.callee()?;
                        is_simple_callee = SimpleArgument::from(callee).is_simple_impl(depth);
                        expr.arguments()
                    }
                    AnyMExpression::MCallExpression(expr) => {
                        let callee = expr.callee()?;
                        is_simple_callee = SimpleArgument::from(callee).is_simple_impl(depth);
                        expr.arguments().ok()
                    }
                    _ => unreachable!("The check is done inside `is_call_like_expression`"),
                };

                if !is_simple_callee {
                    return Ok(false);
                }

                if let Some(arguments) = arguments {
                    // This is a little awkward, but because we _increment_
                    // depth, we need to add it to the left and compare to the
                    // max we allow (2), versus just comparing `len <= depth`.
                    arguments.args().len() + usize::from(depth) <= 2
                        && arguments.args().iter().all(|argument| {
                            argument.map_or(true, |argument| {
                                SimpleArgument::from(argument).is_simple_impl(depth + 1)
                            })
                        })
                } else {
                    true
                }
            } else {
                false
            }
        } else {
            false
        };

        Ok(result)
    }

    fn is_simple_member_expression(&self, depth: u8) -> SyntaxResult<bool> {
        if let SimpleArgument::Expression(expression) = self {
            match expression {
                AnyMExpression::MStaticMemberExpression(static_expression) => {
                    let MStaticMemberExpressionFields { member, object, .. } =
                        static_expression.as_fields();

                    Ok(member.is_ok() && SimpleArgument::from(object?).is_simple_impl(depth))
                }
                AnyMExpression::MComputedMemberExpression(computed_expression) => {
                    let MComputedMemberExpressionFields { member, object, .. } =
                        computed_expression.as_fields();

                    Ok(SimpleArgument::from(member?).is_simple_impl(depth)
                        && object.map_or(false, |object| {
                            SimpleArgument::from(object).is_simple_impl(depth)
                        }))
                }
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    fn is_simple_unary_expression(&self, depth: u8) -> SyntaxResult<bool> {
        if let SimpleArgument::Expression(AnyMExpression::MUnaryExpression(unary_expression)) = self
        {
            if matches!(
                unary_expression.operator()?,
                MUnaryOperator::LogicalNot
                    | MUnaryOperator::Minus
                    | MUnaryOperator::Plus
                    | MUnaryOperator::BitwiseNot
            ) {
                Ok(SimpleArgument::from(unary_expression.argument()?).is_simple_impl(depth))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    fn is_simple_update_expression(&self, depth: u8) -> SyntaxResult<bool> {
        // Both PreUpdate and PostUpdate expressions have Increment and Decrement
        // operators, but they are typed separately, so must be handled that way.
        // These arms should be equivalent.
        match self {
            SimpleArgument::Expression(AnyMExpression::MPreUpdateExpression(update_expression)) => {
                if matches!(
                    update_expression.operator()?,
                    MPreUpdateOperator::Decrement | MPreUpdateOperator::Increment
                ) {
                    Ok(SimpleArgument::from(update_expression.operand()?).is_simple_impl(depth))
                } else {
                    Ok(false)
                }
            }
            SimpleArgument::Expression(AnyMExpression::MPostUpdateExpression(
                update_expression,
            )) => {
                if matches!(
                    update_expression.operator()?,
                    MPostUpdateOperator::Decrement | MPostUpdateOperator::Increment
                ) {
                    Ok(SimpleArgument::from(update_expression.operand()?).is_simple_impl(depth))
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        }
    }

    fn is_simple_array_expression(&self, depth: u8) -> bool {
        if let SimpleArgument::Expression(AnyMExpression::MArrayExpression(array_expression)) = self
        {
            array_expression
                .elements()
                .iter()
                .filter_map(|element| element.ok())
                .all(|element| match element {
                    AnyMArrayElement::AnyMExpression(expression) => {
                        SimpleArgument::from(expression).is_simple_impl(depth + 1)
                    }
                    AnyMArrayElement::MArrayHole(_) => true,
                    _ => false,
                })
        } else {
            false
        }
    }

    const fn is_simple_literal(&self) -> bool {
        matches!(
            self,
            SimpleArgument::Expression(
                AnyMExpression::AnyMLiteralExpression(_)
                    | AnyMExpression::MThisExpression(_)
                    | AnyMExpression::MIdentifierExpression(_)
                    | AnyMExpression::MSuperExpression(_)
            ) | SimpleArgument::Assignment(AnyMAssignment::MIdentifierAssignment(_))
        )
    }

    fn is_simple_object_expression(&self, depth: u8) -> bool {
        if let SimpleArgument::Expression(AnyMExpression::MObjectExpression(object_expression)) =
            self
        {
            object_expression
                .members()
                .iter()
                .filter_map(|member| member.ok())
                .all(|member| {
                    use AnyMObjectMember::*;

                    match member {
                        MShorthandPropertyObjectMember(_) => true,
                        MPropertyObjectMember(property) => {
                            let is_computed = matches!(
                                property.name(),
                                Ok(AnyMObjectMemberName::MComputedMemberName(_))
                            );

                            let is_simple = property.value().map_or(false, |value| {
                                SimpleArgument::from(value).is_simple_impl(depth + 1)
                            });

                            !is_computed && is_simple
                        }
                        _ => false,
                    }
                })
        } else {
            false
        }
    }
}

impl From<AnyMExpression> for SimpleArgument {
    fn from(expr: AnyMExpression) -> Self {
        Self::Expression(expr)
    }
}

impl From<AnyMAssignment> for SimpleArgument {
    fn from(assignment: AnyMAssignment) -> Self {
        Self::Assignment(assignment)
    }
}

impl From<MName> for SimpleArgument {
    fn from(name: MName) -> Self {
        Self::Name(name)
    }
}

impl From<MSpread> for SimpleArgument {
    fn from(_: MSpread) -> Self {
        Self::Spread
    }
}

impl From<AnyMCallArgument> for SimpleArgument {
    fn from(call_argument: AnyMCallArgument) -> Self {
        match call_argument {
            AnyMCallArgument::AnyMExpression(expr) => SimpleArgument::from(expr),
            AnyMCallArgument::MSpread(spread) => SimpleArgument::from(spread),
        }
    }
}
